use std::marker::PhantomData;

use models::{
    entities::ledgers::Model,
    structs::{Environments, LedgerRequest},
};

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct Ledgers<Phase = Types> {
    pub phase: PhantomData<Phase>,
    pub model: LedgerRequest,
    pub environment: Environments,
}

impl<Phase> Ledgers<Phase> {
    pub fn next_phase<Next>(self) -> Ledgers<Next> {
        Ledgers {
            phase: PhantomData::<Next>,
            model: self.model,
            environment: self.environment,
        }
    }
}

impl Ledgers {
    pub fn new(model: LedgerRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: LedgerRequest {
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

    pub async fn insert_ledger(self) -> Result<Model, Response> {
        self.next_phase().insert_ledger_core().await
    }

    pub async fn select_ledger(self) -> Result<Option<Model>, Response> {
        self.next_phase().select_ledger_core().await
    }

    pub async fn select_ledgers(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_ledgers_core().await
    }
}
