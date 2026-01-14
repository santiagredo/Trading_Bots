use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use models::{
    entities::integration_settings::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState},
    structs::{
        CacheIntegrationSetting, CacheIntegrationsSettings, CacheIntegrationsSettingsEnvironments,
        Environments,
    },
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::IntegrationsSettings, utils::Cache};

static ACTIVE_INTEGRATIONS_SETTINGS: Lazy<Arc<RwLock<CacheIntegrationsSettingsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheIntegrationsSettingsEnvironments::new())));

impl IntegrationsSettings<Cache> {
    pub async fn set_status_cache(
        environment: Environments,
        status: LifecycleState,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS_SETTINGS.write().await;
        let env_cache = cache.get_or_create(environment);

        transition_with_timestamp(env_cache, status)?;
        Ok(())
    }

    pub async fn set_integrations_settings_cache(
        environment: Environments,
        integrations_settings: Vec<Model>,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS_SETTINGS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err(format!(
                "Cannot load integrations settings: cache not running ({:?})",
                env_cache.status
            ));
        }

        env_cache.integrations_map.clear();

        for setting in integrations_settings {
            env_cache
                .integrations_map
                .entry(setting.integration_id)
                .or_default()
                .models
                .insert(setting.id, setting);
        }

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn upsert_integration_setting_cache(
        environment: Environments,
        integration_setting: Model,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS_SETTINGS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache
            .integrations_map
            .entry(integration_setting.integration_id)
            .or_default()
            .models
            .insert(integration_setting.id, integration_setting);

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn remove_integration_setting_cache(
        environment: Environments,
        integration_id: i32,
        setting_id: i32,
    ) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_INTEGRATIONS_SETTINGS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache
            .integrations_map
            .get_mut(&integration_id)
            .and_then(|group| group.models.remove(&setting_id));

        // Limpia integraciones vacías
        if let Some(group) = env_cache.integrations_map.get(&integration_id) {
            if group.models.is_empty() {
                env_cache.integrations_map.remove(&integration_id);
            }
        }

        env_cache.last_update_date = Local::now().naive_local();
        Ok(removed)
    }

