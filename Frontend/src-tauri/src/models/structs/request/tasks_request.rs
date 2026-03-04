use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskRequest {
    pub id: Option<i32>,
    pub nick: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub cooldown: Option<i64>,
    pub delay: Option<i64>,
    pub last_update: Option<NaiveDateTime>,
    pub last_execution: Option<NaiveDateTime>,
}
