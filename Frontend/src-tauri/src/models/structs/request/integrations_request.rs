use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct IntegrationRequest {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub is_enabled: Option<bool>,
    pub creation_date: Option<NaiveDateTime>,
    pub last_update_date: Option<NaiveDateTime>,
}
