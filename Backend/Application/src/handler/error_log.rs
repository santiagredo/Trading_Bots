use std::marker::PhantomData;

use models::{entities::error_log::Model, structs::ErrorLogRequest};

use crate::utils::{Response, Types};

#[derive(Debug, Clone)]
pub struct ErrorLogs<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: ErrorLogRequest,
}

impl<Phase> ErrorLogs<Phase> {
    pub fn next_phase<Next>(self) -> ErrorLogs<Next> {
        ErrorLogs {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl ErrorLogs {
    pub fn new(model: ErrorLogRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: ErrorLogRequest {
                ..Default::default()
            },
        }
    }

    pub async fn insert_log(self) -> Result<Model, Response> {
        self.next_phase().insert_log_core().await
    }
}
