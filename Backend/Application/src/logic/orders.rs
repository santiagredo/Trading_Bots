use models::structs::request::OrderRequest;
use sea_orm::prelude::Decimal;

/* ======================================================
 * VALIDATIONS
 * ======================================================
 */

pub fn validate_insert(req: &OrderRequest) -> Result<(), String> {
    if req.status_id.is_none_or(|id| id <= 0) {
        return Err("Invalid status ID".into());
    }

    if req.strategy_id.is_none_or(|id| id <= 0) {
        return Err("Invalid strategy ID".into());
    }

    if req.base_asset_id.is_none_or(|id| id <= 0) {
        return Err("Invalid base asset ID".into());
    }

    if req.quote_asset_id.is_none_or(|id| id <= 0) {
        return Err("Invalid quote asset ID".into());
    }

    if req.price_target.is_some_and(|price| price < Decimal::ZERO) {
        return Err("Invalid target price: < 0".into());
    }

    let is_sell = match req.is_sell {
        Some(val) => val,
        None => return Err("Missing order buy/sell action".into()),
    };

    if req.base_asset_amount.unwrap_or_default() <= Decimal::ZERO
        && req.quote_asset_amount.unwrap_or_default() <= Decimal::ZERO
    {
        return Err(format!(
            "Invalid amounts -- base: {:?} -- quote: {:?}",
            req.base_asset_amount, req.quote_asset_amount
        ));
    }

    if is_sell {
        if req.price_abort.unwrap_or_default() != Decimal::ZERO
            && req.price_abort >= req.price_target
        {
            return Err("Invalid abort price".into());
        }

        if (req.price_target.unwrap_or_default() <= Decimal::ZERO
            && req.status_id.unwrap_or_default() == 1)
            || req.price_entry > req.price_target
        {
            return Err("Invalid sell price target".into());
        }
    } else {
        if req.price_abort.unwrap_or_default() != Decimal::ZERO
            && req.price_abort <= req.price_target
        {
            return Err("Invalid abort price".into());
        }

        if (req.price_target.unwrap_or_default() <= Decimal::ZERO
            && req.status_id.unwrap_or_default() == 1)
            || req.price_entry < req.price_target
        {
            return Err("Invalid buy price target".into());
        }
    }

    Ok(())
}

pub fn validate_select(req: &OrderRequest) -> Result<(), String> {
    if req.id.is_none_or(|id| id <= 0) {
        return Err("Invalid order ID".into());
    }

    Ok(())
}

pub fn validate_update(req: &OrderRequest) -> Result<(), String> {
    if req.id.is_none_or(|id| id <= 0) {
        return Err("Invalid order ID".into());
    }

    if ![2, 3].contains(&req.status_id.unwrap_or_default()) {
        return Err("Invalid status ID".into());
    }

    Ok(())
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
                "invalid status id",
                OrderRequest {
                    status_id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "missing buy/sell",
                OrderRequest {
                    status_id: Some(1),
                    strategy_id: Some(1),
                    base_asset_id: Some(1),
                    quote_asset_id: Some(1),
                    ..Default::default()
                },
                false,
            ),
            (
                "valid buy order",
                OrderRequest {
                    status_id: Some(1),
                    strategy_id: Some(1),
                    base_asset_id: Some(1),
                    quote_asset_id: Some(2),
                    is_sell: Some(false),
                    price_target: Some(Decimal::ONE),
                    price_entry: Some(Decimal::ONE),
                    base_asset_amount: Some(Decimal::ONE),
                    quote_asset_amount: Some(Decimal::ONE),
                    ..Default::default()
                },
                true,
            ),
        ];

        for (name, req, should_pass) in cases {
            let result = validate_insert(&req);
            assert_eq!(result.is_ok(), should_pass, "case `{}` failed", name);
        }
    }
}

#[cfg(test)]
mod fn_validate_select {
    use super::*;

    #[test]
    fn cases() {
        let cases = vec![
            (
                "invalid order id",
                OrderRequest {
                    id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "valid order id",
                OrderRequest {
                    id: Some(1),
                    ..Default::default()
                },
                true,
            ),
        ];

        for (name, req, should_pass) in cases {
            let result = validate_select(&req);
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
                "invalid order id",
                OrderRequest {
                    id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "invalid status id",
                OrderRequest {
                    id: Some(1),
                    status_id: Some(1),
                    ..Default::default()
                },
                false,
            ),
            (
                "valid status id 2",
                OrderRequest {
                    id: Some(1),
                    status_id: Some(2),
                    ..Default::default()
                },
                true,
            ),
            (
                "valid status id 3",
                OrderRequest {
                    id: Some(1),
                    status_id: Some(3),
                    ..Default::default()
                },
                true,
            ),
        ];

        for (name, req, should_pass) in cases {
            let result = validate_update(&req);
            assert_eq!(result.is_ok(), should_pass, "case `{}` failed", name);
        }
    }
}
