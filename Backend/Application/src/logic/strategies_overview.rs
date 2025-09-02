use std::collections::HashMap;

use models::{
    entities::{
        actions, indicators,
        orders::{self},
        pairs, strategies,
    },
    enums::Status,
    structs::{AssetRequest, LedgerRequest, StrategyOverview},
};

use crate::{
    handler::{Actions, Indicators, Orders, Strategies, StrategiesOverview},
    utils::Logic,
};

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

    pub fn evaluate_strategy_overview_logic(
        strategy_overview: &StrategyOverview,
    ) -> Result<Orders, String> {
        // evaluate cooldown has passed
        if !Strategies::default().evaluate_cooldown(
            strategy_overview.strategy.last_execution,
            strategy_overview.strategy.cooldown,
        ) {
            return Err(format!("Cooldown is active"));
        }

        // evalute indicator
        match Indicators::evalute_active_indicators(
            &strategy_overview.ticker,
            &strategy_overview.indicator,
            &strategy_overview.pair,
        ) {
            Err(err) => return Err(err),
            Ok(false) => return Err(format!("Indicator not met")),
            Ok(true) => (),
        };

        // evaluate and modify action
        let action = match Actions::evaluate_active_actions(
            strategy_overview.action.clone(),
            &strategy_overview.pair,
            &strategy_overview.base_asset,
            &strategy_overview.quote_asset,
            &strategy_overview.ticker,
        ) {
            Err(err) => return Err(err),
            Ok(val) => val,
        };

        // create order
        let order_status = *strategy_overview
            .order_status
            .get(&Status::Completed)
            .unwrap_or(&2);

        let order = Orders::default()
            .from_strategy(&strategy_overview.strategy)
            .from_action(&action)
            .from_pair(&strategy_overview.pair)
            .from_ticker(&strategy_overview.ticker)
            .from_status(order_status);

        Ok(order)
    }

    pub fn build_asset_ledger_request_logic(
        strategy_overview: &StrategyOverview,
        order: &orders::Model,
        is_base: bool,
    ) -> (AssetRequest, LedgerRequest) {
        let asset = match is_base {
            true => &strategy_overview.base_asset,
            false => &strategy_overview.quote_asset,
        };

        let value = match is_base {
            true => &order.base_asset_amount,
            false => &order.quote_asset_amount,
        };

        let previous_balance = match is_base {
            true => &strategy_overview.base_asset.free,
            false => &strategy_overview.quote_asset.free,
        };

        let asset_request =
            AssetRequest::from_model(asset).aggregate_values(*value, false, is_base);

        let asset_ledger = LedgerRequest::from_asset(asset)
            .from_order(&order)
            .update_values(
                false,
                *value,
                *previous_balance,
                asset_request.free.unwrap_or_default(),
            );

        (asset_request, asset_ledger)
    }
}

#[cfg(test)]
mod fn_evaluate_strategy_overview_logic {
    use models::{
        entities::{actions, assets, indicators, pairs},
        enums::Status,
        structs::Ticker,
    };
    use sea_orm::prelude::Decimal;

    use super::*;

    fn make_strategy_overview() -> StrategyOverview {
        let mut order_status = HashMap::new();
        order_status.insert(Status::Open, 1);
        order_status.insert(Status::Completed, 2);
        order_status.insert(Status::Aborted, 3);

        StrategyOverview {
            strategy: strategies::Model {
                id: 1,
                name: "Scalping".to_string(),
                is_active: true,
                can_trade: true,
                last_execution: None,
                cooldown: Some(1),
                ..Default::default()
            },
            indicator: indicators::Model {
                id: 1,
                strategy_id: 1,
                is_active: true,
                nick: "lst".to_string(),      // last_price
                direction: "gte".to_string(), // >=
                value: Decimal::ONE,          // 1
                ..Default::default()
            },
            action: actions::Model {
                id: 1,
                strategy_id: 1,
                is_active: true,
                is_sell: false,
                is_quote_asset: false,
                is_percentage: false,
                value: Decimal::ONE, // 1 base asset
                pair_id: 1,
            },
            pair: pairs::Model {
                id: 1,
                base_asset_id: 1,
                quote_asset_id: 2,
                symbol: "BTCUSDT".to_string(),
                update_date: chrono::Local::now().naive_local(),
                all_time_high_price: Decimal::new(100000, 0),
                all_time_high_date: chrono::Local::now().naive_local(),
                percent_from_all_time_high: Decimal::new(10, 0),
                lot_size_step_size: Decimal::ONE,
                notional_min_notional: Decimal::ONE,
                ..Default::default()
            },
            base_asset: assets::Model {
                id: 1,
                name: "BTC".to_string(),
                ticker: "BTC".to_string(),
                free: Decimal::new(10, 0),
                locked: Decimal::ZERO,
            },
            quote_asset: assets::Model {
                id: 2,
                name: "USDT".to_string(),
                ticker: "USDT".to_string(),
                free: Decimal::new(100000, 0),
                locked: Decimal::ZERO,
            },
            ticker: Ticker {
                symbol: "BTCUSDT".to_string(),
                last_price: Decimal::new(20000, 0),
                ..Default::default()
            },
            order_status,
        }
    }

