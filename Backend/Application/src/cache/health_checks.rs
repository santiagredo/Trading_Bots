use std::sync::Arc;

use models::structs::CacheHealthCheck;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::HealthCheck, utils::Cache};

static ACTIVE_HEALTH_CHECK: Lazy<Arc<RwLock<CacheHealthCheck>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheHealthCheck::new())));

impl HealthCheck<Cache> {
    pub async fn set_health_check_cache(health_check: CacheHealthCheck) -> CacheHealthCheck {
        let mut active_health_check = ACTIVE_HEALTH_CHECK.write().await;

        *active_health_check = health_check.clone();

        health_check
    }

    pub async fn get_health_check_cache() -> CacheHealthCheck {
        let active_health_check = ACTIVE_HEALTH_CHECK.read().await;

        active_health_check.clone()
    }
}
