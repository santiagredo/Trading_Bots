use std::{marker::PhantomData, time::Instant};

use chrono::Local;
use models::{enums::WebsocketCommand, structs::Environments};

use crate::{
    handler::{
        Actions, Assets, Indicators, Metrics, OrderStatus, Pairs, Senders, Strategies,
        SubscribedIndicators, Tasks, WebsocketStreams,
    },
    utils::{Response, Types},
};

#[derive(Debug, Clone)]
pub struct UserCommands<Phase = Types> {
    phase: PhantomData<Phase>,
}

impl UserCommands {
    pub async fn start_everything(environment: Environments) -> Result<(), Response> {
        let start = Instant::now();

        Assets::default()
            .with_env(environment)
            .start_active_assets()
            .await?;

        Pairs::default()
            .with_env(environment)
            .start_active_pairs()
            .await?;

        Tasks::default()
            .with_env(environment)
            .start_active_tasks()
            .await?;

        OrderStatus::default()
            .with_env(&environment)
            .start_active_status()
            .await?;

        Strategies::default()
            .with_env(environment)
            .start_active_strategies()
            .await?;

        Indicators::default()
            .with_env(environment)
            .start_active_indicators()
            .await?;

        SubscribedIndicators::new(environment)
            .start_active_subscribed_indicators()
            .await;

        Actions::default()
            .with_env(environment)
            .start_active_actions()
            .await?;

        Self::start_socket_loop(environment).await;

        let now = Local::now().naive_local();

        dbg!(format!(
            "Everything started at {} -- duration of {:?}",
            now,
            start.elapsed()
        ));

        Ok(())
    }

    pub async fn stop_everything(environment: Environments) -> Result<(), Response> {
        let start = Instant::now();

        Assets::default()
            .with_env(environment)
            .stop_active_assets()
            .await?;

        Pairs::default()
            .with_env(environment)
            .stop_active_pairs()
            .await;

        Tasks::default()
            .with_env(environment)
            .stop_active_tasks()
            .await;

        OrderStatus::default()
            .with_env(&environment)
            .stop_active_status()
            .await;

        Strategies::default()
            .with_env(environment)
            .stop_active_strategies()
            .await;

        Indicators::default()
            .with_env(environment)
            .stop_active_indicators()
            .await;

        SubscribedIndicators::new(environment)
            .stop_active_subscribed_indicators()
            .await;

        Actions::default()
            .with_env(environment)
            .stop_active_actions()
            .await;

        let _ = Self::stop_socket_loop().await;

        Metrics::default()
            .with_env(environment)
            .stop_active_metrics()
            .await;

        let now = Local::now().naive_local();

        dbg!(format!(
            "Everything stopped at {} -- duration of {:?}",
            now,
            start.elapsed()
        ));

        Ok(())
    }

    pub async fn restart_everything(environment: Environments) -> Result<(), Response> {
        Self::stop_everything(environment).await?;
        Self::start_everything(environment).await?;

        Ok(())
    }

    pub async fn start_socket_loop(environment: Environments) {
        let active_senders = Senders::get_active_senders().await;

        WebsocketStreams::new(environment)
            .start_socket_loop(active_senders)
            .await;
    }

    pub async fn stop_socket_loop() -> Result<usize, String> {
        let active_senders = Senders::get_active_senders().await;
        let result = active_senders
            .command_sender
            .send(WebsocketCommand::Shutdown)
            .map_err(|_| format!("Failed to send shutdown command"));

        let handle = WebsocketStreams::get_binance_ws_abort_handle().await;

        if let Some(handle) = handle.filter(|h| !h.is_finished()) {
            handle.abort();
            println!("Aborted handle: socket loop");
        }

        result
    }
}
