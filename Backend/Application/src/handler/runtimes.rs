use std::marker::PhantomData;

use models::{
    enums::LifecycleState,
    structs::{CacheRuntimes, Environments},
};

use crate::utils::{Response, Types};

pub struct Runtimes<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: CacheRuntimes,
}

impl<Phase> Runtimes<Phase> {
    pub fn next_phase<Next>(self) -> Runtimes<Next> {
        Runtimes {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl Runtimes {
    pub fn new(environment: Environments, state: LifecycleState) -> Self {
        let mut cache_runtimes = CacheRuntimes::default();

        match environment {
            Environments::PROD => cache_runtimes.prod.status = state,
            Environments::DEV => cache_runtimes.dev.status = state,
        };

        Self {
            phase: PhantomData::<Types>,
            model: cache_runtimes,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: CacheRuntimes::default(),
        }
    }

    pub async fn get_runtimes_status(self) -> CacheRuntimes {
        self.next_phase().get_runtimes_status_core().await
    }

    pub async fn set_runtime_status(
        self,
        environment: Environments,
    ) -> Result<CacheRuntimes, Response> {
        self.next_phase().set_runtime_status_core(environment).await
    }

    pub async fn reset_runtime(self, environment: Environments) -> Result<(), Response> {
        self.next_phase().reset_runtime_core(environment).await
    }
}
