use std::marker::PhantomData;

use models::{
    entities::error_log::Model,
    structs::{Environments, ErrorLogRequest, QueryOptions},
};

use crate::utils::{Response, Types};

#[derive(Debug, Clone)]
pub struct ErrorLogs<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
    pub model: ErrorLogRequest,
}

impl<Phase> ErrorLogs<Phase> {
    pub fn next_phase<Next>(self) -> ErrorLogs<Next> {
        ErrorLogs {
            phase: PhantomData::<Next>,
            model: self.model,
            environment: self.environment,
        }
    }
}

impl ErrorLogs {
    pub fn new(environment: &Environments, model: ErrorLogRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
            environment: *environment,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: ErrorLogRequest {
                ..Default::default()
            },
            environment: Environments::DEV,
        }
    }

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: self.phase,
            environment,
            model: self.model,
        }
    }

    pub async fn insert_log(self) -> Result<Model, Response> {
        self.next_phase().insert_log_core().await
    }

    pub async fn select_logs(self, query: Option<QueryOptions>) -> Result<Vec<Model>, Response> {
        self.next_phase().select_logs_core(query).await
    }
}
