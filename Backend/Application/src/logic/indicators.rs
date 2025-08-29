use models::{
    entities::{indicators, pairs},
    enums::{IndicatorKeys, OperationKeys},
    structs::Ticker,
};
use sea_orm::prelude::Decimal;

use crate::{
    types::Indicators,
    utils::{Logic, Utils},
};

impl Indicators<Logic> {
    pub fn insert_indicator_logic(self) -> Result<Self, String> {
        if self
            .model
            .strategy_id
            .is_none_or(|strategy_id| strategy_id <= 0)
        {
            return Err("Invalid strategy ID".to_string());
        }

        Utils::validate_empty_field(
            self.model.symbol.clone().unwrap_or_default(),
            "Indicator symbol",
        )?;

        if !IndicatorKeys::contains(&self.model.nick.clone().unwrap_or_default()) {
            return Err("Invalid indicator nick".to_string());
        }

        if !OperationKeys::contains(&self.model.direction.clone().unwrap_or_default()) {
            return Err("Invalid direction".to_string());
        }

        if self.model.value.is_none_or(|value| value == Decimal::ZERO) {
            return Err("Invalid indicator value".to_string());
        }

        Ok(self)
    }

    pub fn update_indicator_logic(self) -> Result<Self, String> {
        if self.model.id.is_none_or(|id| id <= 0) {
            return Err("Invalid indicator ID".to_string());
        }

        if self
            .model
            .strategy_id
            .is_none_or(|strategy_id| strategy_id <= 0)
        {
            return Err("Invalid strategy ID".to_string());
        }

        if let Some(symbol) = self.model.symbol.clone() {
            Utils::validate_empty_field(symbol, "Indicator symbol")?;
        }

        if matches!(self.model.nick.clone(), Some(ref n) if !IndicatorKeys::contains(n)) {
            return Err("Invalid indicator nick".to_string());
        }

        if matches!(self.model.direction.clone(), Some(ref d) if !OperationKeys::contains(d)) {
            return Err("Invalid direction".to_string());
        }

        if self.model.value.is_none_or(|value| value == Decimal::ZERO) {
            return Err("Invalid indicator value".to_string());
        }

        Ok(self)
    }

    pub fn delete_indicator_logic(self) -> Result<Self, String> {
        if self.model.id.is_none() || self.model.id.is_some_and(|id| id <= 0) {
            return Err("Invalid indicator ID".to_string());
        }

        Ok(self)
    }

    pub fn evaluate_indicator_logic(
        self,
        indicator: &indicators::Model,
        ticker: &Ticker,
        pair: &pairs::Model,
    ) -> Result<bool, String> {
        let Some(indicator_key) = IndicatorKeys::from_str(&indicator.nick) else {
            return Err("Indicator key not found".to_owned());
        };

        let ticker_value = match Self::get_ticker_value(indicator_key, ticker, pair) {
            val if val == Decimal::ZERO => return Err("Ticker value not found".to_owned()),
            val => val,
        };

        let Some(operation_key) = OperationKeys::from_str(&indicator.direction) else {
            return Err("Operation key not found".to_owned());
        };

        let is_valid = Self::compare(operation_key, ticker_value, indicator.value);
        // dbg!(ticker_value);
        // dbg!(indicator.value);
        // dbg!(is_valid);

        Ok(is_valid)
    }

    fn compare(op: OperationKeys, left: Decimal, right: Decimal) -> bool {
        match op {
            OperationKeys::Gte => left >= right,
            OperationKeys::Lte => left <= right,
            OperationKeys::Gt => left > right,
            OperationKeys::Lt => left < right,
            OperationKeys::Eq => left == right,
            OperationKeys::Neq => left != right,
        }
    }

    fn get_ticker_value(key: IndicatorKeys, ticker: &Ticker, pair: &pairs::Model) -> Decimal {
        match key {
            IndicatorKeys::Pch => ticker.price_change,
            IndicatorKeys::Pcp => ticker.price_change_percent,
            IndicatorKeys::Wap => ticker.weighted_avg_price,
            IndicatorKeys::Lst => ticker.last_price,
            IndicatorKeys::Opn => ticker.open_price,
            IndicatorKeys::Hgh => ticker.high_price,
            IndicatorKeys::Low => ticker.low_price,
            IndicatorKeys::Bav => ticker.base_asset_volume,
            IndicatorKeys::Qav => ticker.quote_asset_volume,
            IndicatorKeys::Ath => pair.all_time_high_price,
            IndicatorKeys::Pfath => pair.percent_from_all_time_high,
        }
    }
}

#[cfg(test)]
mod fn_insert_indicator_logic {
    use super::*;
    use crate::utils::Core;
    use models::structs::IndicatorRequest;

