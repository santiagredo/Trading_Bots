use models::structs::PairRequest;

pub fn validate_insert(req: &PairRequest) -> Result<(), String> {
    if req.base_asset_id.is_none_or(|id| id <= 0) {
        return Err(format!("Invalid base asset id: {:?}", req.base_asset_id));
    }

    if req.quote_asset_id.is_none_or(|id| id <= 0) {
        return Err(format!("Invalid quote asset id: {:?}", req.quote_asset_id));
    }

    if req.symbol.as_ref().is_none_or(|val| val.is_empty()) {
        return Err(format!("Invalid symbol: {:?}", req.symbol));
    }

    Ok(())
}

pub fn validate_update(req: &PairRequest) -> Result<(), String> {
    if req.base_asset_id.is_some_and(|id| id <= 0) {
        return Err(format!("Invalid base asset id: {:?}", req.base_asset_id));
    }

    if req.quote_asset_id.is_some_and(|id| id <= 0) {
        return Err(format!("Invalid quote asset id: {:?}", req.quote_asset_id));
    }

    if req.symbol.as_ref().is_some_and(|val| val.is_empty()) {
        return Err(format!("Invalid symbol: {:?}", req.symbol));
    }

    Ok(())
}

#[cfg(test)]
mod fn_validate_insert {
    use super::*;
    use models::structs::PairRequest;

    #[test]
    fn cases() {
        let cases = vec![
            (
                "ok",
                PairRequest {
                    base_asset_id: Some(1),
                    quote_asset_id: Some(2),
                    symbol: Some("BTCUSDT".into()),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_invalid_base_asset_id",
                PairRequest {
                    base_asset_id: Some(0),
                    quote_asset_id: Some(2),
                    symbol: Some("BTCUSDT".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_quote_asset_id",
                PairRequest {
                    base_asset_id: Some(1),
                    quote_asset_id: Some(0),
                    symbol: Some("BTCUSDT".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_symbol",
                PairRequest {
                    base_asset_id: Some(1),
                    quote_asset_id: Some(2),
                    symbol: None,
                    ..Default::default()
                },
                false,
            ),
            (
                "err_empty_symbol",
                PairRequest {
                    base_asset_id: Some(1),
                    quote_asset_id: Some(2),
                    symbol: Some("".into()),
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let result = validate_insert(&req);
            assert_eq!(result.is_ok(), should_pass, "case `{}` failed", name);
        }
    }
}

#[cfg(test)]
mod fn_validate_update {
    use super::*;
    use models::structs::PairRequest;

    #[test]
    fn cases() {
        let cases = vec![
            (
                "ok_all_none",
                PairRequest {
                    base_asset_id: None,
                    quote_asset_id: None,
                    symbol: None,
                    ..Default::default()
                },
                true,
            ),
            (
                "ok_partial_update",
                PairRequest {
                    base_asset_id: Some(1),
                    quote_asset_id: None,
                    symbol: Some("ETHUSDT".into()),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_invalid_base_asset_id",
                PairRequest {
                    base_asset_id: Some(0),
                    quote_asset_id: None,
                    symbol: None,
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_quote_asset_id",
                PairRequest {
                    base_asset_id: None,
                    quote_asset_id: Some(0),
                    symbol: None,
                    ..Default::default()
                },
                false,
            ),
            (
                "err_empty_symbol",
                PairRequest {
                    base_asset_id: None,
                    quote_asset_id: None,
                    symbol: Some("".into()),
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
