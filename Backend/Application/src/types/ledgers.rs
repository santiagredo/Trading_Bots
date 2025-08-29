use std::marker::PhantomData;

use models::{entities::ledgers::Model, structs::LedgerRequest};

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct Ledgers<Phase = Types> {
    pub phase: PhantomData<Phase>,
    pub model: LedgerRequest,
}

impl Ledgers {
    pub fn new(model: LedgerRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: LedgerRequest {
                ..Default::default()
            },
        }
    }
}

impl<Phase> Ledgers<Phase> {
    pub fn next_phase<Next>(self) -> Ledgers<Next> {
        Ledgers {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl Ledgers<Types> {
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
