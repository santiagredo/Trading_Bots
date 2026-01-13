use models::structs::{CacheRuntimes, Environments};

use crate::{
    handler::Runtimes,
    utils::{Core, Response},
};

impl Runtimes<Core> {
    pub async fn get_runtimes_status_core(self) -> CacheRuntimes {
        self.next_phase().get_runtimes_status_cache().await
    }

    pub async fn set_runtime_status_core(
        self,
        environment: Environments,
    ) -> Result<CacheRuntimes, Response> {
        self.next_phase()
            .set_runtime_status_cache(environment)
            .await
            .map_err(|err| Response::server_error(err))
    }

    pub async fn reset_runtime_core(self, environment: Environments) -> Result<(), Response> {
        self.next_phase()
            .reset_runtime_cache(environment)
            .await
            .map_err(|err| Response::server_error(err))
    }
}
