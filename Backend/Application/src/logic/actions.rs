use std::str::FromStr;

use models::{
    entities::{
        actions::{self, Model},
        assets, pairs,
    },
    structs::{ActionRequest, Ticker},
};
use sea_orm::prelude::Decimal;

/* ======================================================
 * VALIDATIONS
 * ======================================================
 */

pub fn validate_insert(req: &ActionRequest, stored_actions: Vec<Model>) -> Result<(), String> {
    if req.strategy_id.is_none_or(|strat_id| strat_id <= 0) {
        return Err("Missing Strategy id".into());
    }

    if req.is_sell.is_none() {
        return Err("Missing Is Sell property".into());
    }

    if req.is_quote_asset.is_none() {
        return Err("Missing Is Quote Asset property".into());
    }

    if req.is_percentage.is_none() {
        return Err("Missing Is Percentage property".into());
    }

    if req.value.is_none_or(|v| v == Decimal::ZERO) {
        return Err("Missing or invalid Value property".into());
    }

    if !stored_actions.is_empty() {
        return Err("An Action already exists for this strategy".into());
    }

    Ok(())
}

pub fn validate_update(req: &ActionRequest) -> Result<(), String> {
    if req.id.is_none_or(|id| id <= 0) {
        return Err(format!("Invalid action ID: {:?}", req.id));
    }

    if req.value.is_some_and(|v| v == Decimal::ZERO) {
        return Err("Invalid Value property".into());
    }

    Ok(())
}

pub fn validate_delete(req: &ActionRequest) -> Result<(), String> {
    if req.id.is_none_or(|id| id <= 0) {
        return Err(format!("Invalid action ID: {:?}", req.id));
    }

    Ok(())
}

/* ======================================================
 * EVALUATION
 * ======================================================
 */

pub fn evaluate_action(
    mut action: actions::Model,
    pair: &pairs::Model,
    ticker: &Ticker,
    base_asset: &assets::Model,
    quote_asset: &assets::Model,
) -> Result<actions::Model, String> {
    let mut action_value = if action.is_sell {
        match (action.is_quote_asset, action.is_percentage) {
            (_, true) => base_asset.free * (action.value / Decimal::ONE_HUNDRED),
            (true, false) => action.value / ticker.last_price,
            (false, false) => action.value,
        }
    } else {
        match (action.is_quote_asset, action.is_percentage) {
            (true, true) => {
                (quote_asset.free * (action.value / Decimal::ONE_HUNDRED)) / ticker.last_price
            }
            (true, false) => action.value / ticker.last_price,
            (false, true) => base_asset.free * (action.value / Decimal::ONE_HUNDRED),
            (false, false) => action.value,
        }
    };

    match action.is_sell {
        true => {
            if action_value > base_asset.free {
                return Err(format!(
                    "Not enough base asset. Needed {}, but free {}",
                    action_value, base_asset.free
                ));
            }
        }
        false => {
            let required_quote = action_value * ticker.last_price;

            if required_quote > quote_asset.free {
                return Err(format!(
                    "Not enough quote asset. Needed {}, but free {}",
                    required_quote, quote_asset.free
                ));
            }
        }
    }

    let mut step = pair.lot_size_step_size;
    if step == Decimal::ZERO {
        step = Decimal::from_str("0.0001").unwrap_or(Decimal::ONE);
    }

    let floored = (action_value / step).floor();
    action_value = floored * step;

    if action_value < pair.lot_size_min_qty {
        return Err(format!(
            "Order size {} is less than min qty {}",
            action_value, pair.lot_size_min_qty
        ));
    }

    if action_value > pair.lot_size_max_qty {
        return Err(format!(
            "Order size {} is greater than max qty {}",
            action_value, pair.lot_size_max_qty
        ));
    }

    let notional = action_value * ticker.last_price;

    if notional < pair.notional_min_notional {
        return Err(format!(
            "Notional {} is less than minimum allowed {}",
            notional, pair.notional_min_notional
        ));
    }

    action.value = action_value;
    Ok(action)
}

