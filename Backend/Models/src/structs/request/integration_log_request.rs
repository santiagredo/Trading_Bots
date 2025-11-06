use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct IntegrationLogRequest {
    pub id: Option<i32>,
    pub creation_date: Option<NaiveDateTime>,
    pub integration_name: Option<String>,
    pub function_name: Option<String>,
    pub url: Option<String>,
    pub request: Option<String>,
    pub response: Option<String>,
    pub status_code: Option<i32>,
    pub error_message: Option<String>,
    pub execution_time_ms: Option<i32>,
}
