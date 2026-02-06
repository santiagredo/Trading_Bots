use crate::{
    handler::{
        Actions, Assets, Configurations, HealthCheck, Indicators, Integrations,
        IntegrationsSettings, Metrics, OrderStatus, Pairs, Strategies, SubscribedIndicators, Tasks,
        WebsocketStreams, DBC,
    },
    utils::{EntityCache, Response},
};
use models::structs::Environments;
use std::collections::BTreeMap;

impl HealthCheck {
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
        let cache_order_status_dev_status = OrderStatus::get_cache_state(Environments::DEV).await;
        health_map.insert(
            "cache_order_status_dev_status".to_string(),
            cache_order_status_dev_status.to_string(),
        );

        let cache_order_status_prod_status = OrderStatus::get_cache_state(Environments::PROD).await;
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
        let cache_subscribed_indicators_dev_status =
            SubscribedIndicators::blank().state(Environments::DEV).await;
        health_map.insert(
            "cache_subscribed_indicators_dev_status".to_string(),
            cache_subscribed_indicators_dev_status.to_string(),
        );

        let cache_subscribed_indicators_prod_status = SubscribedIndicators::blank()
            .state(Environments::PROD)
            .await;
        health_map.insert(
            "cache_subscribed_indicators_prod_status".to_string(),
            cache_subscribed_indicators_prod_status.to_string(),
        );

        // Tasks
        let cache_tasks_dev_status = Tasks::new().get_tasks_state(Environments::DEV).await;
        health_map.insert(
            "cache_tasks_dev_status".to_string(),
            cache_tasks_dev_status.to_string(),
        );

        let cache_tasks_prod_status = Tasks::new().get_tasks_state(Environments::PROD).await;
        health_map.insert(
            "cache_tasks_prod_status".to_string(),
            cache_tasks_prod_status.to_string(),
        );

        // Websocket streams
        let cache_websocket_status = WebsocketStreams::new().get_status().await;
        health_map.insert(
            "cache_websocket_status".to_string(),
            cache_websocket_status.state.to_string(),
        );

        Ok(health_map)
    }
}
