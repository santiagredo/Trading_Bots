use sea_orm::prelude::{DateTime, Decimal};
use serde::{Deserialize, Serialize};

use crate::models::entities::{assets, orders};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LedgerRequest {
    pub id: Option<i32>,
    pub order_id: Option<i32>,
    pub record_type_id: Option<i32>,
    pub creation_date: Option<DateTime>,
    pub asset_id: Option<i32>,
    pub free_amount: Option<Decimal>,
    pub free_previous_balance: Option<Decimal>,
    pub free_new_balance: Option<Decimal>,
    pub locked_amount: Option<Decimal>,
    pub locked_previous_balance: Option<Decimal>,
    pub locked_new_balance: Option<Decimal>,
}

impl LedgerRequest {
    pub fn from_asset(asset: &assets::Model) -> LedgerRequest {
        LedgerRequest {
            id: None,
            order_id: None,
            record_type_id: None,
            creation_date: None,
            asset_id: Some(asset.id),
            free_amount: Some(asset.free),
            free_previous_balance: Some(asset.free),
            free_new_balance: Some(asset.free),
            locked_amount: Some(asset.locked),
            locked_previous_balance: Some(asset.locked),
            locked_new_balance: Some(asset.locked),
        }
    }

    pub fn from_order(mut self, order: &orders::Model) -> Self {
        let record_type_id = if order.is_sell { 2 } else { 1 };

        self.order_id = Some(order.id);
        self.record_type_id = Some(record_type_id);

        self
    }

    pub fn update_values(
        mut self,
        is_locked: bool,
        amount: Decimal,
        previous_balance: Decimal,
        new_balance: Decimal,
    ) -> Self {
        if is_locked {
            self.locked_amount = Some(amount);
            self.locked_previous_balance = Some(previous_balance);
            self.locked_new_balance = Some(new_balance);
        } else {
            self.free_amount = Some(amount);
            self.free_previous_balance = Some(previous_balance);
            self.free_new_balance = Some(new_balance);
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::prelude::Decimal;

    fn mock_asset(id: i32, free: i64, locked: i64) -> assets::Model {
        assets::Model {
            id,
            free: Decimal::new(free, 0),
            locked: Decimal::new(locked, 0),
            ..Default::default()
        }
    }

    fn mock_order(id: i32, is_sell: bool) -> orders::Model {
        orders::Model {
            id,
            is_sell,
            ..Default::default()
        }
    }

    #[test]
    fn ledger_request_from_asset_order_and_update_free_balance() {
        // Arrange
        let asset = mock_asset(1, 125, 50);
        let order = mock_order(10, false);

        let previous_balance = Decimal::new(100, 0);
        let amount = Decimal::new(25, 0);
        let new_balance = Decimal::new(125, 0);

        // Act
        let ledger = LedgerRequest::from_asset(&asset)
            .from_order(&order)
            .update_values(
                false, // is_locked = false -> free
                amount,
                previous_balance,
                new_balance,
            );

        // Assert - IDs
        assert_eq!(ledger.asset_id, Some(1));
        assert_eq!(ledger.order_id, Some(10));
        assert_eq!(ledger.record_type_id, Some(1)); // buy

        // Assert - Free balance
        assert_eq!(ledger.free_amount, Some(amount));
        assert_eq!(ledger.free_previous_balance, Some(previous_balance));
        assert_eq!(ledger.free_new_balance, Some(new_balance));

        // Assert - Locked untouched
        assert_eq!(ledger.locked_amount, Some(Decimal::new(50, 0)));
        assert_eq!(ledger.locked_previous_balance, Some(Decimal::new(50, 0)));
        assert_eq!(ledger.locked_new_balance, Some(Decimal::new(50, 0)));
    }
}
