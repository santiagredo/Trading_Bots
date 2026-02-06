use crate::{
    handler::{
        Actions, Assets, Cancellations, Indicators, Integrations, IntegrationsSettings, Metrics,
        OrderStatus, Pairs, Runtimes, Senders, Strategies, SubscribedIndicators, Tasks,
        WebsocketStreams,
    },
    utils::{EntityCache, RepoFactory, Response},
};
use models::{
    enums::{LifecycleState, WebsocketCommand},
    structs::Environments,
};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tokio_util::sync::CancellationToken;

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
];

const STOP_SEQUENCE: &[StartupStep] = &[
    StartupStep::Websocket,
    StartupStep::IntegrationsSettings,
    StartupStep::Integrations,
    StartupStep::Actions,
    StartupStep::SubscribedIndicators,
    StartupStep::Indicators,
    StartupStep::Strategies,
    StartupStep::OrderStatus,
    StartupStep::Tasks,
    StartupStep::Pairs,
    StartupStep::Assets,
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
        runtime_token: &CancellationToken,
    ) -> Result<(), Response> {
        match self {
            StartupStep::Runtimes => Ok(()),

            StartupStep::Assets => {
                let senders = Senders::get_senders().await;

                let repo = factory.repo();

                Assets::new(repo)
                    .start(factory, environment, runtime_token, senders)
                    .await
            }

            StartupStep::Pairs => {
                let repo = factory.repo();

                Pairs::new(repo).start_pairs(environment).await
            }

            StartupStep::Tasks => {
                Tasks::new()
                    .start_tasks(factory, environment, runtime_token)
                    .await
            }

            StartupStep::OrderStatus => OrderStatus::new().start_status(environment).await,

            StartupStep::Strategies => {
                let senders = Senders::get_senders().await;

                let repo = factory.repo();

                let factory = RepoFactory::db(environment).await?;

                Strategies::new(repo)
                    .start_strategies(factory, environment, runtime_token, senders)
                    .await
            }

            StartupStep::Indicators => {
                let repo = factory.repo();

                Indicators::new(repo).start_indicators(environment).await
            }

            StartupStep::SubscribedIndicators => {
                SubscribedIndicators::blank()
                    .start_subscribed_indicators(environment)
                    .await
            }

            StartupStep::Actions => {
                let repo = factory.repo();

                Actions::new(repo).start(factory, environment).await
            }

            StartupStep::Websocket => UserCommands::start_socket_loop(runtime_token)
                .await
                .map_err(Response::server_error),

            StartupStep::Metrics => Ok(()),

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

            StartupStep::Tasks => Tasks::new().stop_tasks(environment).await,

            StartupStep::OrderStatus => OrderStatus::new().stop_status(environment).await,

            StartupStep::Strategies => Strategies::blank().stop_strategies(environment).await,

            StartupStep::Indicators => Indicators::blank().stop_indicators(environment).await,

            StartupStep::SubscribedIndicators => {
                SubscribedIndicators::blank()
                    .stop_subscribed_indicators(environment)
                    .await
            }

            StartupStep::Actions => Actions::blank().stop(environment).await,

            StartupStep::Websocket => {
                let _ = UserCommands::stop_socket_loop().await;
                Ok(())
            }

            StartupStep::Metrics => Metrics::blank().stop_metrics(environment).await,

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

        let runtime_token = match Cancellations::new().get_runtime_token(environment).await {
            Some(val) => val,
            None => Cancellations::new()
                .start_runtime(environment)
                .await
                .map_err(|e| StartupError {
                    step: StartupStep::Assets,
                    response: e,
                    reset_result: None,
                })?,
        };

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
            if let Err(err) = step
                .execute(factory.clone(), environment, &runtime_token)
                .await
            {
                let reset_result =
                    UserCommands::reset_everything(environment, runtime_token.clone())
                        .await
                        .err();

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

        Cancellations::new().stop_runtime_core(environment).await;

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

    pub async fn start_socket_loop(token: &CancellationToken) -> Result<(), String> {
        let active_senders = Senders::get_senders().await;

        WebsocketStreams::new()
            .start_websocket(active_senders, token)
            .await
    }

    pub async fn stop_socket_loop() -> Result<(), String> {
        let active_senders = Senders::get_senders().await;
        let _ = active_senders
            .command_sender
            .send(WebsocketCommand::Shutdown)
            .map_err(|_| format!("Failed to send shutdown command"))?;

        WebsocketStreams::stop_websocket().await
    }

    pub async fn reset_everything(
        environment: Environments,
        runtime_token: CancellationToken,
    ) -> Result<(), String> {
        let start = std::time::Instant::now();
        let now = chrono::Local::now().naive_local();
        let mut errors = Vec::new();

        // Cancel runtime token
        runtime_token.cancel();

        // Stop WebSocket
        if let Err(err) = Self::stop_socket_loop().await {
            errors.push(format!("WebSocket: {}", err));
        }

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
        if let Err(err) = Tasks::new().reset_tasks(environment).await {
            errors.push(format!("Tasks: {}", err.message));
        }

        // Reset order status
        if let Err(err) = OrderStatus::new().reset_status(environment).await {
            errors.push(format!("OrderStatus: {}", err.message));
        }

        // Reset strategies
        if let Err(err) = Strategies::blank().reset_strategies(environment).await {
            errors.push(format!("Strategies: {}", err.message));
        }

        // Reset indicators
        if let Err(err) = Indicators::blank().reset_indicators(environment).await {
            errors.push(format!("Indicators: {}", err.message));
        }

        // Reset subscribed indicators
        if let Err(err) = SubscribedIndicators::blank()
            .reset_subscribed_indicators(environment)
            .await
        {
            errors.push(format!("SubscribedIndicators: {}", err.message));
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
