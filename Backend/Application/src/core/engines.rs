use models::structs::CacheEngine;

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

    pub async fn stop_engine_core(self) {
        self.next_phase().stop_engine_cache().await;
    }
}
