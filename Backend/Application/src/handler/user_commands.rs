use std::{marker::PhantomData, time::Instant};

use models::{
    enums::{LifecycleState, WebsocketCommand},
    structs::Environments,
};
use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;

use crate::{
    handler::{
        Actions, Assets, Cancellations, Indicators, Metrics, OrderStatus, Pairs, Runtimes, Senders,
        Strategies, SubscribedIndicators, Tasks, WebsocketStreams,
    },
    utils::{Response, Types},
};

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
}

const STARTUP_SEQUENCE: &[StartupStep] = &[
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
        environment: Environments,
        runtime_token: &CancellationToken,
    ) -> Result<(), Response> {
        match self {
            StartupStep::Runtimes => Ok(()),

            StartupStep::Assets => {
                Assets::default()
                    .with_env(environment)
                    .start_assets(runtime_token)
                    .await
            }

            StartupStep::Pairs => Pairs::default().with_env(environment).start_pairs().await,

            StartupStep::Tasks => {
                Tasks::default()
                    .with_env(environment)
                    .start_tasks(runtime_token)
                    .await
            }

            StartupStep::OrderStatus => {
                OrderStatus::default()
                    .with_env(environment)
                    .start_status()
                    .await
            }

            StartupStep::Strategies => {
                Strategies::default()
                    .with_env(environment)
                    .start_strategies(runtime_token)
                    .await
            }

            StartupStep::Indicators => {
                Indicators::default()
                    .with_env(environment)
                    .start_indicators()
                    .await
            }

            StartupStep::SubscribedIndicators => {
                SubscribedIndicators::new(environment)
                    .start_subscribed_indicators()
                    .await
            }

            StartupStep::Actions => {
                Actions::default()
                    .with_env(environment)
                    .start_actions()
                    .await
            }

            StartupStep::Websocket => UserCommands::start_socket_loop(environment, runtime_token)
                .await
                .map_err(Response::server_error),

            StartupStep::Metrics => Ok(()),
        }
    }

    async fn stop(self, environment: Environments) -> Result<(), Response> {
        match self {
            StartupStep::Runtimes => Ok(()),
            StartupStep::Assets => Assets::default().with_env(environment).stop_assets().await,

            StartupStep::Pairs => Pairs::default().with_env(environment).stop_pairs().await,

            StartupStep::Tasks => Tasks::default().with_env(environment).stop_tasks().await,

            StartupStep::OrderStatus => {
                OrderStatus::default()
                    .with_env(environment)
                    .stop_status()
                    .await
            }

            StartupStep::Strategies => {
                Strategies::default()
                    .with_env(environment)
                    .stop_strategies()
                    .await
            }

            StartupStep::Indicators => {
                Indicators::default()
                    .with_env(environment)
                    .stop_indicators()
                    .await
            }

            StartupStep::SubscribedIndicators => {
                SubscribedIndicators::new(environment)
                    .stop_subscribed_indicators()
                    .await
            }

            StartupStep::Actions => {
                Actions::default()
                    .with_env(environment)
                    .stop_actions()
                    .await
            }

            StartupStep::Websocket => {
                let _ = UserCommands::stop_socket_loop().await;
                Ok(())
            }

            StartupStep::Metrics => {
                Metrics::default()
                    .with_env(environment)
                    .stop_metrics()
                    .await;
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserCommands<Phase = Types> {
    phase: PhantomData<Phase>,
}

impl UserCommands {
    pub async fn start_everything(environment: Environments) -> Result<(), StartupError> {
        let start = Instant::now();

        Runtimes::new(environment, LifecycleState::Starting)
            .set_runtime_status(environment)
            .await
            .map_err(|e| StartupError {
                step: StartupStep::Runtimes, // pre-start
                response: e,
                reset_result: None,
            })?;

        let runtime_token = match Cancellations::new(environment).get_runtime_token().await {
            Some(val) => val,
            None => Cancellations::new(environment)
                .start_runtime()
                .await
                .map_err(|e| StartupError {
                    step: StartupStep::Assets,
                    response: e,
                    reset_result: None,
                })?,
        };

        for step in STARTUP_SEQUENCE {
            if let Err(err) = step.execute(environment, &runtime_token).await {
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

        Runtimes::new(environment, LifecycleState::Running)
            .set_runtime_status(environment)
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

        if let Err(e) = Runtimes::new(environment, LifecycleState::Stopping)
            .set_runtime_status(environment)
            .await
        {
            errors.push(StopError {
                step: StartupStep::Runtimes,
                response: Some(e),
            });
        }

        if let Some(runtime_token) = Cancellations::new(environment).get_runtime_token().await {
            runtime_token.cancel();
        };

        for step in STOP_SEQUENCE {
            if let Err(err) = step.stop(environment).await {
                errors.push(StopError {
                    step: *step,
                    response: Some(err),
                });
            }
        }

        Metrics::default()
            .with_env(environment)
            .stop_metrics()
            .await;

        if let Err(e) = Runtimes::new(environment, LifecycleState::Off)
            .set_runtime_status(environment)
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

    pub async fn restart_everything(environment: Environments) -> Result<(), StartupError> {
        if let Err(stop_errors) = Self::stop_everything(environment).await {
            dbg!(stop_errors);
        }

        Self::start_everything(environment).await
    }

    pub async fn start_socket_loop(
        environment: Environments,
        token: &CancellationToken,
    ) -> Result<(), String> {
        let active_senders = Senders::get_senders().await;

        WebsocketStreams::new(environment)
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
        if let Err(err) = Runtimes::default().reset_runtime(environment).await {
            errors.push(format!("Runtime: {}", err.message));
        }

        // Reset assets
        if let Err(err) = Assets::default().with_env(environment).reset_assets().await {
            errors.push(format!("Assets: {}", err.message));
        }

        // Reset pairs
        if let Err(err) = Pairs::default().with_env(environment).reset_pairs().await {
            errors.push(format!("Pairs: {}", err.message));
        }

        // Reset tasks
        if let Err(err) = Tasks::default().with_env(environment).reset_tasks().await {
            errors.push(format!("Tasks: {}", err.message));
        }

        // Reset order status
        if let Err(err) = OrderStatus::default()
            .with_env(environment)
            .reset_status()
            .await
        {
            errors.push(format!("OrderStatus: {}", err.message));
        }

        // Reset strategies
        if let Err(err) = Strategies::default()
            .with_env(environment)
            .reset_strategies()
            .await
        {
            errors.push(format!("Strategies: {}", err.message));
        }

        // Reset indicators
        if let Err(err) = Indicators::default()
            .with_env(environment)
            .reset_indicators()
            .await
        {
            errors.push(format!("Indicators: {}", err.message));
        }

        // Reset subscribed indicators
        if let Err(err) = SubscribedIndicators::new(environment)
            .reset_subscribed_indicators()
            .await
        {
            errors.push(format!("SubscribedIndicators: {}", err.message));
        }

        // Reset actions
        if let Err(err) = Actions::default()
            .with_env(environment)
            .reset_actions()
            .await
        {
            errors.push(format!("Actions: {}", err.message));
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
