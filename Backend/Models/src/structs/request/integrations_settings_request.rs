use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct IntegrationSettingRequest {
    pub id: Option<i32>,
    pub integration_id: Option<i32>,
    pub name: Option<String>,
    pub nick: Option<String>,
    pub value: Option<String>,
    pub creation_date: Option<NaiveDateTime>,
    pub last_update_date: Option<NaiveDateTime>,
}
