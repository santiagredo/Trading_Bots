use actix_web::web;

use crate::controller::{
    delete_asset, delete_strategy, insert_asset, insert_backtests, insert_ledger, insert_order,
    insert_pair_asset, insert_strategy, select_active_strategies, select_asset, select_assets,
    select_ledger, select_ledgers, select_order, select_pair_asset, select_record_types,
    select_strategy, update_asset, update_order, update_pair_asset, update_strategy,
};

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/strategies")
            .service(insert_strategy)
            .service(select_strategy)
            .service(select_active_strategies)
            .service(update_strategy)
            .service(delete_strategy),
    )
    .service(
        web::scope("/assets")
            .service(insert_asset)
            .service(select_asset)
            .service(select_assets)
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
    .service(web::scope("/backtest").service(insert_backtests))
    .service(
        web::scope("/pair_assets")
            .service(insert_pair_asset)
            .service(select_pair_asset)
            .service(update_pair_asset),
    );
}
