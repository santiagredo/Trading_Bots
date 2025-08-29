use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct IndicatorRequest {
    pub id: Option<i32>,
    pub strategy_id: Option<i32>,
    pub is_active: Option<bool>,
    pub symbol: Option<String>,
    pub nick: Option<String>,
    pub direction: Option<String>,
    pub is_percentage: Option<bool>,
    pub value: Option<Decimal>,
}
