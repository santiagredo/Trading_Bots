use models::structs::LedgerRequest;

/* ======================================================
 * VALIDATIONS
 * ======================================================
 */

pub fn validate_insert(req: &LedgerRequest) -> Result<(), String> {
    if req.record_type_id.is_none_or(|id| id <= 0) {
        return Err("Invalid record type ID".into());
    }

    if req.asset_id.is_none_or(|id| id <= 0) {
        return Err("Invalid asset ID".into());
    }

    if req.free_amount.is_none() {
        return Err("Invalid free asset amount".into());
    }

    if req.free_previous_balance.is_none() {
        return Err("Invalid free asset previous balance".into());
    }

    if req.free_new_balance.is_none() {
        return Err("Invalid free asset new balance".into());
    }

    if req.locked_amount.is_none() {
        return Err("Invalid locked asset amount".into());
    }

    if req.locked_previous_balance.is_none() {
        return Err("Invalid locked asset previous balance".into());
    }

    if req.locked_new_balance.is_none() {
        return Err("Invalid locked asset new balance".into());
    }

    Ok(())
}

/* ======================================================
 * TESTS
 * ======================================================
 */

#[cfg(test)]
mod fn_validate_insert {
    use sea_orm::prelude::Decimal;

    use super::*;

    #[test]
    fn cases() {
        let cases = vec![
            (
                "ok",
                LedgerRequest {
                    record_type_id: Some(1),
                    asset_id: Some(1),
                    free_amount: Some(Decimal::ONE),
                    free_previous_balance: Some(Decimal::ZERO),
                    free_new_balance: Some(Decimal::ZERO),
                    locked_amount: Some(Decimal::ZERO),
                    locked_previous_balance: Some(Decimal::ZERO),
                    locked_new_balance: Some(Decimal::ZERO),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_invalid_record_type_id",
                LedgerRequest {
                    record_type_id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_invalid_asset_id",
                LedgerRequest {
                    record_type_id: Some(1),
                    asset_id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_free_amount",
                LedgerRequest {
                    record_type_id: Some(1),
                    asset_id: Some(1),
                    free_amount: None,
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_locked_amount",
                LedgerRequest {
                    record_type_id: Some(1),
                    asset_id: Some(1),
                    free_amount: Some(Decimal::ONE),
                    free_previous_balance: Some(Decimal::ONE),
                    free_new_balance: Some(Decimal::ONE),
                    locked_amount: None,
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
