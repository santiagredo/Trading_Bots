use std::sync::Arc;

use models::structs::Configuration;
use once_cell::sync::Lazy;
use tokio::sync::{Notify, RwLock};

use crate::{handler::Configurations, utils::Cache};

static ACTIVE_CONFIG: Lazy<Arc<RwLock<Option<Configuration>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

pub static ACTIVE_SHUTDOWN: Lazy<Notify> = Lazy::new(Notify::new);

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

    pub fn stop_engine_cache() {
        ACTIVE_SHUTDOWN.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use models::structs::Configuration;

    use crate::{handler::Configurations, utils::Cache};

    // Helpers
    async fn reset_config() {
        Configurations::<Cache>::set_configuration_cache(Configuration::default()).await;
    }

    // Scenarios (unit responsibilities)

    // Verifies initial state is None
    async fn scenario_get_configuration_initially_none() {
        let config = Configurations::<Cache>::get_configuration_cache().await;

        assert!(config.is_none());
    }

    // Verifies configuration is set correctly
    async fn scenario_set_configuration_cache() {
        reset_config().await;

        let config = Configuration {
            api_key: "Test".to_string(),
            ..Default::default()
        };

        Configurations::<Cache>::set_configuration_cache(config.clone()).await;

        let cached = Configurations::<Cache>::get_configuration_cache().await;

        assert_eq!(cached, Some(config));
    }

    // Verifies configuration overwrite
    async fn scenario_overwrite_configuration_cache() {
        reset_config().await;

        let first = Configuration {
            dev_database_url: "Test 1".to_string(),
            ..Default::default()
        };
        let second = Configuration {
            dev_database_url: "Test 2".to_string(),
            ..Default::default()
        };

        Configurations::<Cache>::set_configuration_cache(first).await;
        Configurations::<Cache>::set_configuration_cache(second.clone()).await;

        let cached = Configurations::<Cache>::get_configuration_cache().await;

        assert_eq!(cached, Some(second));
    }

    #[tokio::test]
    async fn configuration_cache_unit_responsibilities() {
        scenario_get_configuration_initially_none().await;
        scenario_set_configuration_cache().await;
        scenario_overwrite_configuration_cache().await;

        // Final cleanup
        reset_config().await;
    }
}
