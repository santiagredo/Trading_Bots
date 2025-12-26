use std::sync::Arc;

use models::structs::Configuration;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Configurations, utils::Cache};

static ACTIVE_CONFIG: Lazy<Arc<RwLock<Option<Configuration>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Configurations<Cache> {
    pub async fn set_configuration_cache(config: Configuration) -> Configuration {
        let mut active_config = ACTIVE_CONFIG.write().await;

        *active_config = Some(config.clone());

        config
    }

    pub async fn get_configuration_cache() -> Option<Configuration> {
        let active_config = ACTIVE_CONFIG.read().await;

        active_config.clone()
    }
}
