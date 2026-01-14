use std::collections::BTreeMap;

use models::structs::Environments;

use crate::{
    handler::{
        Actions, Assets, Configurations, HealthCheck, Indicators, Integrations,
        IntegrationsSettings, Metrics, OrderStatus, Pairs, Strategies, SubscribedIndicators, Tasks,
        WebsocketStreams, DBC,
    },
    utils::{Core, Response},
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
        let cache_actions_dev_status = Actions::default()
            .with_env(Environments::DEV)
            .get_actions_state()
            .await;
        health_map.insert(
            "cache_actions_dev_status".to_string(),
            cache_actions_dev_status.to_string(),
        );

        let cache_actions_prod_status = Actions::default()
            .with_env(Environments::PROD)
            .get_actions_state()
            .await;
        health_map.insert(
            "cache_actions_prod_status".to_string(),
            cache_actions_prod_status.to_string(),
        );

        // Assets
        let cache_assets_dev_status = Assets::default()
            .with_env(Environments::DEV)
            .get_assets_state()
            .await;
        health_map.insert(
            "cache_assets_dev_status".to_string(),
            cache_assets_dev_status.to_string(),
        );

        let cache_assets_prod_status = Assets::default()
            .with_env(Environments::PROD)
            .get_assets_state()
            .await;
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
        let cache_indicators_dev_status = Indicators::default()
            .with_env(Environments::DEV)
            .get_indicators_state()
            .await;
        health_map.insert(
            "cache_indicators_dev_status".to_string(),
            cache_indicators_dev_status.to_string(),
        );

        let cache_indicators_prod_status = Indicators::default()
            .with_env(Environments::PROD)
            .get_indicators_state()
            .await;
        health_map.insert(
            "cache_indicators_prod_status".to_string(),
            cache_indicators_prod_status.to_string(),
        );

        // Integrations
        let cache_integrations_dev_status = Integrations::default()
            .with_env(Environments::DEV)
            .get_integrations_state()
            .await;
        health_map.insert(
            "cache_integrations_dev_status".to_string(),
            cache_integrations_dev_status.to_string(),
        );

        let cache_integrations_prod_status = Integrations::default()
            .with_env(Environments::PROD)
            .get_integrations_state()
            .await;
        health_map.insert(
            "cache_integrations_prod_status".to_string(),
            cache_integrations_prod_status.to_string(),
        );

        // Integrations Settings
        let cache_integrations_settings_dev_status = IntegrationsSettings::default()
            .with_env(Environments::DEV)
            .get_integrations_settings_state()
            .await;

        health_map.insert(
            "cache_integrations_settings_dev_status".to_string(),
            cache_integrations_settings_dev_status.to_string(),
        );

        let cache_integrations_settings_prod_status = IntegrationsSettings::default()
            .with_env(Environments::PROD)
            .get_integrations_settings_state()
            .await;

        health_map.insert(
            "cache_integrations_settings_prod_status".to_string(),
            cache_integrations_settings_prod_status.to_string(),
        );

        // Metrics
        let cache_metrics_dev_is_some = Metrics::default()
            .with_env(Environments::DEV)
            .get_metric()
            .await;
        health_map.insert(
            "cache_metrics_dev_is_some".to_string(),
            cache_metrics_dev_is_some.is_some().to_string(),
        );

        let cache_metrics_prod_is_some = Metrics::default()
            .with_env(Environments::PROD)
            .get_metric()
            .await;
        health_map.insert(
            "cache_metrics_prod_is_some".to_string(),
            cache_metrics_prod_is_some.is_some().to_string(),
        );

        // Pairs
        let cache_pairs_dev_status = Pairs::default()
            .with_env(Environments::DEV)
            .get_pairs_state()
            .await;
        health_map.insert(
            "cache_pairs_dev_status".to_string(),
            cache_pairs_dev_status.to_string(),
        );

        let cache_pairs_prod_status = Pairs::default()
            .with_env(Environments::PROD)
            .get_pairs_state()
            .await;
        health_map.insert(
            "cache_pairs_prod_status".to_string(),
            cache_pairs_prod_status.to_string(),
        );

        // Order Status
        let cache_order_status_dev_status = OrderStatus::default()
            .with_env(Environments::DEV)
            .get_status_state()
            .await;
        health_map.insert(
            "cache_order_status_dev_status".to_string(),
            cache_order_status_dev_status.to_string(),
        );

        let cache_order_status_prod_status = OrderStatus::default()
            .with_env(Environments::PROD)
            .get_status_state()
            .await;
        health_map.insert(
            "cache_order_status_prod_status".to_string(),
            cache_order_status_prod_status.to_string(),
        );

        // Strategies
        let cache_strategies_dev_status = Strategies::default()
            .with_env(Environments::DEV)
            .get_strategies_state()
            .await;
        health_map.insert(
            "cache_strategies_dev_status".to_string(),
            cache_strategies_dev_status.to_string(),
        );

        let cache_strategies_prod_status = Strategies::default()
            .with_env(Environments::PROD)
            .get_strategies_state()
            .await;
        health_map.insert(
            "cache_strategies_prod_status".to_string(),
            cache_strategies_prod_status.to_string(),
        );

        // Subscribed Indicators
        let cache_subscribed_indicators_dev_status = SubscribedIndicators::new(Environments::DEV)
            .get_subscribed_indicators_state()
            .await;
        health_map.insert(
            "cache_subscribed_indicators_dev_status".to_string(),
            cache_subscribed_indicators_dev_status.to_string(),
        );

        let cache_subscribed_indicators_prod_status = SubscribedIndicators::new(Environments::PROD)
            .get_subscribed_indicators_state()
            .await;
        health_map.insert(
            "cache_subscribed_indicators_prod_status".to_string(),
            cache_subscribed_indicators_prod_status.to_string(),
        );

        // Tasks
        let cache_tasks_dev_status = Tasks::default()
            .with_env(Environments::DEV)
            .get_tasks_state()
            .await;
        health_map.insert(
            "cache_tasks_dev_status".to_string(),
            cache_tasks_dev_status.to_string(),
        );

        let cache_tasks_prod_status = Tasks::default()
            .with_env(Environments::PROD)
            .get_tasks_state()
            .await;
        health_map.insert(
            "cache_tasks_prod_status".to_string(),
            cache_tasks_prod_status.to_string(),
        );

        // Websocket streams
        let cache_websocket_status = WebsocketStreams::default().get_status().await;
        health_map.insert(
            "cache_websocket_status".to_string(),
            cache_websocket_status.state.to_string(),
        );

        Ok(health_map)
    }
}
