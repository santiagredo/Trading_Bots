use crate::{handler::Ledgers, utils::Logic};

impl Ledgers<Logic> {
    pub fn insert_ledger_logic(self) -> Result<Self, String> {
        if self.model.record_type_id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid record type ID"));
        }

        if self.model.asset_id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid asset ID"));
        }

        if self.model.free_amount.is_none() {
            return Err(format!("Invalid free asset amount"));
        }

        if self.model.free_previous_balance.is_none() {
            return Err(format!("Invalid free asset previous balance"));
        }

        if self.model.free_new_balance.is_none() {
            return Err(format!("Invalid free asset new balance"));
        }

        if self.model.locked_amount.is_none() {
            return Err(format!("Invalid locked asset amount"));
        }

        if self.model.locked_previous_balance.is_none() {
            return Err(format!("Invalid locked asset previous balance"));
        }

        if self.model.locked_new_balance.is_none() {
            return Err(format!("Invalid locked asset new balance"));
        }

        Ok(self)
    }
}

#[cfg(test)]
mod fn_insert_ledger_logic {
    use models::structs::LedgerRequest;
    use sea_orm::prelude::Decimal;

    use crate::handler::Ledgers;
    use crate::utils::Logic;

    #[test]
    fn test_insert_ledger_logic() {
        let test_cases = vec![
            (
                "valid insert",
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
                "invalid record type id",
                LedgerRequest {
                    record_type_id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "invalid asset id",
                LedgerRequest {
                    record_type_id: Some(1),
                    asset_id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "invalid free asset amount",
                LedgerRequest {
                    record_type_id: Some(1),
                    asset_id: Some(1),
                    free_amount: None,
                    ..Default::default()
                },
                false,
            ),
            (
                "invalid locked asset amount",
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

        for (name, input, expected) in test_cases {
            let ledger = Ledgers::new(input).next_phase::<Logic>();
            let result = ledger.insert_ledger_logic().is_ok();
            assert_eq!(result, expected, "{}", name);
        }
    }
}
