use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct StrategyRequest {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub is_active: Option<bool>,
    pub can_trade: Option<bool>,
    pub description: Option<String>,
    pub last_execution: Option<DateTime>,
    pub cooldown: Option<i32>,
    pub error_cooldown: Option<i32>,
    pub error_last_date: Option<DateTime>,
    pub last_update: Option<DateTime>,
}
