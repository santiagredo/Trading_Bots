use models::{
    enums::{transition_with_timestamp, LifecycleState},
    structs::CacheEngine,
};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

use crate::handler::Engines;

static ACTIVE_ENGINE: Lazy<Arc<RwLock<CacheEngine>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheEngine::new())));

pub static ACTIVE_ENGINE_TOKEN: Lazy<CancellationToken> = Lazy::new(CancellationToken::new);

impl<R> Engines<R>
where
    R: Send + Sync,
{
    pub async fn set_state(state: LifecycleState) -> Result<CacheEngine, String> {
        let mut active_engine = ACTIVE_ENGINE.write().await;

        transition_with_timestamp(&mut *active_engine, state)?;

        Ok(active_engine.clone())
    }

    pub async fn state() -> LifecycleState {
        let active_engine = ACTIVE_ENGINE.read().await;

        active_engine.status.clone()
    }

    pub async fn get_all() -> CacheEngine {
        let active_engine = ACTIVE_ENGINE.read().await;

        active_engine.clone()
    }

    pub fn get_engine_token() -> CancellationToken {
        ACTIVE_ENGINE_TOKEN.clone()
    }

    pub fn stop_engine_token() {
        ACTIVE_ENGINE_TOKEN.cancel();
    }
}
