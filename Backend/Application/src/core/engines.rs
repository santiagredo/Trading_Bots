use models::{enums::LifecycleState, structs::CacheEngine};
use tokio_util::sync::CancellationToken;

use crate::{
    handler::Engines,
    utils::{Core, Response},
};

impl Engines<Core> {
    pub async fn get_engine_status_core(self) -> CacheEngine {
        self.next_phase().get_engine_status_cache().await
    }

    pub async fn set_engine_status_core(self) -> Result<CacheEngine, Response> {
        self.next_phase()
            .set_engine_status_cache()
            .await
            .map_err(|err| Response::server_error(err))
    }

    pub fn get_engine_token_core(self) -> CancellationToken {
        self.next_phase().get_engine_token_cache()
    }

    pub async fn stop_engine_core(self) {
        let _ = Engines::new(LifecycleState::Stopping)
            .set_engine_status()
            .await;

        self.next_phase().stop_engine_cache()
    }
}
