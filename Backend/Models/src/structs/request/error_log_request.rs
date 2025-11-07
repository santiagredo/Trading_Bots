use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ErrorLogRequest {
    pub id: Option<i32>,
    pub creation_date: Option<NaiveDateTime>,
    pub file_path: Option<String>,
    pub line_number: Option<String>,
    pub function_name: Option<String>,
    pub request: Option<String>,
    pub error_type: Option<String>,
    pub error_details: Option<String>,
}
