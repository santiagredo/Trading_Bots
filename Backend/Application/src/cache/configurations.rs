use std::sync::Arc;

use models::structs::Configuration;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::handler::Configurations;

static ACTIVE_CONFIG: Lazy<Arc<RwLock<Option<Configuration>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Configurations {
    pub async fn set_configuration(config: Configuration) -> Configuration {
        let mut active_config = ACTIVE_CONFIG.write().await;

        *active_config = Some(config.clone());

        config
    }

    pub async fn get_configuration() -> Option<Configuration> {
        let active_config = ACTIVE_CONFIG.read().await;

        active_config.clone()
    }
}

#[cfg(test)]
mod tests {
    use models::structs::Configuration;

    use crate::handler::Configurations;

    // Helpers
    async fn reset_config() {
        Configurations::set_configuration(Configuration::default()).await;
    }

    // Scenarios (unit responsibilities)

    // Verifies initial state is None
    async fn scenario_get_configuration_initially_none() {
        let config = Configurations::get_configuration().await;

        assert!(config.is_none());
    }

    // Verifies configuration is set correctly
    async fn scenario_set_configuration() {
        reset_config().await;

        let config = Configuration {
            ..Default::default()
        };

        Configurations::set_configuration(config.clone()).await;

        let cached = Configurations::get_configuration().await;

        assert_eq!(cached, Some(config));
    }

    // Verifies configuration overwrite
    async fn scenario_overwrite_configuration() {
        reset_config().await;

        let first = Configuration {
            dev_database_url: "Test 1".to_string(),
            ..Default::default()
        };
        let second = Configuration {
            dev_database_url: "Test 2".to_string(),
            ..Default::default()
        };

        Configurations::set_configuration(first).await;
        Configurations::set_configuration(second.clone()).await;

        let cached = Configurations::get_configuration().await;

        assert_eq!(cached, Some(second));
    }

    #[tokio::test]
    async fn configuration_unit_responsibilities() {
        scenario_get_configuration_initially_none().await;
        scenario_set_configuration().await;
        scenario_overwrite_configuration().await;

        // Final cleanup
        reset_config().await;
    }
}
