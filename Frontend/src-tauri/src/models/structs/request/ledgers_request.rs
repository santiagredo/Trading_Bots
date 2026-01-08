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
