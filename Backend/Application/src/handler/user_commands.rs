use crate::{
    handler::{
        Actions, Assets, Indicators, Integrations, IntegrationsSettings, Metrics, OrderStatus,
        Pairs, Runtimes, Senders, Strategies, SubscribedIndicators, Tasks, WebsocketStreams,
    },
    utils::{EntityCache, RepoFactory, Response},
};
use models::{enums::LifecycleState, structs::Environments};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StartupStep {
    Runtimes,
    Assets,
    Pairs,
    Tasks,
    OrderStatus,
    Strategies,
    Indicators,
    SubscribedIndicators,
    Actions,
    Websocket,
    Metrics,
    Integrations,
    IntegrationsSettings,
}

const STARTUP_SEQUENCE: &[StartupStep] = &[
    StartupStep::Integrations,
    StartupStep::IntegrationsSettings,
    StartupStep::Assets,
    StartupStep::Pairs,
    StartupStep::Tasks,
    StartupStep::OrderStatus,
    StartupStep::Strategies,
    StartupStep::Indicators,
    StartupStep::SubscribedIndicators,
    StartupStep::Actions,
    StartupStep::Websocket,
    StartupStep::Metrics,
];

const STOP_SEQUENCE: &[StartupStep] = &[
    StartupStep::SubscribedIndicators,
    StartupStep::Websocket,
    StartupStep::IntegrationsSettings,
    StartupStep::Integrations,
    StartupStep::Actions,
    StartupStep::Indicators,
    StartupStep::Strategies,
    StartupStep::OrderStatus,
    StartupStep::Tasks,
    StartupStep::Pairs,
    StartupStep::Assets,
    StartupStep::Metrics,
];

#[derive(Debug, Serialize, Deserialize)]
pub struct StartupError {
    pub step: StartupStep,
    pub response: Response,
    pub reset_result: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopError {
    pub step: StartupStep,
    pub response: Option<Response>,
}

impl StartupStep {
    async fn execute(
        self,
        factory: RepoFactory,
        environment: Environments,
    ) -> Result<(), Response> {
        match self {
            StartupStep::Runtimes => Ok(()),

            StartupStep::Assets => {
                let senders = Senders::get_senders().await;

                let repo = factory.repo();

                Assets::new(repo).start(factory, environment, senders).await
            }

            StartupStep::Pairs => {
                let repo = factory.repo();

                Pairs::new(repo).start_pairs(environment).await
            }

            StartupStep::Tasks => {
                let repo = factory.repo();

                Tasks::new(repo).start(factory, environment).await
            }

            StartupStep::OrderStatus => {
                let repo = factory.repo();

                OrderStatus::new(repo).start(factory, environment).await
            }

            StartupStep::Strategies => {
                let senders = Senders::get_senders().await;

                let repo = factory.repo();

                Strategies::new(repo)
                    .start(factory, environment, senders)
                    .await
            }

            StartupStep::Indicators => {
                let repo = factory.repo();

                Indicators::new(repo).start_indicators(environment).await
            }

            StartupStep::SubscribedIndicators => {
                SubscribedIndicators::blank().start(environment).await
            }

            StartupStep::Actions => {
                let repo = factory.repo();

                Actions::new(repo).start(factory, environment).await
            }

            StartupStep::Websocket => {
                let senders = Senders::get_senders().await;
                WebsocketStreams::new()
                    .start(senders)
                    .await
                    .map_err(Response::server_error)
            }

            StartupStep::Metrics => Metrics::blank().start(environment).await,

            StartupStep::Integrations => {
                let repo = factory.repo();

                Integrations::new(repo)
                    .start_integrations(environment)
                    .await
            }

            StartupStep::IntegrationsSettings => {
                let repo = factory.repo();

                IntegrationsSettings::new(repo).start(environment).await
            }
        }
    }

