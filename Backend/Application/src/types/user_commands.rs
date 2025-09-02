use std::{marker::PhantomData, time::Instant};

use chrono::Local;
use models::enums::WebsocketCommand;

use crate::{
    types::{
        Actions, Assets, Indicators, Metrics, OrderStatus, Pairs, Senders, Strategies, Tasks,
        WebsocketStreams,
    },
    utils::{Response, Types},
};

#[derive(Debug, Clone)]
pub struct UserCommands<Phase = Types> {
    phase: PhantomData<Phase>,
}

impl UserCommands {
    pub async fn start_everything() -> Result<(), Response> {
        let start = Instant::now();

        Tasks::start_async_tasks().await;

        OrderStatus::start_active_status().await?;

        Assets::start_active_assets().await?;

        Metrics::start_active_metrics().await;

        Pairs::start_active_pairs().await?;

        Strategies::start_active_strategies().await?;

        Indicators::start_active_indicators().await?;

        Indicators::start_subscribed_indicators().await;

        Actions::start_active_actions().await?;

        Self::start_socket_loop().await;

        Strategies::start_strategies_evaluation_loop().await;

        let now = Local::now().naive_local();

        dbg!(format!(
            "Everything started at {} -- duration of {:?}",
            now,
            start.elapsed()
        ));

        Ok(())
    }

    pub async fn stop_everything() {
        let start = Instant::now();

        Tasks::stop_async_tasks().await;

        OrderStatus::stop_active_status().await;

        Assets::stop_active_assets().await;

        Metrics::stop_active_metrics().await;

        Pairs::stop_active_pairs().await;

        Strategies::stop_active_strategies().await;

        Indicators::stop_active_indicators().await;

        Indicators::stop_subscribed_indicators().await;

        Actions::stop_active_actions().await;

        let _ = Self::stop_socket_loop().await;

        Strategies::stop_strategies_evaluation_loop().await;

        let now = Local::now().naive_local();

        dbg!(format!(
            "Everything stopped at {} -- duration of {:?}",
            now,
            start.elapsed()
        ));
    }

    pub async fn refresh_everything() -> Result<(), Response> {
        Self::stop_everything().await;
        Self::start_everything().await?;

        // StrategiesOverview::run_active_strategies().await;

        Ok(())
    }

    pub async fn start_socket_loop() {
        let active_senders = Senders::get_active_senders().await;

        WebsocketStreams::new()
            .start_socket_loop(active_senders)
            .await;
    }

    // pub async fn reload_socket_loop() -> Result<usize, String> {
    //     let ws_senders = Senders::get_ws_senders().await;
    //     ws_senders
    //         .command_sender
    //         .send(WebsocketCommand::Reload)
    //         .map_err(|_| format!("Failed to send reload command"))
    // }

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
