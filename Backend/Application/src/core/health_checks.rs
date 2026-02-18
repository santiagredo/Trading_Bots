use crate::{
    handler::{
        Actions, Assets, Configurations, Engines, HealthCheck, Indicators, Integrations,
        IntegrationsSettings, Metrics, OrderStatus, Pairs, Runtimes, Strategies,
        SubscribedIndicators, Tasks, WebsocketStreams, DBC,
    },
    utils::{EntityCache, Response},
};
use models::structs::Environments;
use std::collections::BTreeMap;

impl HealthCheck {
    pub async fn select_health_check(is_test: bool) -> Result<BTreeMap<String, String>, Response> {
        let mut health_map = BTreeMap::new();

        // DB
        let dev_conn = match is_test {
            false => DBC::db(&Environments::DEV).await?.ping().await.is_ok(),
            true => true,
        };
        health_map.insert("db_dev_conn_is_valid".to_string(), dev_conn.to_string());

        let prod_conn = match is_test {
            false => DBC::db(&Environments::PROD).await?.ping().await.is_ok(),
            true => true,
        };
        health_map.insert("db_prod_conn_is_valid".to_string(), prod_conn.to_string());

        // Actions
        let cache_actions_dev_status = Actions::blank().state(Environments::DEV).await;
        health_map.insert(
            "cache_actions_dev_status".to_string(),
            cache_actions_dev_status.to_string(),
        );

        let cache_actions_prod_status = Actions::blank().state(Environments::PROD).await;
        health_map.insert(
            "cache_actions_prod_status".to_string(),
            cache_actions_prod_status.to_string(),
        );

        // Assets
        let cache_assets_dev_status = Assets::blank().state(Environments::DEV).await;
        health_map.insert(
            "cache_assets_dev_status".to_string(),
            cache_assets_dev_status.to_string(),
        );

        let cache_assets_prod_status = Assets::blank().state(Environments::PROD).await;
        health_map.insert(
            "cache_assets_prod_status".to_string(),
            cache_assets_prod_status.to_string(),
        );

        // Configuration
        let _ = Configurations::default().select_configuration().await;
        health_map.insert(
            "cache_configuration_is_some".to_string(),
            "true".to_string(),
        );

        // Indicators
        let cache_indicators_dev_status = Indicators::blank().state(Environments::DEV).await;
        health_map.insert(
            "cache_indicators_dev_status".to_string(),
            cache_indicators_dev_status.to_string(),
        );

        let cache_indicators_prod_status = Indicators::blank().state(Environments::PROD).await;
        health_map.insert(
            "cache_indicators_prod_status".to_string(),
            cache_indicators_prod_status.to_string(),
        );

        // Integrations
        let cache_integrations_dev_status = Integrations::blank().state(Environments::DEV).await;
        health_map.insert(
            "cache_integrations_dev_status".to_string(),
            cache_integrations_dev_status.to_string(),
        );

        let cache_integrations_prod_status = Integrations::blank().state(Environments::PROD).await;
        health_map.insert(
            "cache_integrations_prod_status".to_string(),
            cache_integrations_prod_status.to_string(),
        );

        // Integrations Settings
        let cache_integrations_settings_dev_status =
            IntegrationsSettings::blank().state(Environments::DEV).await;

        health_map.insert(
            "cache_integrations_settings_dev_status".to_string(),
            cache_integrations_settings_dev_status.to_string(),
        );

        let cache_integrations_settings_prod_status = IntegrationsSettings::blank()
            .state(Environments::PROD)
            .await;

        health_map.insert(
            "cache_integrations_settings_prod_status".to_string(),
            cache_integrations_settings_prod_status.to_string(),
        );

        // Metrics
        let cache_metrics_dev_status = Metrics::blank().state(Environments::DEV).await;
        health_map.insert(
            "cache_metrics_dev_status".to_string(),
            cache_metrics_dev_status.to_string(),
        );

        let cache_metrics_prod_status = Metrics::blank().state(Environments::PROD).await;
        health_map.insert(
            "cache_metrics_prod_status".to_string(),
            cache_metrics_prod_status.to_string(),
        );

        // Pairs
        let cache_pairs_dev_status = Pairs::blank().state(Environments::DEV).await;
        health_map.insert(
            "cache_pairs_dev_status".to_string(),
            cache_pairs_dev_status.to_string(),
        );

        let cache_pairs_prod_status = Pairs::blank().state(Environments::PROD).await;
        health_map.insert(
            "cache_pairs_prod_status".to_string(),
            cache_pairs_prod_status.to_string(),
        );

        // Order Status
        let cache_order_status_dev_status = OrderStatus::blank().state(Environments::DEV).await;
        health_map.insert(
            "cache_order_status_dev_status".to_string(),
            cache_order_status_dev_status.to_string(),
        );

        let cache_order_status_prod_status = OrderStatus::blank().state(Environments::PROD).await;
        health_map.insert(
            "cache_order_status_prod_status".to_string(),
            cache_order_status_prod_status.to_string(),
        );

        // Strategies
        let cache_strategies_dev_status = Strategies::blank().state(Environments::DEV).await;
        health_map.insert(
            "cache_strategies_dev_status".to_string(),
            cache_strategies_dev_status.to_string(),
        );

        let cache_strategies_prod_status = Strategies::blank().state(Environments::PROD).await;
        health_map.insert(
            "cache_strategies_prod_status".to_string(),
            cache_strategies_prod_status.to_string(),
        );

        // Subscribed Indicators
        let cache_subscribed_indicators_status = SubscribedIndicators::blank().state().await;
        health_map.insert(
            "cache_subscribed_indicators_status".to_string(),
            cache_subscribed_indicators_status.to_string(),
        );

        // Tasks
        let cache_tasks_dev_status = Tasks::blank().state(Environments::DEV).await;
        health_map.insert(
            "cache_tasks_dev_status".to_string(),
            cache_tasks_dev_status.to_string(),
        );

        let cache_tasks_prod_status = Tasks::blank().state(Environments::PROD).await;
        health_map.insert(
            "cache_tasks_prod_status".to_string(),
            cache_tasks_prod_status.to_string(),
        );

        // Websocket streams
        let cache_websocket_status = WebsocketStreams::state().await;
        health_map.insert(
            "cache_websocket_status".to_string(),
            cache_websocket_status.state.to_string(),
        );

        // Engine
        let engine = Engines::blank().get_engine_cache().await;
        health_map.insert(
            "engine_startup_date".to_string(),
            engine.startup_date.format("%Y-%m-%d %H:%M:%S").to_string(),
        );
        health_map.insert(
            "engine_last_update_date".to_string(),
            engine
                .last_update_date
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        );
        health_map.insert("engine_status".to_string(), engine.status.to_string());

        // Runtimes
        let runtimes = Runtimes::new().get_runtimes_status().await;
        health_map.insert(
            "runtime_dev_startup_date".to_string(),
            runtimes
                .dev
                .startup_date
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        );
        health_map.insert(
            "runtime_dev_last_update_date".to_string(),
            runtimes
                .dev
                .last_update_date
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        );
        health_map.insert(
            "runtime_dev_status".to_string(),
            runtimes.dev.status.to_string(),
        );

        health_map.insert(
            "runtime_prod_startup_date".to_string(),
            runtimes
                .prod
                .startup_date
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        );
        health_map.insert(
            "runtime_prod_last_update_date".to_string(),
            runtimes
                .prod
                .last_update_date
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        );
        health_map.insert(
            "runtime_prod_status".to_string(),
            runtimes.prod.status.to_string(),
        );

        Ok(health_map)
    }
}
