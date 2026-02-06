use models::{entities::critical_metrics::Model, structs::MetricRequest};

#[derive(Debug, Clone)]
pub struct Metrics<R> {
    pub repo: R,
}

impl<R> Metrics<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl Metrics<()> {
    pub fn blank() -> Metrics<()> {
        Self { repo: () }
    }

    pub fn into_request(model: Model) -> MetricRequest {
        MetricRequest {
            id: None,
            creation_date: None,
            executions_ok: Some(model.executions_ok),
            executions_err: Some(model.executions_err),
            total_execution_time: Some(model.total_execution_time),
            max_execution_time: Some(model.max_execution_time),
            slowest_duration: Some(model.slowest_duration),
            active_posting: Some(model.active_posting),
            max_active_posting: Some(model.max_active_posting),
            skipped_due_to_lock: Some(model.skipped_due_to_lock),
            last_success: model.last_success,
            last_error: model.last_error,
            consecutive_errors: Some(model.consecutive_errors),
        }
    }
}
