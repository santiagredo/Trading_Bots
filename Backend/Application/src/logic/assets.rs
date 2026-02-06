use models::structs::AssetRequest;

use crate::utils::Utils;

pub fn validate_insert(req: &AssetRequest) -> Result<(), String> {
    let _ = Utils::validate_empty_field(req.name.clone().unwrap_or_default(), "Asset name")?
        .to_uppercase();

    let _ = Utils::validate_empty_field(req.ticker.clone().unwrap_or_default(), "Asset ticker")?
        .to_uppercase();

    Ok(())
}

pub fn validate_update(req: &AssetRequest) -> Result<(), String> {
    if req.id.is_none_or(|id| id <= 0) {
        return Err(format!("Invalid ID: {:?}", req.id));
    }

    let _ = Utils::validate_empty_field(req.name.clone().unwrap_or_default(), "Asset name")?
        .to_uppercase();

    let _ = Utils::validate_empty_field(req.ticker.clone().unwrap_or_default(), "Asset ticker")?
        .to_uppercase();

    Ok(())
}

pub fn validate_delete(req: &AssetRequest) -> Result<(), String> {
    if req.id.is_none_or(|id| id <= 0) {
        return Err(format!("Invalid ID: {:?}", req.id));
    }

    Ok(())
}

#[cfg(test)]
mod fn_validate_insert {
    use super::*;
    use models::structs::AssetRequest;

    #[test]
    fn validate_insert_cases() {
        let cases = vec![
            (
                "ok_valid_insert",
                AssetRequest {
                    name: Some("Bitcoin".into()),
                    ticker: Some("btc".into()),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_missing_name",
                AssetRequest {
                    name: None,
                    ticker: Some("BTC".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_empty_name",
                AssetRequest {
                    name: Some("".into()),
                    ticker: Some("BTC".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_ticker",
                AssetRequest {
                    name: Some("Ethereum".into()),
                    ticker: None,
                    ..Default::default()
                },
                false,
            ),
            (
                "err_empty_ticker",
                AssetRequest {
                    name: Some("Ethereum".into()),
                    ticker: Some("".into()),
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let result = validate_insert(&req);

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
mod fn_validate_update {
    use super::*;
    use models::structs::AssetRequest;

    #[test]
    fn validate_update_cases() {
        let cases = vec![
            (
                "ok_valid_update",
                AssetRequest {
                    id: Some(1),
                    name: Some("Cardano".into()),
                    ticker: Some("ada".into()),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_invalid_id_zero",
                AssetRequest {
                    id: Some(0),
                    name: Some("Litecoin".into()),
                    ticker: Some("ltc".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_id_negative",
                AssetRequest {
                    id: Some(-1),
                    name: Some("Litecoin".into()),
                    ticker: Some("ltc".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_id",
                AssetRequest {
                    id: None,
                    name: Some("XRP".into()),
                    ticker: Some("xrp".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_name",
                AssetRequest {
                    id: Some(1),
                    name: None,
                    ticker: Some("xrp".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_ticker",
                AssetRequest {
                    id: Some(1),
                    name: Some("XRP".into()),
                    ticker: None,
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let result = validate_update(&req);

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
mod fn_validate_delete {
    use super::*;
    use models::structs::AssetRequest;

    #[test]
    fn validate_delete_cases() {
        let cases = vec![
            (
                "ok_valid_delete",
                AssetRequest {
                    id: Some(10),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_invalid_id_zero",
                AssetRequest {
                    id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_id_negative",
                AssetRequest {
                    id: Some(-5),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_id",
                AssetRequest {
                    id: None,
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let result = validate_delete(&req);

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
