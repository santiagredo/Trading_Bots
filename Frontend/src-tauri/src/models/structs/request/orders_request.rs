use chrono::NaiveDateTime;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct OrderRequest {
    pub id: Option<i32>,
    pub status_id: Option<i32>,
    pub creation_date: Option<NaiveDateTime>,
    pub update_date: Option<NaiveDateTime>,
    pub is_sell: Option<bool>,
    pub strategy_id: Option<i32>,
    pub base_asset_id: Option<i32>,
    pub base_asset_amount: Option<Decimal>,
    pub quote_asset_id: Option<i32>,
    pub quote_asset_amount: Option<Decimal>,
    pub price_entry: Option<Decimal>,
    pub price_target: Option<Decimal>,
    pub price_abort: Option<Decimal>,
}