/* ======================================================
 * TESTS
 * ======================================================
 */

#[cfg(test)]
mod fn_validate_insert {
    use super::*;

    #[test]
    fn cases() {
        let cases = vec![
            (
                "ok",
                ActionRequest {
                    strategy_id: Some(1),
                    is_sell: Some(true),
                    is_quote_asset: Some(true),
                    is_percentage: Some(false),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                vec![], // no stored actions
                true,
            ),
            (
                "err_missing_strategy_id",
                ActionRequest {
                    strategy_id: None,
                    is_sell: Some(true),
                    is_quote_asset: Some(true),
                    is_percentage: Some(false),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                vec![],
                false,
            ),
            (
                "err_missing_is_sell",
                ActionRequest {
                    strategy_id: Some(1),
                    is_sell: None,
                    is_quote_asset: Some(true),
                    is_percentage: Some(false),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                vec![],
                false,
            ),
            (
                "err_value_zero",
                ActionRequest {
                    strategy_id: Some(1),
                    is_sell: Some(true),
                    is_quote_asset: Some(true),
                    is_percentage: Some(false),
                    value: Some(Decimal::ZERO),
                    ..Default::default()
                },
                vec![],
                false,
            ),
            (
                "err_action_already_exists",
                ActionRequest {
                    strategy_id: Some(1),
                    is_sell: Some(true),
                    is_quote_asset: Some(true),
                    is_percentage: Some(false),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                vec![Model::default()], // simula que ya existe una acción
                false,
            ),
        ];

        for (name, req, stored_actions, should_pass) in cases {
            let result = validate_insert(&req, stored_actions);
            assert_eq!(result.is_ok(), should_pass, "case `{}` failed", name);
        }
    }
}

#[cfg(test)]
mod fn_validate_update {
    use super::*;

    #[test]
    fn cases() {
        let cases = vec![
            (
                "ok",
                ActionRequest {
                    id: Some(1),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_invalid_id",
                ActionRequest {
                    id: Some(0),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let result = validate_update(&req);
            assert_eq!(result.is_ok(), should_pass, "case `{}` failed", name);
        }
    }
}

#[cfg(test)]
mod fn_validate_delete {
    use super::*;

    #[test]
    fn cases() {
        let cases = vec![
            (
                "ok",
                ActionRequest {
                    id: Some(1),
                    ..Default::default()
                },
                true,
            ),
            (
                "err",
                ActionRequest {
                    id: None,
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let result = validate_delete(&req);
            assert_eq!(result.is_ok(), should_pass, "case `{}` failed", name);
        }
    }
}

#[cfg(test)]
mod fn_evaluate_action {
    use super::*;
    use sea_orm::prelude::Decimal;

    fn pair() -> pairs::Model {
        pairs::Model {
            lot_size_step_size: Decimal::from_str_exact("0.00001").unwrap(),
            lot_size_max_qty: Decimal::ONE_THOUSAND,
            notional_min_notional: Decimal::from_str_exact("5").unwrap(),
            ..Default::default()
        }
    }

    fn asset(id: i32, free: &str) -> assets::Model {
        assets::Model {
            id,
            free: Decimal::from_str_exact(free).unwrap(),
            ..Default::default()
        }
    }

    fn ticker(price: &str) -> Ticker {
        Ticker {
            last_price: Decimal::from_str_exact(price).unwrap(),
            ..Default::default()
        }
    }

    #[test]
    fn ok_sell() {
        let action = actions::Model {
            is_sell: true,
            is_quote_asset: false,
            is_percentage: false,
            value: Decimal::ONE,
            ..Default::default()
        };

        let result = evaluate_action(
            action,
            &pair(),
            &ticker("100000"),
            &asset(1, "10"),
            &asset(2, "1000"),
        );

        assert!(result.is_ok());
    }
}