    async fn stop(self, environment: Environments) -> Result<(), Response> {
        match self {
            StartupStep::Runtimes => Ok(()),
            StartupStep::Assets => Assets::blank().stop(environment).await,

            StartupStep::Pairs => Pairs::blank().stop_pairs(environment).await,

            StartupStep::Tasks => Tasks::blank().stop(environment).await,

            StartupStep::OrderStatus => OrderStatus::blank().stop(environment).await,

            StartupStep::Strategies => Strategies::blank().stop(environment).await,

            StartupStep::Indicators => Indicators::blank().stop_indicators(environment).await,

            StartupStep::SubscribedIndicators => {
                let senders = Senders::get_senders().await;
                SubscribedIndicators::blank()
                    .stop(environment, senders)
                    .await
            }

            StartupStep::Actions => Actions::blank().stop(environment).await,

            StartupStep::Websocket => {
                let senders = Senders::get_senders().await;

                WebsocketStreams::stop(senders)
                    .await
                    .map_err(Response::server_error)
            }

            StartupStep::Metrics => Metrics::blank().stop(environment).await,

            StartupStep::Integrations => Integrations::blank().stop(environment).await,

            StartupStep::IntegrationsSettings => {
                IntegrationsSettings::blank().stop(environment).await
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserCommands;

impl UserCommands {
    pub async fn start_everything(
        is_test: bool,
        environment: Environments,
    ) -> Result<(), StartupError> {
        let start = Instant::now();

        Runtimes::new()
            .set_runtime_status(environment, LifecycleState::Starting)
            .await
            .map_err(|e| StartupError {
                step: StartupStep::Runtimes, // pre-start
                response: e,
                reset_result: None,
            })?;

        let factory = match is_test {
            true => RepoFactory::mock(),
            false => {
                RepoFactory::db(environment)
                    .await
                    .map_err(|e| StartupError {
                        step: StartupStep::Runtimes, // pre-start
                        response: e,
                        reset_result: None,
                    })?
            }
        };

        for step in STARTUP_SEQUENCE {
            if let Err(err) = step.execute(factory.clone(), environment).await {
                let reset_result = UserCommands::reset_everything(environment).await.err();

                return Err(StartupError {
                    step: *step,
                    response: err,
                    reset_result,
                });
            }
        }

        Runtimes::new()
            .set_runtime_status(environment, LifecycleState::Running)
            .await
            .map_err(|e| StartupError {
                step: StartupStep::Assets,
                response: e,
                reset_result: None,
            })?;

        dbg!(format!("Everything started in {:?}", start.elapsed()));

        Ok(())
    }

    pub async fn stop_everything(environment: Environments) -> Result<(), Vec<StopError>> {
        let start = Instant::now();
        let mut errors = Vec::new();

        if let Err(e) = Runtimes::new()
            .set_runtime_status(environment, LifecycleState::Stopping)
            .await
        {
            errors.push(StopError {
                step: StartupStep::Runtimes,
                response: Some(e),
            });
        }

        for step in STOP_SEQUENCE {
            if let Err(err) = step.stop(environment).await {
                errors.push(StopError {
                    step: *step,
                    response: Some(err),
                });
            }
        }

        if let Err(e) = Runtimes::new()
            .set_runtime_status(environment, LifecycleState::Off)
            .await
        {
            errors.push(StopError {
                step: StartupStep::Runtimes,
                response: Some(e),
            });
        }

        dbg!(format!("Everything stopped in {:?}", start.elapsed()));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub async fn restart_everything(
        is_test: bool,
        environment: Environments,
    ) -> Result<(), StartupError> {
        if let Err(stop_errors) = Self::stop_everything(environment).await {
            dbg!(stop_errors);
        }

        Self::start_everything(is_test, environment).await
    }

    pub async fn reset_everything(environment: Environments) -> Result<(), String> {
        let start = std::time::Instant::now();
        let now = chrono::Local::now().naive_local();
        let mut errors = Vec::new();

        // Stop WebSocket
        WebsocketStreams::abort_handle().await;

        // Reset runtime
        if let Err(err) = Runtimes::new().reset_runtime(environment).await {
            errors.push(format!("Runtime: {}", err.message));
        }

        // Reset assets
        if let Err(err) = Assets::blank().reset(environment).await {
            errors.push(format!("Assets: {}", err));
        }

        // Reset pairs
        if let Err(err) = Pairs::blank().reset(environment).await {
            errors.push(format!("Pairs: {}", err));
        }

        // Reset tasks
        if let Err(err) = Tasks::blank().reset(environment).await {
            errors.push(format!("Tasks: {}", err));
        }

        // Reset order status
        if let Err(err) = OrderStatus::blank().reset(environment).await {
            errors.push(format!("OrderStatus: {}", err));
        }

        // Reset strategies
        if let Err(err) = Strategies::blank().reset(environment).await {
            errors.push(format!("Strategies: {}", err));
        }

        // Reset indicators
        if let Err(err) = Indicators::blank().reset_indicators(environment).await {
            errors.push(format!("Indicators: {}", err.message));
        }

        // Reset subscribed indicators
        if let Err(err) = SubscribedIndicators::blank().reset().await {
            errors.push(format!("SubscribedIndicators: {}", err));
        }

        // Reset actions
        if let Err(err) = Actions::blank().reset(environment).await {
            errors.push(format!("Actions: {}", err));
        }

        let duration = start.elapsed();

        // Check if there were any errors
        if !errors.is_empty() {
            let error_message = format!(
                "Reset completed with {} error(s) at {} (duration: {:?}):\n- {}",
                errors.len(),
                now,
                duration,
                errors.join("\n- ")
            );
            return Err(error_message);
        }

        Ok(())
    }
}

#[tokio::test]
async fn start_everything_successfully() {
    use crate::handler::HealthCheck;

    // Arrange
    let dev = UserCommands::start_everything(true, Environments::DEV).await;

    assert!(dev.is_ok(), "{dev:?}");

    let prod = UserCommands::start_everything(true, Environments::PROD).await;

    assert!(prod.is_ok(), "{prod:?}");

    // Health
    let health_check = HealthCheck::select_health_check(true)
        .await
        .expect("health check must succeed");

    // Helper closures
    let assert_running = |key: &str| {
        let res = health_check.get(key);
        assert_eq!(
            res,
            Some(&"running".to_string()),
            "{key} should be running: {res:?}"
        );
    };

    let assert_true = |key: &str| {
        let res = health_check.get(key);
        assert_eq!(
            res,
            Some(&"true".to_string()),
            "{key} should be true: {res:?}"
        );
    };

    // ===========================
    // DEV should be running
    // ===========================

    assert_running("cache_actions_dev_status");
    assert_running("cache_assets_dev_status");
    assert_running("cache_indicators_dev_status");
    assert_running("cache_integrations_dev_status");
    assert_running("cache_integrations_settings_dev_status");
    assert_running("cache_metrics_dev_status");
    assert_running("cache_order_status_dev_status");
    assert_running("cache_pairs_dev_status");
    assert_running("cache_strategies_dev_status");
    assert_running("cache_tasks_dev_status");

    // ===========================
    // PROD should be running
    // ===========================

    assert_running("cache_actions_prod_status");
    assert_running("cache_assets_prod_status");
    assert_running("cache_indicators_prod_status");
    assert_running("cache_integrations_prod_status");
    assert_running("cache_integrations_settings_prod_status");
    assert_running("cache_metrics_prod_status");
    assert_running("cache_order_status_prod_status");
    assert_running("cache_pairs_prod_status");
    assert_running("cache_strategies_prod_status");
    assert_running("cache_tasks_prod_status");

    // ===========================
    // Global checks
    // ===========================

    assert_true("cache_configuration_is_some");
    assert_running("cache_subscribed_indicators_status");
    assert_true("db_dev_conn_is_valid");
    assert_true("db_prod_conn_is_valid");

    use models::enums::SocketState;
    use tokio::time::{sleep, Duration};

    sleep(Duration::from_secs(5)).await;

    let cache_websocket_status = WebsocketStreams::state().await;

    assert_eq!(
        cache_websocket_status.state,
        SocketState::Connected,
        "Error: {:?} - Attempts: {:?}",
        cache_websocket_status.last_error,
        cache_websocket_status.reconnect_attempts
    );
}
