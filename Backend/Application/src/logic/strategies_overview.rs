use std::collections::HashMap;

use models::{
    entities::{actions, indicators, pairs, strategies},
    structs::StrategyOverview,
};

use crate::{handler::StrategiesOverview, utils::Logic};

impl StrategiesOverview<Logic> {
    pub fn select_strategies_overview_logic(
        results: (
            Vec<strategies::Model>,
            Vec<indicators::Model>,
            Vec<actions::Model>,
            Vec<pairs::Model>,
        ),
    ) -> Vec<StrategyOverview> {
        let (strategies, indicators, actions, pairs) = results;

        let mut indicators_map: HashMap<i32, indicators::Model> =
            indicators.into_iter().map(|i| (i.strategy_id, i)).collect();

        let mut actions_map: HashMap<i32, actions::Model> =
            actions.into_iter().map(|a| (a.strategy_id, a)).collect();

        let mut pairs_map: HashMap<i32, pairs::Model> =
            pairs.into_iter().map(|p| (p.id, p)).collect();

        let mut overview = vec![];

        for strategy in strategies {
            let indicator = indicators_map.remove(&strategy.id).unwrap_or_default();
            let action = actions_map.remove(&strategy.id).unwrap_or_default();
            let pair = pairs_map.remove(&action.pair_id).unwrap_or_default();

            overview.push(StrategyOverview {
                strategy,
                indicator,
                action,
                pair,
                ..Default::default()
            });
        }

        overview
    }

    // pub fn build_asset_ledger_request_logic(
    //     strategy_overview: &StrategyOverview,
    //     order: &orders::Model,
    //     is_base: bool,
    // ) -> (AssetRequest, LedgerRequest) {
    //     let asset = match is_base {
    //         true => &strategy_overview.base_asset,
    //         false => &strategy_overview.quote_asset,
    //     };

    //     let value = match is_base {
    //         true => &order.base_asset_amount,
    //         false => &order.quote_asset_amount,
    //     };

    //     let previous_balance = match is_base {
    //         true => &strategy_overview.base_asset.free,
    //         false => &strategy_overview.quote_asset.free,
    //     };

    //     let is_sell = is_base == order.is_sell;

    //     let asset_request =
    //         AssetRequest::from_model(asset).aggregate_values(*value, false, is_sell);

    //     let asset_ledger = LedgerRequest::from_asset(asset)
    //         .from_order(&order)
    //         .update_values(
    //             false,
    //             *value,
    //             *previous_balance,
    //             asset_request.free.unwrap_or_default(),
    //         );

    //     (asset_request, asset_ledger)
    // }
}

// #[cfg(test)]
// mod fn_build_asset_ledger_request {
//     use models::entities::assets;
//     use sea_orm::prelude::Decimal;

//     use super::*;

//     fn make_strategy_overview() -> StrategyOverview {
//         StrategyOverview {
//             base_asset: assets::Model {
//                 id: 1,
//                 name: "BTC".to_string(),
//                 ticker: "BTC".to_string(),
//                 free: Decimal::new(10, 0),
//                 locked: Decimal::ZERO,
//             },
//             quote_asset: assets::Model {
//                 id: 2,
//                 name: "USDT".to_string(),
//                 ticker: "USDT".to_string(),
//                 free: Decimal::new(1000, 0),
//                 locked: Decimal::ZERO,
//             },
//             ..Default::default()
//         }
//     }

//     fn make_order() -> orders::Model {
//         orders::Model {
//             id: 1,
//             status_id: 1,
//             creation_date: chrono::Local::now().naive_local(),
//             update_date: chrono::Local::now().naive_local(),
//             is_sell: false,
//             strategy_id: 1,
//             base_asset_id: 1,
//             base_asset_amount: Decimal::new(2, 0), // 2 BTC
//             quote_asset_id: 2,
//             quote_asset_amount: Decimal::new(40000, 0), // 40,000 USDT
//             price_entry: Decimal::new(20000, 0),
//             price_target: Decimal::new(20000, 0),
//             price_abort: Decimal::ZERO,
//         }
//     }

//     #[test]
//     fn build_asset_ledger_request_cases() {
//         let overview = make_strategy_overview();
//         let order = make_order();

//         let cases = vec![
//             ("ok_base_asset", true, overview.clone(), order.clone()),
//             ("ok_quote_asset", false, overview.clone(), order.clone()),
//         ];

//         for (name, is_base, overview, order) in cases {
//             let (asset_req, ledger_req) =
//                 StrategiesOverview::build_asset_ledger_request_logic(&overview, &order, is_base);

//             if is_base {
//                 assert_eq!(
//                     asset_req.id,
//                     Some(overview.base_asset.id),
//                     "case `{}` failed: expected base_asset id",
//                     name
//                 );
//                 assert_eq!(
//                     ledger_req.asset_id,
//                     Some(overview.base_asset.id),
//                     "case `{}` failed: expected ledger to use base_asset",
//                     name
//                 );
//             } else {
//                 assert_eq!(
//                     asset_req.id,
//                     Some(overview.quote_asset.id),
//                     "case `{}` failed: expected quote_asset id",
//                     name
//                 );
//                 assert_eq!(
//                     ledger_req.asset_id,
//                     Some(overview.quote_asset.id),
//                     "case `{}` failed: expected ledger to use quote_asset",
//                     name
//                 );
//             }
//         }
//     }
// }
