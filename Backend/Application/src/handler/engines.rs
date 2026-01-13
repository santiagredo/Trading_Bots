use std::marker::PhantomData;

use models::{enums::LifecycleState, structs::CacheEngine};
use tokio_util::sync::CancellationToken;

use crate::utils::{Response, Types};

pub struct Engines<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: LifecycleState,
}

impl<Phase> Engines<Phase> {
    pub fn next_phase<Next>(self) -> Engines<Next> {
        Engines {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl Engines {
    pub fn new(model: LifecycleState) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: LifecycleState::Off,
        }
    }

    pub async fn get_engine_status(self) -> CacheEngine {
        self.next_phase().get_engine_status_core().await
    }

    pub async fn set_engine_status(self) -> Result<CacheEngine, Response> {
        self.next_phase().set_engine_status_core().await
    }

    pub fn get_engine_token(self) -> CancellationToken {
        self.next_phase().get_engine_token_core()
    }

    pub async fn stop_engine(self) {
        self.next_phase().stop_engine_core().await;
    }
}
