use models::{
    entities::{indicators, pairs},
    enums::{IndicatorKeys, OperationKeys},
    structs::{IndicatorRequest, Ticker},
};
use sea_orm::prelude::Decimal;

use crate::utils::Utils;

/* ======================================================
 * VALIDATIONS
 * ======================================================
 */

pub fn validate_insert(req: &IndicatorRequest) -> Result<(), String> {
    if req.strategy_id.is_none_or(|id| id <= 0) {
        return Err("Invalid strategy ID".to_string());
    }

    Utils::validate_empty_field(req.symbol.clone().unwrap_or_default(), "Indicator symbol")?;

    if !IndicatorKeys::contains(&req.nick.clone().unwrap_or_default()) {
        return Err("Invalid indicator nick".to_string());
    }

    if !OperationKeys::contains(&req.direction.clone().unwrap_or_default()) {
        return Err("Invalid direction".to_string());
    }

    if req.value.is_none_or(|v| v == Decimal::ZERO) {
        return Err("Invalid indicator value".to_string());
    }

    Ok(())
}

pub fn validate_update(req: &IndicatorRequest) -> Result<(), String> {
    if req.id.is_none_or(|id| id <= 0) {
        return Err("Invalid indicator ID".to_string());
    }

    if req.strategy_id.is_none_or(|id| id <= 0) {
        return Err("Invalid strategy ID".to_string());
    }

    if let Some(symbol) = req.symbol.clone() {
        Utils::validate_empty_field(symbol, "Indicator symbol")?;
    }

    if matches!(req.nick.clone(), Some(ref n) if !IndicatorKeys::contains(n)) {
        return Err("Invalid indicator nick".to_string());
    }

    if matches!(req.direction.clone(), Some(ref d) if !OperationKeys::contains(d)) {
        return Err("Invalid direction".to_string());
    }

    if req.value.is_none_or(|v| v == Decimal::ZERO) {
        return Err("Invalid indicator value".to_string());
    }

    Ok(())
}

pub fn validate_delete(req: &IndicatorRequest) -> Result<(), String> {
    if req.id.is_none_or(|id| id <= 0) {
        return Err("Invalid indicator ID".to_string());
    }

    Ok(())
}

/* ======================================================
 * EVALUATION
 * ======================================================
 */

pub fn evaluate_indicator(
    indicator: &indicators::Model,
    ticker: &Ticker,
    pair: &pairs::Model,
) -> Result<bool, String> {
    let Some(indicator_key) = IndicatorKeys::from_str(&indicator.nick) else {
        return Err("Indicator key not found".to_owned());
    };

    let ticker_value = get_ticker_value(indicator_key, ticker, pair);
    if ticker_value == Decimal::ZERO {
        return Err("Ticker value not found".to_owned());
    }

    let Some(operation_key) = OperationKeys::from_str(&indicator.direction) else {
        return Err("Operation key not found".to_owned());
    };

    Ok(compare(operation_key, ticker_value, indicator.value))
}

