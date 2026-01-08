export interface CacheHealthCheck {
    cache_actions_dev_is_initialized: string;
    cache_actions_prod_is_initialized: string;
    cache_assets_dev_is_initialized: string;
    cache_assets_prod_is_initialized: string;
    cache_configuration_is_some: string;
    cache_indicators_dev_is_initialized: string;
    cache_indicators_prod_is_initialized: string;
    cache_metrics_dev_is_some: string;
    cache_metrics_prod_is_some: string;
    cache_pairs_dev_is_initialized: string;
    cache_pairs_prod_is_initialized: string;
    cache_status_dev_is_initialized: string;
    cache_status_prod_is_initialized: string;
    cache_strategies_dev_is_initialized: string;
    cache_strategies_prod_is_initialized: string;
    cache_subscribed_indicators_dev_is_initialized: string;
    cache_subscribed_indicators_prod_is_initialized: string;
    cache_tasks_dev_is_initialized: string;
    cache_tasks_prod_is_initialized: string;
    cache_ws_binance_abort_handle_is_finished: string;
    cache_ws_binance_abort_handle_is_some: string;
    db_dev_conn_is_valid: string;
    db_prod_conn_is_valid: string;
    startup_date: string;
    uptime: string;
}