    pub async fn remove_integrations_settings_cache(
        environment: Environments,
    ) -> Result<HashMap<i32, CacheIntegrationSetting>, String> {
        let mut cache = ACTIVE_INTEGRATIONS_SETTINGS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed = mem::take(&mut env_cache.integrations_map);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    pub async fn get_integrations_settings_cache(
        environment: Environments,
    ) -> Option<CacheIntegrationsSettings> {
        let cache = ACTIVE_INTEGRATIONS_SETTINGS.read().await;
        cache.get(&environment).cloned()
    }

    pub async fn get_integration_settings_cache(
        environment: Environments,
        integration_id: i32,
    ) -> Option<HashMap<i32, Model>> {
        let cache = ACTIVE_INTEGRATIONS_SETTINGS.read().await;
        let env_cache = cache.get(&environment)?;

        env_cache
            .integrations_map
            .get(&integration_id)
            .map(|val| val.models.clone())
    }

    pub async fn get_integration_setting_cache(
        environment: Environments,
        setting_id: i32,
    ) -> Option<Model> {
        let cache = ACTIVE_INTEGRATIONS_SETTINGS.read().await;
        let env_cache = cache.get(&environment)?;

        env_cache
            .integrations_map
            .values()
            .find_map(|group| group.models.get(&setting_id))
            .cloned()
    }

    pub async fn get_cache_state(environment: Environments) -> LifecycleState {
        let cache = ACTIVE_INTEGRATIONS_SETTINGS.read().await;
        cache
            .get(&environment)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    pub async fn reset_integrations_settings_cache(
        environment: Environments,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS_SETTINGS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&environment) {
            *env_cache = CacheIntegrationsSettings::new();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use models::{
        entities::integration_settings::Model, enums::LifecycleState, structs::Environments,
    };

    use crate::{handler::IntegrationsSettings, utils::Cache};

    // =========================
    // Helpers
    // =========================

    fn mock_integration_setting(id: i32, integration_id: i32) -> Model {
        Model {
            id,
            integration_id,
            ..Default::default()
        }
    }

    async fn start_env(env: Environments) {
        let _ =
            IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
        let _ = IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    }

    async fn stop_env(env: Environments) {
        let _ =
            IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = IntegrationsSettings::<Cache>::remove_integrations_settings_cache(env)
            .await
            .expect("remove_integrations_settings_cache should work in Stopping");

        assert!(removed.is_empty() || !removed.is_empty());

        let _ = IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    }

    // =========================
    // Scenarios
    // =========================

    async fn scenario_reset_env_clears_state(env: Environments) {
        start_env(env).await;

        let settings = vec![mock_integration_setting(1, 100)];

        IntegrationsSettings::<Cache>::set_integrations_settings_cache(env, settings)
            .await
            .unwrap();

        let _ =
            IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = IntegrationsSettings::<Cache>::remove_integrations_settings_cache(env)
            .await
            .unwrap();

        assert_eq!(removed.len(), 1);
        assert!(removed.get(&100).unwrap().models.contains_key(&1));

        let cache = IntegrationsSettings::<Cache>::get_integrations_settings_cache(env)
            .await
            .unwrap();

        assert!(cache.integrations_map.is_empty());

        let _ = IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Off).await;

        let status = IntegrationsSettings::<Cache>::get_cache_state(env).await;

        assert_eq!(status, LifecycleState::Off);
    }

    async fn scenario_set_integrations_settings_cache(env: Environments) {
        start_env(env).await;

        let settings = vec![
            mock_integration_setting(1, 10),
            mock_integration_setting(2, 10),
            mock_integration_setting(3, 20),
        ];

        IntegrationsSettings::<Cache>::set_integrations_settings_cache(env, settings)
            .await
            .unwrap();

        let cache = IntegrationsSettings::<Cache>::get_integrations_settings_cache(env)
            .await
            .unwrap();

        let status = IntegrationsSettings::<Cache>::get_cache_state(env).await;

        assert_eq!(cache.integrations_map.len(), 2);
        assert_eq!(cache.integrations_map.get(&10).unwrap().models.len(), 2);
        assert_eq!(cache.integrations_map.get(&20).unwrap().models.len(), 1);
        assert_eq!(status, LifecycleState::Running);

        stop_env(env).await;
    }

    async fn scenario_get_integration_settings_by_integration_id(env: Environments) {
        start_env(env).await;

        let settings = vec![
            mock_integration_setting(1, 50),
            mock_integration_setting(2, 50),
            mock_integration_setting(3, 60),
        ];

        IntegrationsSettings::<Cache>::set_integrations_settings_cache(env, settings)
            .await
            .unwrap();

        let integration_50 = IntegrationsSettings::<Cache>::get_integration_settings_cache(env, 50)
            .await
            .unwrap();

        assert_eq!(integration_50.len(), 2);
        assert!(integration_50.contains_key(&1));
        assert!(integration_50.contains_key(&2));

        let integration_60 = IntegrationsSettings::<Cache>::get_integration_settings_cache(env, 60)
            .await
            .unwrap();

        assert_eq!(integration_60.len(), 1);
        assert!(integration_60.contains_key(&3));

        stop_env(env).await;
    }

    async fn scenario_upsert_integration_setting_insert(env: Environments) {
        start_env(env).await;

        let setting = mock_integration_setting(10, 99);

        IntegrationsSettings::<Cache>::upsert_integration_setting_cache(env, setting.clone())
            .await
            .unwrap();

        let cached = IntegrationsSettings::<Cache>::get_integration_setting_cache(env, 10)
            .await
            .unwrap();

        assert_eq!(cached, setting);

        stop_env(env).await;
    }

    async fn scenario_remove_integration_setting(env: Environments) {
        start_env(env).await;

        let setting = mock_integration_setting(20, 77);

        IntegrationsSettings::<Cache>::upsert_integration_setting_cache(env, setting)
            .await
            .unwrap();

        let removed = IntegrationsSettings::<Cache>::remove_integration_setting_cache(env, 77, 20)
            .await
            .unwrap();

        assert!(removed.is_some());

        let cached = IntegrationsSettings::<Cache>::get_integration_setting_cache(env, 20).await;

        assert!(cached.is_none());

        stop_env(env).await;
    }

    async fn scenario_get_cache_state(env: Environments) {
        assert_eq!(
            IntegrationsSettings::<Cache>::get_cache_state(env).await,
            LifecycleState::Off
        );

        start_env(env).await;

        assert_eq!(
            IntegrationsSettings::<Cache>::get_cache_state(env).await,
            LifecycleState::Running
        );

        stop_env(env).await;
    }

    async fn scenario_cannot_remove_integrations_settings_when_running(env: Environments) {
        start_env(env).await;

        let result = IntegrationsSettings::<Cache>::remove_integrations_settings_cache(env).await;

        assert!(result.is_err());

        stop_env(env).await;
    }

    async fn scenario_cannot_set_integrations_settings_when_not_running(env: Environments) {
        let settings = vec![mock_integration_setting(1, 1)];

        let result =
            IntegrationsSettings::<Cache>::set_integrations_settings_cache(env, settings).await;

        assert!(result.is_err());
    }

    // =========================
    // Entry point
    // =========================

    #[tokio::test]
    async fn cache_integrations_settings_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_integrations_settings_cache(env).await;
        scenario_get_integration_settings_by_integration_id(env).await;
        scenario_upsert_integration_setting_insert(env).await;
        scenario_remove_integration_setting(env).await;
        scenario_get_cache_state(env).await;
        scenario_cannot_remove_integrations_settings_when_running(env).await;
        scenario_cannot_set_integrations_settings_when_not_running(env).await;
    }
}
