use std::marker::PhantomData;

use models::{entities::integration_log::Model, structs::IntegrationLogRequest};

use crate::utils::{Response, Types};

#[derive(Debug, Clone)]
pub struct IntegrationLogs<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: IntegrationLogRequest,
}

impl<Phase> IntegrationLogs<Phase> {
    pub fn next_phase<Next>(self) -> IntegrationLogs<Next> {
        IntegrationLogs {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl IntegrationLogs {
    pub fn new(model: IntegrationLogRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: IntegrationLogRequest {
                ..Default::default()
            },
        }
    }

    pub async fn insert_log(self) -> Result<Model, Response> {
        self.next_phase().insert_log_core().await
    }
}