    #[test]
    fn insert_indicator_cases() {
        let cases = vec![
            (
                "ok_valid_insert",
                IndicatorRequest {
                    strategy_id: Some(1),
                    symbol: Some("BTCUSDT".to_string()),
                    nick: Some("lst".to_string()),
                    direction: Some("gte".to_string()),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_invalid_strategy_id",
                IndicatorRequest {
                    strategy_id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_symbol",
                IndicatorRequest {
                    strategy_id: Some(1),
                    symbol: Some("".to_string()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_nick",
                IndicatorRequest {
                    strategy_id: Some(1),
                    symbol: Some("BTCUSDT".to_string()),
                    nick: Some("abc".to_string()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_direction",
                IndicatorRequest {
                    strategy_id: Some(1),
                    symbol: Some("BTCUSDT".to_string()),
                    nick: Some("lst".to_string()),
                    direction: Some("abc".to_string()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_value",
                IndicatorRequest {
                    strategy_id: Some(1),
                    symbol: Some("BTCUSDT".to_string()),
                    nick: Some("lst".to_string()),
                    direction: Some("gte".to_string()),
                    value: Some(Decimal::ZERO),
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let indicator = Indicators::new(req);

            let result = indicator
                .next_phase::<Core>()
                .next_phase()
                .insert_indicator_logic();

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
mod fn_update_indicator_logic {
    use super::*;
    use crate::utils::Core;
    use models::structs::IndicatorRequest;

    #[test]
    fn update_indicator_cases() {
        let cases = vec![
            (
                "ok_valid_update",
                IndicatorRequest {
                    id: Some(1),
                    strategy_id: Some(1),
                    symbol: Some("BTCUSDT".to_string()),
                    nick: Some("lst".to_string()),
                    direction: Some("gte".to_string()),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_invalid_id",
                IndicatorRequest {
                    id: Some(0),
                    strategy_id: Some(1),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_strategy_id",
                IndicatorRequest {
                    id: Some(1),
                    strategy_id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_symbol",
                IndicatorRequest {
                    id: Some(1),
                    strategy_id: Some(1),
                    symbol: Some("".to_string()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_nick",
                IndicatorRequest {
                    id: Some(1),
                    strategy_id: Some(1),
                    symbol: Some("BTCUSDT".to_string()),
                    nick: Some("abc".to_string()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_direction",
                IndicatorRequest {
                    id: Some(1),
                    strategy_id: Some(1),
                    symbol: Some("BTCUSDT".to_string()),
                    nick: Some("lst".to_string()),
                    direction: Some("abc".to_string()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_value",
                IndicatorRequest {
                    id: Some(1),
                    strategy_id: Some(1),
                    symbol: Some("BTCUSDT".to_string()),
                    nick: Some("lst".to_string()),
                    direction: Some("gte".to_string()),
                    value: Some(Decimal::ZERO),
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let indicator = Indicators::new(req);

            let result = indicator
                .next_phase::<Core>()
                .next_phase()
                .update_indicator_logic();

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
mod fn_delete_indicator_logic {
    use models::structs::IndicatorRequest;

    use crate::utils::Core;

    use super::*;

    #[test]
    fn valid_delete_indicator_logic() {
        let indicator = Indicators::new(IndicatorRequest {
            id: Some(1),
            ..Default::default()
        });

        let result = indicator
            .next_phase::<Core>()
            .next_phase()
            .delete_indicator_logic();

        assert!(result.is_ok())
    }

    #[test]
    fn invalid_delete_indicator_logic() {
        let indicator = Indicators::new(IndicatorRequest {
            id: Some(0),
            ..Default::default()
        });

        let result = indicator
            .next_phase::<Core>()
            .next_phase()
            .delete_indicator_logic();

        assert!(result.is_err())
    }
}

#[cfg(test)]
mod fn_evaluate_indicator_logic {
    use crate::utils::Core;

    use super::*;

    #[test]
    fn ok_valid_indicator() {
        let indicator = indicators::Model {
            nick: format!("lst"),
            direction: format!("gte"),
            ..Default::default()
        };

        let ticker = Ticker {
            last_price: Decimal::ONE_THOUSAND,
            ..Default::default()
        };

        let pair = pairs::Model::default();

        let result = Indicators::default()
            .next_phase::<Core>()
            .next_phase()
            .evaluate_indicator_logic(&indicator, &ticker, &pair);

        assert!(result.is_ok_and(|res| res));
    }

    #[test]
    fn ok_invalid_indicator() {
        let indicator = indicators::Model {
            nick: format!("lst"),
            direction: format!("lte"),
            ..Default::default()
        };

        let ticker = Ticker {
            last_price: Decimal::ONE_THOUSAND,
            ..Default::default()
        };

        let pair = pairs::Model::default();

        let result = Indicators::default()
            .next_phase::<Core>()
            .next_phase()
            .evaluate_indicator_logic(&indicator, &ticker, &pair);

        assert!(result.is_ok_and(|res| !res));
    }

    #[test]
    fn err_indicator_nick() {
        let indicator = indicators::Model {
            nick: format!("abc"),
            direction: format!("lte"),
            ..Default::default()
        };

        let ticker = Ticker {
            last_price: Decimal::ONE_THOUSAND,
            ..Default::default()
        };

        let pair = pairs::Model::default();

        let result = Indicators::default()
            .next_phase::<Core>()
            .next_phase()
            .evaluate_indicator_logic(&indicator, &ticker, &pair);

        assert!(result.is_err());
    }

    #[test]
    fn err_indicator_key() {
        let indicator = indicators::Model {
            nick: format!("lst"),
            direction: format!("abc"),
            ..Default::default()
        };

        let ticker = Ticker {
            last_price: Decimal::ONE_THOUSAND,
            ..Default::default()
        };

        let pair = pairs::Model::default();

        let result = Indicators::default()
            .next_phase::<Core>()
            .next_phase()
            .evaluate_indicator_logic(&indicator, &ticker, &pair);

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod fn_compare_tests {
    use super::*;

    #[test]
    fn valid_gte() {
        assert!(Indicators::<Logic>::compare(
            OperationKeys::Gte,
            Decimal::ZERO,
            Decimal::ZERO,
        ))
    }

    #[test]
    fn invalid_gte() {
        assert!(!Indicators::<Logic>::compare(
            OperationKeys::Gte,
            Decimal::ZERO,
            Decimal::ONE
        ))
    }
}
