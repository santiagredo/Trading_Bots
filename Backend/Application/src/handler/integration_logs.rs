use std::marker::PhantomData;

use models::{
    entities::integration_log::Model,
    structs::{Environments, IntegrationLogRequest, QueryOptions},
};

use crate::utils::{Response, Types};

#[derive(Debug, Clone)]
pub struct IntegrationLogs<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: IntegrationLogRequest,
    pub environment: Environments,
}

impl<Phase> IntegrationLogs<Phase> {
    pub fn next_phase<Next>(self) -> IntegrationLogs<Next> {
        IntegrationLogs {
            phase: PhantomData::<Next>,
            model: self.model,
            environment: self.environment,
        }
    }
}

impl IntegrationLogs {
    pub fn new(environment: &Environments, model: IntegrationLogRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
            environment: *environment,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: IntegrationLogRequest {
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
