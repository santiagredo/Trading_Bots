use std::marker::PhantomData;

use models::structs::Environments;
use tokio_util::sync::CancellationToken;

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct Cancellations<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
}

impl<Phase> Cancellations<Phase> {
    pub fn next_phase<Next>(self) -> Cancellations<Next> {
        Cancellations {
            phase: PhantomData::<Next>,
            environment: self.environment,
        }
    }
}

impl Cancellations {
    pub fn new(environment: Environments) -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment,
        }
    }

    pub async fn get_runtime_token(self) -> Option<CancellationToken> {
        self.next_phase().get_runtime_token_core().await
    }

    pub async fn start_runtime(self) -> Result<CancellationToken, Response> {
        self.next_phase().start_runtime_core().await
    }

    pub async fn stop_runtime(self) {
        self.next_phase().stop_runtime_core().await
    }
}
