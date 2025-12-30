use std::collections::BTreeMap;

use chrono::Local;
use models::structs::Environments;

use crate::{
    handler::{
        Actions, Assets, Configurations, HealthCheck, Indicators, Metrics, OrderStatus, Pairs,
        Strategies, SubscribedIndicators, Tasks, WebsocketStreams, DBC,
    },
    utils::{Cache, Core, Response},
};

impl HealthCheck<Core> {
    pub async fn select_health_check_core() -> Result<BTreeMap<String, String>, Response> {
        let mut health_map = BTreeMap::new();

        // DB
        let dev_conn = DBC::db(&Environments::DEV).await?;
        health_map.insert(
            "db_dev_conn_is_valid".to_string(),
            dev_conn.ping().await.is_ok().to_string(),
        );

        let prod_conn = DBC::db(&Environments::PROD).await?;
        health_map.insert(
            "db_prod_conn_is_valid".to_string(),
            prod_conn.ping().await.is_ok().to_string(),
        );

        // Actions
        let cache_actions_dev_is_initialized =
            Actions::<Cache>::get_active_actions_status_cache(&Environments::DEV).await;
        health_map.insert(
            "cache_actions_dev_is_initialized".to_string(),
            cache_actions_dev_is_initialized.to_string(),
        );

        let cache_actions_prod_is_initialized =
            Actions::<Cache>::get_active_actions_status_cache(&Environments::PROD).await;
        health_map.insert(
            "cache_actions_prod_is_initialized".to_string(),
            cache_actions_prod_is_initialized.to_string(),
        );

        // Assets
        let cache_assets_dev_is_initialized =
            Assets::<Cache>::get_active_assets_status_cache(&Environments::DEV).await;
        health_map.insert(
            "cache_assets_dev_is_initialized".to_string(),
            cache_assets_dev_is_initialized.to_string(),
        );

        let cache_assets_prod_is_initialized =
            Assets::<Cache>::get_active_assets_status_cache(&Environments::PROD).await;
        health_map.insert(
            "cache_assets_prod_is_initialized".to_string(),
            cache_assets_prod_is_initialized.to_string(),
        );

        // Configuration
        let cache_configuration_is_some = Configurations::<Cache>::get_configuration_cache().await;
        health_map.insert(
            "cache_configuration_is_some".to_string(),
            cache_configuration_is_some.is_some().to_string(),
        );

        // Indicators
        let cache_indicators_dev_is_initialized =
            Indicators::<Cache>::get_active_indicators_status_cache(&Environments::DEV).await;
        health_map.insert(
            "cache_indicators_dev_is_initialized".to_string(),
            cache_indicators_dev_is_initialized.to_string(),
        );

        let cache_indicators_prod_is_initialized =
            Indicators::<Cache>::get_active_indicators_status_cache(&Environments::PROD).await;
        health_map.insert(
            "cache_indicators_prod_is_initialized".to_string(),
            cache_indicators_prod_is_initialized.to_string(),
        );

        // Metrics
        let cache_metrics_dev_is_some = Metrics::default()
            .with_env(Environments::DEV)
            .get_active_metric()
            .await;
        health_map.insert(
            "cache_metrics_dev_is_some".to_string(),
            cache_metrics_dev_is_some.is_some().to_string(),
        );

        let cache_metrics_prod_is_some = Metrics::default()
            .with_env(Environments::PROD)
            .get_active_metric()
            .await;
        health_map.insert(
            "cache_metrics_prod_is_some".to_string(),
            cache_metrics_prod_is_some.is_some().to_string(),
        );

        // Pairs
        let cache_pairs_dev_is_initialized =
            Pairs::<Cache>::get_active_pairs_status_cache(&Environments::DEV).await;
        health_map.insert(
            "cache_pairs_dev_is_initialized".to_string(),
            cache_pairs_dev_is_initialized.to_string(),
        );

        let cache_pairs_prod_is_initialized =
            Pairs::<Cache>::get_active_pairs_status_cache(&Environments::PROD).await;
        health_map.insert(
            "cache_pairs_prod_is_initialized".to_string(),
            cache_pairs_prod_is_initialized.to_string(),
        );

        // Status
        let cache_status_dev_is_initialized =
            OrderStatus::<Cache>::get_active_status_status_cache(&Environments::DEV).await;
        health_map.insert(
            "cache_status_dev_is_initialized".to_string(),
            cache_status_dev_is_initialized.to_string(),
        );

        let cache_status_prod_is_initialized =
            OrderStatus::<Cache>::get_active_status_status_cache(&Environments::PROD).await;
        health_map.insert(
            "cache_status_prod_is_initialized".to_string(),
            cache_status_prod_is_initialized.to_string(),
        );

        // Strategies
        let cache_strategies_dev_is_initialized =
            Strategies::<Cache>::get_active_strategies_status_cache(&Environments::DEV).await;
        health_map.insert(
            "cache_strategies_dev_is_initialized".to_string(),
            cache_strategies_dev_is_initialized.to_string(),
        );

        let cache_strategies_prod_is_initialized =
            Strategies::<Cache>::get_active_strategies_status_cache(&Environments::PROD).await;
        health_map.insert(
            "cache_strategies_prod_is_initialized".to_string(),
            cache_strategies_prod_is_initialized.to_string(),
        );

        // Subscribed Indicators
        let cache_subscribed_indicators_dev_is_initialized =
            SubscribedIndicators::<Cache>::get_active_subscribed_indicators_status_cache(
                &Environments::DEV,
            )
            .await;
        health_map.insert(
            "cache_subscribed_indicators_dev_is_initialized".to_string(),
            cache_subscribed_indicators_dev_is_initialized.to_string(),
        );

        let cache_subscribed_indicators_prod_is_initialized =
            SubscribedIndicators::<Cache>::get_active_subscribed_indicators_status_cache(
                &Environments::PROD,
            )
            .await;
        health_map.insert(
            "cache_subscribed_indicators_prod_is_initialized".to_string(),
            cache_subscribed_indicators_prod_is_initialized.to_string(),
        );

        // Tasks
        let cache_tasks_dev_is_initialized =
            Tasks::<Cache>::get_active_tasks_status_cache(&Environments::DEV).await;
        health_map.insert(
            "cache_tasks_dev_is_initialized".to_string(),
            cache_tasks_dev_is_initialized.to_string(),
        );

        let cache_tasks_prod_is_initialized =
            Tasks::<Cache>::get_active_tasks_status_cache(&Environments::PROD).await;
        health_map.insert(
            "cache_tasks_prod_is_initialized".to_string(),
            cache_tasks_prod_is_initialized.to_string(),
        );

        // Websocket streams
        let (cache_ws_binance_abort_handle_is_some, cache_ws_binance_abort_handle_is_finished) =
            match WebsocketStreams::get_binance_ws_abort_handle().await {
                None => (false, false),
                Some(val) => (true, val.is_finished()),
            };
        health_map.insert(
            "cache_ws_binance_abort_handle_is_some".to_string(),
            cache_ws_binance_abort_handle_is_some.to_string(),
        );
        health_map.insert(
            "cache_ws_binance_abort_handle_is_finished".to_string(),
            cache_ws_binance_abort_handle_is_finished.to_string(),
        );

        // Startup time and date
        let cache_health_check = HealthCheck::get_health_check_cache().await;
        let now = Local::now().naive_local();
        let elapsed = now - cache_health_check.startup_date;

        health_map.insert(
            "uptime".to_string(),
            format!(
                "{}d {}h {}m {}s",
                elapsed.num_days(),
                elapsed.num_hours() % 24,
                elapsed.num_minutes() % 60,
                elapsed.num_seconds() % 60,
            ),
        );

        health_map.insert(
            "startup_date".to_string(),
            cache_health_check.startup_date.to_string(),
        );

        Ok(health_map)
    }
}