    #[test]
    fn evaluate_strategy_overview_logic_cases() {
        let base_case = make_strategy_overview();

        let cases = vec![
            (
                "err_cooldown_active",
                {
                    let mut s = base_case.clone();
                    s.strategy.last_execution = Some(chrono::Local::now().naive_local());
                    s
                },
                false,
            ),
            (
                "err_indicator_not_met",
                {
                    let mut s = base_case.clone();
                    s.indicator.direction = "gt".to_string(); // exige last_price > 1
                    s.indicator.value = Decimal::new(30000, 0); // 30k
                    s
                },
                false,
            ),
            (
                "ok_valid_case",
                {
                    let s = base_case.clone();
                    s
                },
                true,
            ),
        ];

        for (name, overview, should_pass) in cases {
            let result = StrategiesOverview::evaluate_strategy_overview_logic(&overview);

            assert_eq!(
                result.is_ok(),
                should_pass,
                "case `{}` failed: expected {}, got {:?}",
                name,
                should_pass,
                result
            );
        }
    }
}

#[cfg(test)]
mod fn_build_asset_ledger_request {
    use models::entities::assets;
    use sea_orm::prelude::Decimal;

    use super::*;

    fn make_strategy_overview() -> StrategyOverview {
        StrategyOverview {
            base_asset: assets::Model {
                id: 1,
                name: "BTC".to_string(),
                ticker: "BTC".to_string(),
                free: Decimal::new(10, 0),
                locked: Decimal::ZERO,
            },
            quote_asset: assets::Model {
                id: 2,
                name: "USDT".to_string(),
                ticker: "USDT".to_string(),
                free: Decimal::new(1000, 0),
                locked: Decimal::ZERO,
            },
            ..Default::default()
        }
    }

    fn make_order() -> orders::Model {
        orders::Model {
            id: 1,
            status_id: 1,
            creation_date: chrono::Local::now().naive_local(),
            update_date: chrono::Local::now().naive_local(),
            is_sell: false,
            strategy_id: 1,
            base_asset_id: 1,
            base_asset_amount: Decimal::new(2, 0), // 2 BTC
            quote_asset_id: 2,
            quote_asset_amount: Decimal::new(40000, 0), // 40,000 USDT
            price_entry: Decimal::new(20000, 0),
            price_target: Decimal::new(20000, 0),
            price_abort: Decimal::ZERO,
        }
    }

    #[test]
    fn build_asset_ledger_request_cases() {
        let overview = make_strategy_overview();
        let order = make_order();

        let cases = vec![
            ("ok_base_asset", true, overview.clone(), order.clone()),
            ("ok_quote_asset", false, overview.clone(), order.clone()),
        ];

        for (name, is_base, overview, order) in cases {
            let (asset_req, ledger_req) =
                StrategiesOverview::build_asset_ledger_request_logic(&overview, &order, is_base);

            if is_base {
                assert_eq!(
                    asset_req.id,
                    Some(overview.base_asset.id),
                    "case `{}` failed: expected base_asset id",
                    name
                );
                assert_eq!(
                    ledger_req.asset_id,
                    Some(overview.base_asset.id),
                    "case `{}` failed: expected ledger to use base_asset",
                    name
                );
            } else {
                assert_eq!(
                    asset_req.id,
                    Some(overview.quote_asset.id),
                    "case `{}` failed: expected quote_asset id",
                    name
                );
                assert_eq!(
                    ledger_req.asset_id,
                    Some(overview.quote_asset.id),
                    "case `{}` failed: expected ledger to use quote_asset",
                    name
                );
            }
        }
    }
}
