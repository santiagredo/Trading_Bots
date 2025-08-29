use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ActionRequest {
    pub id: Option<i32>,
    pub strategy_id: Option<i32>,
    pub is_active: Option<bool>,
    pub is_sell: Option<bool>,
    pub is_quote_asset: Option<bool>,
    pub is_percentage: Option<bool>,
    pub value: Option<Decimal>,
    pub pair_id: Option<i32>,
}
