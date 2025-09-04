use actix_web::web;

use crate::controller::{
    delete_action, delete_asset, delete_indicator, delete_strategy, get_account, get_assets,
    get_subscribed_indicators, insert_action, insert_asset, insert_indicator, insert_ledger,
    insert_order, insert_pair, insert_strategy, refresh_everything, select_action, select_actions,
    select_active_tasks, select_asset, select_assets, select_indicator, select_indicators,
    select_ledger, select_ledgers, select_metrics, select_order, select_pair, select_record_types,
    select_status, select_strategies, select_strategies_overview, select_strategy, select_tasks,
    start_everything, stop_everything, update_action, update_asset, update_indicator, update_order,
    update_pair, update_strategy,
};

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/strategies")
            .service(insert_strategy)
            .service(select_strategy)
            .service(select_strategies)
            .service(update_strategy)
            .service(delete_strategy),
    )
    .service(
        web::scope("/assets")
            .service(insert_asset)
            .service(select_asset)
            .service(select_assets)
            .service(get_assets)
            .service(update_asset)
            .service(delete_asset),
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
            .service(update_pair),
    )
    .service(
        web::scope("/indicators")
            .service(insert_indicator)
            .service(select_indicator)
            .service(select_indicators)
            .service(update_indicator)
            .service(delete_indicator)
            .service(get_subscribed_indicators),
    )
    .service(
        web::scope("/actions")
            .service(insert_action)
            .service(select_action)
            .service(select_actions)
            .service(update_action)
            .service(delete_action),
    )
    .service(web::scope("/strategies_overview").service(select_strategies_overview))
    .service(
        web::scope("/user_commands")
            .service(start_everything)
            .service(stop_everything)
            .service(refresh_everything),
    )
    .service(web::scope("/metrics").service(select_metrics))
    .service(web::scope("/binance").service(get_account))
    .service(web::scope("/order_status").service(select_status))
    .service(
        web::scope("/tasks")
            .service(select_tasks)
            .service(select_active_tasks),
    );
}
