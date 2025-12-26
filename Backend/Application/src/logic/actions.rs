use std::str::FromStr;

use models::{
    entities::{actions, assets, pairs},
    structs::Ticker,
};
use sea_orm::prelude::Decimal;

use crate::{handler::Actions, utils::Logic};

impl Actions<Logic> {
    pub fn insert_action_logic(self) -> Result<Self, String> {
        if self.model.is_sell.is_none() {
            return Err(format!("Missing Is Sell property"));
        }

        if self.model.is_quote_asset.is_none() {
            return Err(format!("Missing Is Quote Asset property"));
        }

        if self.model.is_percentage.is_none() {
            return Err(format!("Missing Is Percentage property"));
        }

        if self.model.value.is_none_or(|val| val == Decimal::ZERO) {
            return Err(format!("Missing or invalid Value property"));
        }

        Ok(self)
    }

    pub fn update_action_logic(self) -> Result<Self, String> {
        if self.model.id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid action ID"));
        }

        if self.model.value.is_some_and(|val| val == Decimal::ZERO) {
            return Err(format!("Invalid Value property"));
        }

        Ok(self)
    }

    pub fn delete_action_logic(self) -> Result<Self, String> {
        if self.model.id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid action ID"));
        }

        Ok(self)
    }

    pub fn evaluate_action_logic(
        self,
        mut action: actions::Model,
        pair: &pairs::Model,
        ticker: &Ticker,
        base_asset: &assets::Model,
        quote_asset: &assets::Model,
    ) -> Result<actions::Model, String> {
        // Decimal == coefficient / 10 ^ scale

        let mut action_value = if action.is_sell {
            match (action.is_quote_asset, action.is_percentage) {
                (true, true) | (false, true) => {
                    base_asset.free * (action.value / Decimal::ONE_HUNDRED)
                }

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

        // validate step size (applies only to amounts in base asset)
        if action_value < pair.lot_size_step_size {
            return Err(format!(
                "Order size {} is less than minimum step size {}",
                action_value, pair.lot_size_step_size
            ));
        }

        // build a valid lot size in base currency
        let mut lot_size_step_size = pair.lot_size_step_size;
        if lot_size_step_size == Decimal::ZERO {
            lot_size_step_size = Decimal::from_str("0.0001").unwrap_or(Decimal::ONE);
        }

        let floored = (action_value / lot_size_step_size).floor();
        action_value = floored * lot_size_step_size;

        // validate min notional (in quote currency)
        let notional_value = action_value * ticker.last_price;

        if notional_value < pair.notional_min_notional {
            return Err(format!(
                "Notional {} is less than minimum allowed {}",
                notional_value, pair.notional_min_notional
            ));
        }

        action.value = action_value;
        // dbg!(action.value);
        Ok(action)
    }
}

#[cfg(test)]
mod fn_insert_action_logic {
    use super::*;
    use crate::utils::Core;
    use models::structs::ActionRequest;

    #[test]
    fn insert_action_cases() {
        let cases = vec![
            (
                "ok_valid_insert",
                ActionRequest {
                    is_sell: Some(true),
                    is_quote_asset: Some(true),
                    is_percentage: Some(false),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_missing_is_sell",
                ActionRequest {
                    is_sell: None,
                    is_quote_asset: Some(true),
                    is_percentage: Some(false),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_is_quote_asset",
                ActionRequest {
                    is_sell: Some(true),
                    is_quote_asset: None,
                    is_percentage: Some(false),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_is_percentage",
                ActionRequest {
                    is_sell: Some(true),
                    is_quote_asset: Some(true),
                    is_percentage: None,
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_value_zero",
                ActionRequest {
                    is_sell: Some(true),
                    is_quote_asset: Some(true),
                    is_percentage: Some(false),
                    value: Some(Decimal::ZERO),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_value",
                ActionRequest {
                    is_sell: Some(true),
                    is_quote_asset: Some(true),
                    is_percentage: Some(false),
                    value: None,
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let action = Actions::new(req);

            let result = action
                .next_phase::<Core>()
                .next_phase()
                .insert_action_logic();

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
mod fn_update_action_logic {
    use super::*;
    use crate::utils::Core;
    use models::structs::ActionRequest;

    #[test]
    fn update_action_cases() {
        let cases = vec![
            (
                "ok_valid_update",
                ActionRequest {
                    id: Some(1),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_invalid_id_zero",
                ActionRequest {
                    id: Some(0),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_id",
                ActionRequest {
                    id: None,
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_value_zero",
                ActionRequest {
                    id: Some(1),
                    value: Some(Decimal::ZERO),
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let action = Actions::new(req);

            let result = action
                .next_phase::<Core>()
                .next_phase()
                .update_action_logic();

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
mod fn_delete_action_logic {
    use super::*;
    use crate::utils::Core;
    use models::structs::ActionRequest;

    #[test]
    fn delete_action_cases() {
        let cases = vec![
            (
                "ok_valid_delete",
                ActionRequest {
                    id: Some(1),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_invalid_id_zero",
                ActionRequest {
                    id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_id",
                ActionRequest {
                    id: None,
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let action = Actions::new(req);

            let result = action
                .next_phase::<Core>()
                .next_phase()
                .delete_action_logic();

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
mod fn_evaluate_action_logic {
    use super::*;
    use models::entities::{actions, assets, pairs};
    use models::structs::{ActionRequest, Ticker};
    use sea_orm::prelude::Decimal;

    fn mock_pair() -> pairs::Model {
        pairs::Model {
            id: 1,
            base_asset_id: 3,
            quote_asset_id: 2,
            symbol: "BTCUSDT".into(),
            lot_size_step_size: Decimal::from_str_exact("0.00001").unwrap(),
            notional_min_notional: Decimal::from_str_exact("5").unwrap(),
            ..Default::default()
        }
    }

    fn mock_base_asset(free: &str) -> assets::Model {
        assets::Model {
            id: 3,
            free: Decimal::from_str_exact(free).unwrap(),
            ..Default::default()
        }
    }

    fn mock_quote_asset(free: &str) -> assets::Model {
        assets::Model {
            id: 2,
            free: Decimal::from_str_exact(free).unwrap(),
            ..Default::default()
        }
    }

    fn mock_ticker(price: &str) -> Ticker {
        Ticker {
            last_price: Decimal::from_str_exact(price).unwrap(),
            ..Default::default()
        }
    }

    #[test]
    fn evaluate_action_cases() {
        let pair = mock_pair();
        let ticker = mock_ticker("100000"); // BTC = 100000 USDT

        let cases = vec![
            (
                "ok_sell_base_absolute",
                actions::Model {
                    is_sell: true,
                    is_quote_asset: false,
                    is_percentage: false,
                    value: Decimal::from(1),
                    ..Default::default()
                },
                mock_base_asset("10"),
                mock_quote_asset("1000"),
                true,
            ),
            (
                "ok_buy_quote_absolute",
                actions::Model {
                    is_sell: false,
                    is_quote_asset: true,
                    is_percentage: false,
                    value: Decimal::from(100), // spend 100 USDT
                    ..Default::default()
                },
                mock_base_asset("10"),
                mock_quote_asset("1000"),
                true,
            ),
            (
                "err_sell_base_insufficient",
                actions::Model {
                    is_sell: true,
                    is_quote_asset: false,
                    is_percentage: false,
                    value: Decimal::from(20), // wants 20 BTC, only 10 available
                    ..Default::default()
                },
                mock_base_asset("10"),
                mock_quote_asset("1000"),
                false,
            ),
            (
                "err_buy_quote_insufficient",
                actions::Model {
                    is_sell: false,
                    is_quote_asset: true,
                    is_percentage: false,
                    value: Decimal::from(2000), // wants to spend 2000 USDT, only 1000 available
                    ..Default::default()
                },
                mock_base_asset("10"),
                mock_quote_asset("1000"),
                false,
            ),
            (
                "err_step_size_too_small",
                actions::Model {
                    is_sell: true,
                    is_quote_asset: false,
                    is_percentage: false,
                    value: Decimal::from_str_exact("0.000001").unwrap(), // smaller than step size
                    ..Default::default()
                },
                mock_base_asset("10"),
                mock_quote_asset("1000"),
                false,
            ),
            (
                "err_notional_too_small",
                actions::Model {
                    is_sell: true,
                    is_quote_asset: false,
                    is_percentage: false,
                    value: Decimal::from_str_exact("0.00001").unwrap(), // 0.00001 * 100000 = 1 < min notional (5)
                    ..Default::default()
                },
                mock_base_asset("10"),
                mock_quote_asset("1000"),
                false,
            ),
        ];

        for (name, action, base, quote, should_pass) in cases {
            let req = ActionRequest::default();

            let result = Actions::new(req).evaluate_action(action, &pair, &ticker, &base, &quote);

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
