use actix_web::web;

use crate::controller::{
    delete_action, delete_asset, delete_indicator, delete_strategy, get_account, get_active_action,
    get_active_actions, get_active_asset, get_active_assets, get_active_indicator,
    get_active_indicators, get_active_metric, get_active_pair, get_active_pairs,
    get_active_strategies, get_active_strategy, get_subscribed_indicators, insert_action,
    insert_asset, insert_configuration, insert_indicator, insert_ledger, insert_order, insert_pair,
    insert_strategy, restart_everything, select_action, select_actions, select_active_tasks,
    select_asset, select_assets, select_configuration, select_health_check, select_indicator,
    select_indicators, select_ledger, select_ledgers, select_metrics, select_order, select_pair,
    select_pairs, select_record_types, select_status, select_strategies,
    select_strategies_overview, select_strategy, select_tasks, shutdown_engine,
    start_active_actions, start_active_assets, start_active_indicators, start_active_pairs,
    start_active_strategies, start_active_tasks, start_everything, stop_active_actions,
    stop_active_assets, stop_active_indicators, stop_active_pairs, stop_active_strategies,
    stop_active_tasks, stop_everything, update_action, update_asset, update_indicator,
    update_order, update_pair, update_strategy, update_task,
};

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/strategies")
            .service(insert_strategy)
            .service(select_strategy)
            .service(select_strategies)
            .service(update_strategy)
            .service(delete_strategy)
            .service(get_active_strategy)
            .service(get_active_strategies)
            .service(start_active_strategies)
            .service(stop_active_strategies),
    )
    .service(
        web::scope("/assets")
            .service(insert_asset)
            .service(select_asset)
            .service(select_assets)
            .service(update_asset)
            .service(delete_asset)
            .service(get_active_asset)
            .service(get_active_assets)
            .service(start_active_assets)
            .service(stop_active_assets),
    )
    .service(web::scope("/record_types").service(select_record_types))
    .service(
        web::scope("/orders")
            .service(insert_order)
            .service(select_order)
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
            .service(get_active_pair)
            .service(get_active_pairs)
            .service(start_active_pairs)
            .service(stop_active_pairs)
            .service(select_pairs),
    )
    .service(
        web::scope("/indicators")
            .service(insert_indicator)
            .service(select_indicator)
            .service(select_indicators)
            .service(update_indicator)
            .service(delete_indicator)
            .service(get_active_indicator)
            .service(get_active_indicators)
            .service(get_subscribed_indicators)
            .service(start_active_indicators)
            .service(stop_active_indicators),
    )
    .service(
        web::scope("/actions")
            .service(insert_action)
            .service(select_action)
            .service(select_actions)
            .service(update_action)
            .service(delete_action)
            .service(get_active_action)
            .service(get_active_actions)
            .service(start_active_actions)
            .service(stop_active_actions),
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
            .service(get_active_metric),
    )
    .service(web::scope("/binance").service(get_account))
    .service(web::scope("/order_status").service(select_status))
    .service(
        web::scope("/tasks")
            .service(select_tasks)
            .service(select_active_tasks)
            .service(start_active_tasks)
            .service(stop_active_tasks)
            .service(update_task),
    )
    .service(
        web::scope("/configurations")
            .service(insert_configuration)
            .service(select_configuration)
            .service(shutdown_engine),
    )
    .service(web::scope("/health_check").service(select_health_check));
}
