use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use migration::async_trait::async_trait;
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

use crate::{handler::IntegrationsSettings, utils::EntityCache};

static ACTIVE_INTEGRATIONS_SETTINGS: Lazy<Arc<RwLock<CacheIntegrationsSettingsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheIntegrationsSettingsEnvironments::new())));

#[async_trait]
impl<R> EntityCache<Environments> for IntegrationsSettings<R>
where
    R: Send + Sync,
{
    type Key = i32;
    type Value = Model;
    type Collection = CacheIntegrationsSettings;

    async fn state(&self, env: Environments) -> LifecycleState {
        let cache = ACTIVE_INTEGRATIONS_SETTINGS.read().await;
        cache
            .get(&env)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    async fn set_state(&self, env: Environments, state: LifecycleState) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS_SETTINGS.write().await;
        let env_cache = cache.get_or_create(env);

        transition_with_timestamp(env_cache, state)?;
        Ok(())
    }

    async fn get_all(&self, env: Environments) -> Option<CacheIntegrationsSettings> {
        let cache = ACTIVE_INTEGRATIONS_SETTINGS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        Some(env_cache.clone())
    }

    async fn get(&self, env: Environments, key: i32) -> Option<Model> {
        let cache = ACTIVE_INTEGRATIONS_SETTINGS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        env_cache
            .integrations_map
            .values()
            .find_map(|group| group.models.get(&key))
            .cloned()
    }

    async fn set_all(&self, env: Environments, values: Vec<Model>) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS_SETTINGS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let values: Vec<(i32, Model)> = values.into_iter().map(|m| (m.id, m)).collect();

        env_cache.integrations_map.clear();

        for (_, setting) in values {
            env_cache
                .integrations_map
                .entry(setting.integration_id)
                .or_insert_with(CacheIntegrationSetting::default)
                .models
                .insert(setting.id, setting);
        }

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    async fn upsert(&self, env: Environments, key: i32, value: Model) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS_SETTINGS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache
            .integrations_map
            .entry(value.integration_id)
            .or_insert_with(CacheIntegrationSetting::default)
            .models
            .insert(key, value);

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    async fn remove(&self, env: Environments, key: i32) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_INTEGRATIONS_SETTINGS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let mut removed: Option<Model> = None;
        let mut empty_integration_id: Option<i32> = None;

        for (integration_id, group) in env_cache.integrations_map.iter_mut() {
            if let Some(val) = group.models.remove(&key) {
                removed = Some(val);

                if group.models.is_empty() {
                    empty_integration_id = Some(*integration_id);
                }

                break;
            }
        }

        if let Some(integration_id) = empty_integration_id {
            env_cache.integrations_map.remove(&integration_id);
        }

        env_cache.last_update_date = Local::now().naive_local();
        Ok(removed)
    }

    async fn remove_all(&self, env: Environments) -> Result<HashMap<i32, Model>, String> {
        let mut cache = ACTIVE_INTEGRATIONS_SETTINGS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed_groups = mem::take(&mut env_cache.integrations_map);

        let mut flat = HashMap::new();
        for group in removed_groups.values() {
            for (id, model) in &group.models {
                flat.insert(*id, model.clone());
            }
        }

        env_cache.last_update_date = Local::now().naive_local();
        Ok(flat)
    }

    async fn reset(&self, env: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS_SETTINGS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&env) {
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

    use crate::{handler::IntegrationsSettings, utils::EntityCache};

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

    async fn new_service() -> IntegrationsSettings<()> {
        IntegrationsSettings::blank()
    }

    async fn start_env(service: &IntegrationsSettings<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Starting)
            .await
            .unwrap();

        service
            .set_state(env, LifecycleState::Running)
            .await
            .unwrap();
    }

    async fn stop_env(service: &IntegrationsSettings<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert!(removed.is_empty() || !removed.is_empty());

        service.set_state(env, LifecycleState::Off).await.unwrap();
    }

    // =========================
    // Unit responsibilities
    // =========================

    #[tokio::test]
    async fn cache_integrations_settings_unit_responsibilities() {
        let env = Environments::DEV;
        let service = new_service().await;

        // -------------------------
        // reset clears state
        // -------------------------
        start_env(&service, env).await;

        service
            .set_all(env, vec![mock_integration_setting(1, 100)])
            .await
            .unwrap();

        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert_eq!(removed.len(), 1);

        service.set_state(env, LifecycleState::Off).await.unwrap();

        assert_eq!(service.state(env).await, LifecycleState::Off);

        // -------------------------
        // set / get
        // -------------------------
        start_env(&service, env).await;

        service
            .set_all(
                env,
                vec![
                    mock_integration_setting(1, 10),
                    mock_integration_setting(2, 10),
                    mock_integration_setting(3, 20),
                ],
            )
            .await
            .unwrap();

        let cache = service.get_all(env).await.unwrap();
        assert_eq!(cache.integrations_map.len(), 2);
        assert_eq!(cache.integrations_map.get(&10).unwrap().models.len(), 2);
        assert_eq!(cache.integrations_map.get(&20).unwrap().models.len(), 1);

        // -------------------------
        // upsert
        // -------------------------
        service
            .upsert(env, 10, mock_integration_setting(10, 30))
            .await
            .unwrap();

        assert!(service.get(env, 10).await.is_some());

        // -------------------------
        // remove
        // -------------------------
        service.remove(env, 10).await.unwrap();
        assert!(service.get(env, 10).await.is_none());

        // -------------------------
        // cannot remove_all while running
        // -------------------------
        assert!(service.remove_all(env).await.is_err());

        stop_env(&service, env).await;
    }
}
