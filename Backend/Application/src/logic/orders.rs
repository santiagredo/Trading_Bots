use models::structs::request::OrderRequest;
use sea_orm::prelude::Decimal;

use crate::{types::Orders, utils::Logic};

impl Orders<Logic> {
    pub fn insert_order_logic(mut self) -> Result<Self, String> {
        if self.model.status_id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid status ID"));
        }

        if self.model.strategy_id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid strategy ID"));
        }

        if self.model.base_asset_id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid base asset ID"));
        }

        if self.model.quote_asset_id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid quote asset ID"));
        }

        if self
            .model
            .price_target
            .is_some_and(|price| price < Decimal::ZERO)
        {
            return Err(format!("Invalid target price: < 0"));
        }

        let is_sell = match self.model.is_sell {
            None => return Err(format!("Missing order buy/sell action")),
            Some(val) => val,
        };

        if self.model.base_asset_amount.unwrap_or_default() <= Decimal::ZERO
            && self.model.quote_asset_amount.unwrap_or_default() <= Decimal::ZERO
        {
            return Err(format!(
                "Invalid amounts -- base: {:?} -- quote: {:?}",
                self.model.base_asset_amount, self.model.quote_asset_amount
            ));
        }

        if self.model.base_asset_amount.unwrap_or_default() == Decimal::ZERO {
            self.model.base_asset_amount = Some(
                self.model.quote_asset_amount.unwrap_or_default()
                    / self.model.price_target.unwrap_or_default(),
            );
        }

        if self.model.quote_asset_amount.unwrap_or_default() == Decimal::ZERO {
            self.model.quote_asset_amount = Some(
                self.model.base_asset_amount.unwrap_or_default()
                    * self.model.price_target.unwrap_or_default(),
            );
        }

        match is_sell {
            true => {
                if self.model.price_abort.unwrap_or_default() != Decimal::ZERO
                    && self.model.price_abort >= self.model.price_target
                {
                    return Err(format!("Invalid abort price"));
                }

                if (self.model.price_target.unwrap_or_default() <= Decimal::ZERO
                    && self.model.status_id.unwrap_or_default() == 1)
                    || self.model.price_entry > self.model.price_target
                {
                    return Err(format!("Invalid sell price target"));
                }
            }
            false => {
                if self.model.price_abort.unwrap_or_default() != Decimal::ZERO
                    && self.model.price_abort <= self.model.price_target
                {
                    return Err(format!("Invalid abort price"));
                }

                if (self.model.price_target.unwrap_or_default() <= Decimal::ZERO
                    && self.model.status_id.unwrap_or_default() == 1)
                    || self.model.price_entry < self.model.price_target
                {
                    return Err(format!("Invalid buy price target"));
                }
            }
        }

        Ok(self)
    }

    pub fn select_order_logic(self) -> Result<Self, String> {
        if self.model.id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid order ID"));
        }

        Ok(self)
    }

    pub fn update_order_logic(self) -> Result<Self, String> {
        if self.model.id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid order ID"));
        }

        if ![2, 3].contains(&self.model.status_id.unwrap_or_default()) {
            return Err(format!("Invalid status ID"));
        }

        Ok(self)
    }

    pub fn create_order_request() -> OrderRequest {
        todo!()
    }
}

#[cfg(test)]
mod insert_order_logic_tests {
    use crate::types::Orders;
    use models::structs::request::OrderRequest;
    use sea_orm::prelude::Decimal;

    #[test]
    fn test_insert_order_logic_cases() {
        let cases = vec![
            (
                "invalid status id",
                OrderRequest {
                    status_id: Some(0),
                    ..Default::default()
                },
                Err("Invalid status ID".to_string()),
            ),
            (
                "invalid strategy id",
                OrderRequest {
                    status_id: Some(1),
                    strategy_id: Some(0),
                    ..Default::default()
                },
                Err("Invalid strategy ID".to_string()),
            ),
            (
                "invalid base asset id",
                OrderRequest {
                    status_id: Some(1),
                    strategy_id: Some(1),
                    base_asset_id: Some(0),
                    ..Default::default()
                },
                Err("Invalid base asset ID".to_string()),
            ),
            (
                "invalid quote asset id",
                OrderRequest {
                    status_id: Some(1),
                    strategy_id: Some(1),
                    base_asset_id: Some(1),
                    quote_asset_id: Some(0),
                    ..Default::default()
                },
                Err("Invalid quote asset ID".to_string()),
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
                Err("Missing order buy/sell action".to_string()),
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
                Ok(()),
            ),
        ];

        for (name, req, expected) in cases {
            let result = Orders::new(req)
                .next_phase()
                .insert_order_logic()
                .map(|_| ());

            assert_eq!(result, expected, "failed case: {}", name);
        }
    }
}

#[cfg(test)]
mod select_order_logic_tests {
    use crate::types::Orders;
    use models::structs::request::OrderRequest;

    #[test]
    fn test_select_order_logic_cases() {
        let cases = vec![
            (
                "invalid order id",
                OrderRequest {
                    id: Some(0),
                    ..Default::default()
                },
                Err("Invalid order ID".to_string()),
            ),
            (
                "valid order id",
                OrderRequest {
                    id: Some(1),
                    ..Default::default()
                },
                Ok(()),
            ),
        ];

        for (name, req, expected) in cases {
            let result = Orders::new(req)
                .next_phase()
                .select_order_logic()
                .map(|_| ());

            assert_eq!(result, expected, "failed case: {}", name);
        }
    }
}

#[cfg(test)]
mod update_order_logic_tests {
    use crate::types::Orders;
    use models::structs::request::OrderRequest;

    #[test]
    fn test_update_order_logic_cases() {
        let cases = vec![
            (
                "invalid order id",
                OrderRequest {
                    id: Some(0),
                    ..Default::default()
                },
                Err("Invalid order ID".to_string()),
            ),
            (
                "invalid status id",
                OrderRequest {
                    id: Some(1),
                    status_id: Some(1),
                    ..Default::default()
                },
                Err("Invalid status ID".to_string()),
            ),
            (
                "valid status id 2",
                OrderRequest {
                    id: Some(1),
                    status_id: Some(2),
                    ..Default::default()
                },
                Ok(()),
            ),
            (
                "valid status id 3",
                OrderRequest {
                    id: Some(1),
                    status_id: Some(3),
                    ..Default::default()
                },
                Ok(()),
            ),
        ];

        for (name, req, expected) in cases {
            let result = Orders::new(req)
                .next_phase()
                .update_order_logic()
                .map(|_| ());

            assert_eq!(result, expected, "failed case: {}", name);
        }
    }
}
