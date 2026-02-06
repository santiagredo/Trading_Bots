use crate::{handler::Runtimes, utils::Response};
use models::{
    enums::LifecycleState,
    structs::{CacheRuntimes, Environments},
};

impl Runtimes {
    pub async fn get_runtimes_status_core(self) -> CacheRuntimes {
        self.get_runtimes_status_cache().await
    }

    pub async fn set_runtime_status_core(
        self,
        environment: Environments,
        state: LifecycleState,
    ) -> Result<CacheRuntimes, Response> {
        self.set_runtime_status_cache(environment, state)
            .await
            .map_err(|err| Response::server_error(err))
    }

    pub async fn reset_runtime_core(self, environment: Environments) -> Result<(), Response> {
        self.reset_runtime_cache(environment)
            .await
            .map_err(|err| Response::server_error(err))
    }
}
