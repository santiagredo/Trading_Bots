use chrono::Local;
use models::{
    enums::{transition_with_timestamp, LifecycleState},
    structs::{CacheRuntimes, Environments},
};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{handler::Runtimes, utils::Cache};

static ACTIVE_RUNTIMES: Lazy<Arc<RwLock<CacheRuntimes>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheRuntimes::new())));

impl Runtimes<Cache> {
    pub async fn set_runtime_status_cache(
        self,
        environment: Environments,
    ) -> Result<CacheRuntimes, String> {
        let mut active_runtimes = ACTIVE_RUNTIMES.write().await;

        let active_runtime = match environment {
            Environments::PROD => &mut active_runtimes.prod,
            _ => &mut active_runtimes.dev,
        };

        let next_model = match environment {
            Environments::PROD => self.model.prod.status,
            _ => self.model.dev.status,
        };

        transition_with_timestamp(&mut *active_runtime, next_model)?;

        Ok(active_runtimes.clone())
    }

    pub async fn get_runtimes_status_cache(self) -> CacheRuntimes {
        let active_runtimes = ACTIVE_RUNTIMES.read().await;

        active_runtimes.clone()
    }

    pub async fn reset_runtime_cache(self, environment: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_RUNTIMES.write().await;

        let active_runtime = match environment {
            Environments::PROD => &mut cache.prod,
            _ => &mut cache.dev,
        };

        active_runtime.status = LifecycleState::Off;

        active_runtime.last_update_date = Local::now().naive_local();

        Ok(())
    }
}
