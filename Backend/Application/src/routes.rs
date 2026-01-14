use actix_web::web;

use crate::controller::{
    delete_action, delete_asset, delete_indicator, delete_strategy, get_account, get_action,
    get_actions, get_asset, get_assets, get_engine_status, get_indicator, get_indicators,
    get_integrations, get_integrations_settings, get_metric, get_pair, get_pairs,
    get_runtimes_status, get_strategies, get_strategy, get_subscribed_indicators, get_tasks,
    insert_action, insert_asset, insert_indicator, insert_ledger, insert_order, insert_pair,
    insert_strategy, restart_everything, select_action, select_actions, select_asset,
    select_assets, select_configuration, select_error_logs, select_health_check, select_indicator,
    select_indicators, select_integration_logs, select_integrations, select_integrations_settings,
    select_ledger, select_ledgers, select_metrics, select_order, select_orders, select_pair,
    select_pairs, select_record_types, select_status, select_strategies,
    select_strategies_overview, select_strategy, select_tasks, shutdown_engine, start_everything,
    start_tasks_manually, stop_everything, stop_tasks, update_action, update_asset,
    update_indicator, update_integration, update_integration_setting, update_order, update_pair,
    update_strategy, update_task,
};

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/strategies")
            .service(insert_strategy)
            .service(select_strategy)
            .service(select_strategies)
            .service(update_strategy)
            .service(delete_strategy)
            .service(get_strategy)
            .service(get_strategies),
    )
    .service(
        web::scope("/assets")
            .service(insert_asset)
            .service(select_asset)
            .service(select_assets)
            .service(update_asset)
            .service(delete_asset)
            .service(get_asset)
            .service(get_assets),
    )
    .service(web::scope("/record_types").service(select_record_types))
    .service(
        web::scope("/orders")
            .service(insert_order)
            .service(select_order)
            .service(select_orders)
            .service(update_order),
    )
    .service(
        web::scope("/ledgers")
            .service(insert_ledger)
            .service(select_ledger)
            .service(select_ledgers),
    )
    .service(
        web::scope("/pairs")
            .service(insert_pair)
            .service(select_pair)
            .service(update_pair)
            .service(get_pair)
            .service(get_pairs)
            .service(select_pairs),
    )
    .service(
        web::scope("/indicators")
            .service(insert_indicator)
            .service(select_indicator)
            .service(select_indicators)
            .service(update_indicator)
            .service(delete_indicator)
            .service(get_indicator)
            .service(get_indicators)
            .service(get_subscribed_indicators),
    )
    .service(
        web::scope("/actions")
            .service(insert_action)
            .service(select_action)
            .service(select_actions)
            .service(update_action)
            .service(delete_action)
            .service(get_action)
            .service(get_actions),
    )
    .service(web::scope("/strategies_overview").service(select_strategies_overview))
    .service(
        web::scope("/user_commands")
            .service(start_everything)
            .service(stop_everything)
            .service(restart_everything),
    )
    .service(
        web::scope("/metrics")
            .service(select_metrics)
            .service(get_metric),
    )
    .service(web::scope("/binance").service(get_account))
    .service(web::scope("/order_status").service(select_status))
    .service(
        web::scope("/tasks")
            .service(select_tasks)
            .service(get_tasks)
            .service(update_task)
            .service(start_tasks_manually)
            .service(stop_tasks),
    )
    .service(web::scope("/configurations").service(select_configuration))
    .service(
        web::scope("/engines")
            .service(shutdown_engine)
            .service(get_engine_status),
    )
    .service(web::scope("/runtimes").service(get_runtimes_status))
    .service(web::scope("/health_check").service(select_health_check))
    .service(web::scope("/error_log").service(select_error_logs))
    .service(web::scope("/integration_log").service(select_integration_logs))
    .service(
        web::scope("/integrations")
            .service(select_integrations)
            .service(update_integration)
            .service(get_integrations),
    )
    .service(
        web::scope("/integrations_settings")
            .service(select_integrations_settings)
            .service(update_integration_setting)
            .service(get_integrations_settings),
    );
}