pub fn compare(op: OperationKeys, left: Decimal, right: Decimal) -> bool {
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

#[cfg(test)]
mod fn_validate_insert {
    use super::*;

    #[test]
    fn cases() {
        let cases = vec![
            (
                "ok",
                IndicatorRequest {
                    strategy_id: Some(1),
                    symbol: Some("BTCUSDT".into()),
                    nick: Some("lst".into()),
                    direction: Some("gte".into()),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_strategy",
                IndicatorRequest {
                    strategy_id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_symbol",
                IndicatorRequest {
                    strategy_id: Some(1),
                    symbol: Some("".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_nick",
                IndicatorRequest {
                    strategy_id: Some(1),
                    symbol: Some("BTCUSDT".into()),
                    nick: Some("abc".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_direction",
                IndicatorRequest {
                    strategy_id: Some(1),
                    symbol: Some("BTCUSDT".into()),
                    nick: Some("lst".into()),
                    direction: Some("abc".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_value",
                IndicatorRequest {
                    strategy_id: Some(1),
                    symbol: Some("BTCUSDT".into()),
                    nick: Some("lst".into()),
                    direction: Some("gte".into()),
                    value: Some(Decimal::ZERO),
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, ok) in cases {
            assert_eq!(validate_insert(&req).is_ok(), ok, "case `{}` failed", name);
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
                IndicatorRequest {
                    id: Some(1),
                    strategy_id: Some(1),
                    symbol: Some("BTCUSDT".into()),
                    nick: Some("lst".into()),
                    direction: Some("gte".into()),
                    value: Some(Decimal::ONE),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_id",
                IndicatorRequest {
                    id: Some(0),
                    strategy_id: Some(1),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_strategy",
                IndicatorRequest {
                    id: Some(1),
                    strategy_id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_symbol",
                IndicatorRequest {
                    id: Some(1),
                    strategy_id: Some(1),
                    symbol: Some("".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_nick",
                IndicatorRequest {
                    id: Some(1),
                    strategy_id: Some(1),
                    nick: Some("abc".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_direction",
                IndicatorRequest {
                    id: Some(1),
                    strategy_id: Some(1),
                    nick: Some("lst".into()),
                    direction: Some("abc".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_value",
                IndicatorRequest {
                    id: Some(1),
                    strategy_id: Some(1),
                    value: Some(Decimal::ZERO),
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, ok) in cases {
            assert_eq!(validate_update(&req).is_ok(), ok, "case `{}` failed", name);
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
                IndicatorRequest {
                    id: Some(1),
                    ..Default::default()
                },
                true,
            ),
            (
                "err",
                IndicatorRequest {
                    id: Some(0),
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, ok) in cases {
            assert_eq!(validate_delete(&req).is_ok(), ok, "case `{}` failed", name);
        }
    }
}

#[cfg(test)]
mod fn_evaluate_indicator {
    use super::*;

    #[test]
    fn ok_valid() {
        let indicator = indicators::Model {
            nick: "lst".into(),
            direction: "gte".into(),
            value: Decimal::ONE,
            ..Default::default()
        };

        let ticker = Ticker {
            last_price: Decimal::ONE_THOUSAND,
            ..Default::default()
        };

        let pair = pairs::Model::default();

        assert!(evaluate_indicator(&indicator, &ticker, &pair).unwrap());
    }

    #[test]
    fn ok_invalid() {
        let indicator = indicators::Model {
            nick: "lst".into(),
            direction: "lte".into(),
            value: Decimal::ONE,
            ..Default::default()
        };

        let ticker = Ticker {
            last_price: Decimal::ONE_THOUSAND,
            ..Default::default()
        };

        let pair = pairs::Model::default();

        let result = evaluate_indicator(&indicator, &ticker, &pair).unwrap();

        assert!(!result);
    }

    #[test]
    fn err_nick() {
        let indicator = indicators::Model {
            nick: "abc".into(),
            ..Default::default()
        };

        let ticker = Ticker::default();
        let pair = pairs::Model::default();

        assert!(evaluate_indicator(&indicator, &ticker, &pair).is_err());
    }

    #[test]
    fn err_direction() {
        let indicator = indicators::Model {
            nick: "lst".into(),
            direction: "abc".into(),
            ..Default::default()
        };

        let ticker = Ticker::default();
        let pair = pairs::Model::default();

        assert!(evaluate_indicator(&indicator, &ticker, &pair).is_err());
    }
}

#[cfg(test)]
mod fn_compare {
    use super::*;

    #[test]
    fn valid_gte() {
        assert!(compare(OperationKeys::Gte, Decimal::ONE, Decimal::ONE));
    }

    #[test]
    fn invalid_gte() {
        assert!(!compare(OperationKeys::Gte, Decimal::ZERO, Decimal::ONE));
    }
}
