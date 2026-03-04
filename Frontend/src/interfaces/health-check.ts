export interface CacheHealthCheck {
    // Cache
    cache_actions_dev_status: string;
    cache_actions_prod_status: string;

    cache_assets_dev_status: string;
    cache_assets_prod_status: string;

    cache_configuration_is_some: string;

    cache_indicators_dev_status: string;
    cache_indicators_prod_status: string;

    cache_integrations_dev_status: string;
    cache_integrations_prod_status: string;

    cache_integrations_settings_dev_status: string;
    cache_integrations_settings_prod_status: string;

    cache_metrics_dev_status: string;
    cache_metrics_prod_status: string;

    cache_order_status_dev_status: string;
    cache_order_status_prod_status: string;

    cache_pairs_dev_status: string;
    cache_pairs_prod_status: string;

    cache_strategies_dev_status: string;
    cache_strategies_prod_status: string;

    cache_subscribed_indicators_status: string;

    cache_tasks_dev_status: string;
    cache_tasks_prod_status: string;

    cache_websocket_status: string;

    // DB
    db_dev_conn_is_valid: string;
    db_prod_conn_is_valid: string;

    // Engine
    engine_last_update_date: string;
    engine_startup_date: string;
    engine_status: string;

    // Runtimes DEV
    runtime_dev_startup_date: string;
    runtime_dev_last_update_date: string;
    runtime_dev_status: string;

    // Runtimes PROD
    runtime_prod_startup_date: string;
    runtime_prod_last_update_date: string;
    runtime_prod_status: string;
}
