use crate::utils::Response;
use models::{
    enums::LifecycleState,
    structs::{CacheRuntimes, Environments},
};

pub struct Runtimes;

impl Runtimes {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_runtimes_status(self) -> CacheRuntimes {
        self.get_runtimes_status_core().await
    }

    pub async fn set_runtime_status(
        self,
        environment: Environments,
        state: LifecycleState,
    ) -> Result<CacheRuntimes, Response> {
        self.set_runtime_status_core(environment, state).await
    }

    pub async fn reset_runtime(self, environment: Environments) -> Result<(), Response> {
        self.reset_runtime_core(environment).await
    }
}
