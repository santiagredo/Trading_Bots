use models::enums::LifecycleState;
use models::structs::CacheEngine;
use tokio_util::sync::CancellationToken;

use crate::handler::Engines;
use crate::utils::Response;

impl<R> Engines<R>
where
    R: Send + Sync,
{
    /* ===========================
     * CACHE WRITE
     * ===========================
     */

    pub async fn set_engine_state(&self, state: LifecycleState) -> Result<CacheEngine, String> {
        Self::set_state(state).await
    }

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_engine_cache(&self) -> CacheEngine {
        Self::get_all().await
    }

    pub async fn get_engine_state(&self) -> LifecycleState {
        Self::state().await
    }

    pub fn get_engine_token_core(&self) -> CancellationToken {
        Self::get_engine_token()
    }

    /* ===========================
     * STOP / RESET
     * ===========================
     */

    pub async fn stop_engine(&self) -> Result<(), Response> {
        Self::set_state(LifecycleState::Stopping)
            .await
            .map_err(Response::server_error)?;

        Self::stop_engine_token();

        Self::set_state(LifecycleState::Off)
            .await
            .map_err(Response::server_error)?;

        Ok(())
    }

    pub async fn reset_engine(&self) -> Result<(), Response> {
        Self::set_state(LifecycleState::Off)
            .await
            .map_err(Response::server_error)?;

        Ok(())
    }
}
