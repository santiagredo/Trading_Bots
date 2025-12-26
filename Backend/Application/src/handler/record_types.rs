use std::marker::PhantomData;

use models::{
    entities::record_types::Model,
    structs::{Environments, RecordTypeRequest},
};

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct RecordTypes<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
    pub model: RecordTypeRequest,
}

impl<Phase> RecordTypes<Phase> {
    pub fn next_phase<Next>(self) -> RecordTypes<Next> {
        RecordTypes {
            phase: PhantomData::<Next>,
            environment: self.environment,
            model: self.model,
        }
    }
}

impl RecordTypes {
    pub fn new(model: RecordTypeRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model,
        }
    }

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: self.phase,
            environment,
            model: self.model,
        }
    }

    pub async fn select_record_types(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_record_types_core().await
    }
}
