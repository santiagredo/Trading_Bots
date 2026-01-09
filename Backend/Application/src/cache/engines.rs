use models::{enums::transition_with_timestamp, structs::CacheEngine};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::{Notify, RwLock};

use crate::{handler::Engines, utils::Cache};

static ACTIVE_ENGINE: Lazy<Arc<RwLock<CacheEngine>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheEngine::new())));

pub static ACTIVE_SHUTDOWN: Lazy<Notify> = Lazy::new(Notify::new);

impl Engines<Cache> {
    pub async fn set_engine_status_cache(self) -> Result<CacheEngine, String> {
        let mut active_engine = ACTIVE_ENGINE.write().await;

        transition_with_timestamp(&mut *active_engine, self.model)?;

        Ok(active_engine.clone())
    }

    pub async fn get_engine_status_cache(self) -> CacheEngine {
        let active_engine = ACTIVE_ENGINE.read().await;

        active_engine.clone()
    }

    pub async fn stop_engine_cache(self) {
        ACTIVE_SHUTDOWN.notify_one();
    }
}
