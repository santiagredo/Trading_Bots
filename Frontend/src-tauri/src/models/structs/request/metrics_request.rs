use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MetricRequest {
    pub id: Option<i32>,
    pub creation_date: Option<NaiveDateTime>,
    pub executions_ok: Option<i64>,
    pub executions_err: Option<i64>,
    pub total_execution_time: Option<i64>,
    pub max_execution_time: Option<i64>,
    pub slowest_duration: Option<i64>,
    pub active_posting: Option<i64>,
    pub max_active_posting: Option<i64>,
    pub skipped_due_to_lock: Option<i64>,
    pub last_success: Option<NaiveDateTime>,
    pub last_error: Option<NaiveDateTime>,
    pub consecutive_errors: Option<i64>,
}
