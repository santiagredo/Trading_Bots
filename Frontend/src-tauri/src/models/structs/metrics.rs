// use sea_orm::prelude::DateTime;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct CriticalMetric {
    pub id: i64,

    pub executions_ok: u64,
    pub executions_err: u64,

    pub total_execution_time: Duration,
    pub max_execution_time: Duration,

    pub active_posting: u64,
    pub max_active_posting: u64,

    pub skipped_due_to_lock: u64,

    pub last_success: Option<NaiveDateTime>,
    pub last_error: Option<NaiveDateTime>,

    pub consecutive_errors: u64,
    pub slowest_duration: Duration,
}
