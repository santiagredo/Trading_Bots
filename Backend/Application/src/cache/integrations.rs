use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use migration::async_trait::async_trait;
use models::{
    entities::integrations::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState},
    structs::{CacheIntegrations, CacheIntegrationsEnvironments, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Integrations, utils::EntityCache};

static ACTIVE_INTEGRATIONS: Lazy<Arc<RwLock<CacheIntegrationsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheIntegrationsEnvironments::new())));

#[async_trait]
impl<R> EntityCache<Environments> for Integrations<R>
where
    R: Send + Sync,
{
    type Key = i32;
    type Value = Model;
    type Collection = CacheIntegrations;

    async fn state(&self, env: Environments) -> LifecycleState {
        let cache = ACTIVE_INTEGRATIONS.read().await;
        cache
            .get(&env)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    async fn set_state(&self, env: Environments, state: LifecycleState) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS.write().await;
        let env_cache = cache.get_or_create(env);

        transition_with_timestamp(env_cache, state)?;
        Ok(())
    }

    async fn get_all(&self, env: Environments) -> Option<CacheIntegrations> {
        let cache = ACTIVE_INTEGRATIONS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        Some(env_cache.clone())
    }

    async fn get(&self, env: Environments, key: i32) -> Option<Model> {
        let cache = ACTIVE_INTEGRATIONS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        env_cache.models.get(&key).cloned()
    }

    async fn set_all(&self, env: Environments, values: Vec<Model>) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let values: Vec<(i32, Model)> = values.into_iter().map(|m| (m.id, m)).collect();

        env_cache.models = values.into_iter().collect();
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    async fn upsert(&self, env: Environments, key: i32, value: Model) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(key, value);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    async fn remove(&self, env: Environments, key: i32) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_INTEGRATIONS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache.models.remove(&key);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    async fn remove_all(&self, env: Environments) -> Result<HashMap<i32, Model>, String> {
        let mut cache = ACTIVE_INTEGRATIONS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed = mem::take(&mut env_cache.models);

        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    async fn reset(&self, env: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&env) {
            *env_cache = CacheIntegrations::new();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use models::{entities::integrations::Model, enums::LifecycleState, structs::Environments};

    use crate::{handler::Integrations, utils::EntityCache};

    // =========================
    // Helpers
    // =========================

    fn mock_integration(id: i32) -> Model {
        Model {
            id,
            ..Default::default()
        }
    }

    async fn new_service() -> Integrations<()> {
        Integrations::blank()
    }

    async fn start_env(service: &Integrations<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Starting)
            .await
            .unwrap();

        service
            .set_state(env, LifecycleState::Running)
            .await
            .unwrap();
    }

    async fn stop_env(service: &Integrations<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert!(removed.is_empty() || !removed.is_empty());

        service.set_state(env, LifecycleState::Off).await.unwrap();
    }

    // =========================
    // Scenarios
    // =========================

    async fn scenario_reset_env_clears_state(service: &Integrations<()>, env: Environments) {
        start_env(service, env).await;

        service
            .set_all(env, vec![mock_integration(1)])
            .await
            .unwrap();

        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert_eq!(removed.len(), 1);

        let cache = service.get_all(env).await;
        assert!(cache.is_none());

        service.set_state(env, LifecycleState::Off).await.unwrap();

        let status = service.state(env).await;
        assert_eq!(status, LifecycleState::Off);
    }

    async fn scenario_set_integrations_cache(service: &Integrations<()>, env: Environments) {
        start_env(service, env).await;

        let integrations = vec![mock_integration(1), mock_integration(2)];

        service.set_all(env, integrations).await.unwrap();

        let cache = service.get_all(env).await.unwrap();
        let status = service.state(env).await;

        assert_eq!(cache.models.len(), 2);
        assert!(cache.models.contains_key(&1));
        assert!(cache.models.contains_key(&2));
        assert_eq!(status, LifecycleState::Running);

        stop_env(service, env).await;
    }

    async fn scenario_upsert_integration_insert(service: &Integrations<()>, env: Environments) {
        start_env(service, env).await;

        let integration = mock_integration(10);

        service
            .upsert(env, integration.id, integration.clone())
            .await
            .unwrap();

        let cached = service.get(env, 10).await.unwrap();

        assert_eq!(cached, integration);

        stop_env(service, env).await;
    }

    async fn scenario_remove_integration(service: &Integrations<()>, env: Environments) {
        start_env(service, env).await;

        let integration = mock_integration(20);

        service
            .upsert(env, integration.id, integration)
            .await
            .unwrap();

        let removed = service.remove(env, 20).await.unwrap();
        assert!(removed.is_some());

        let cached = service.get(env, 20).await;
        assert!(cached.is_none());

        stop_env(service, env).await;
    }

    async fn scenario_get_cache_state(service: &Integrations<()>, env: Environments) {
        assert_eq!(service.state(env).await, LifecycleState::Off);

        start_env(service, env).await;

        assert_eq!(service.state(env).await, LifecycleState::Running);

        stop_env(service, env).await;
    }

    async fn scenario_cannot_remove_integrations_when_running(
        service: &Integrations<()>,
        env: Environments,
    ) {
        start_env(service, env).await;

        let result = service.remove_all(env).await;
        assert!(result.is_err());

        stop_env(service, env).await;
    }

    async fn scenario_cannot_set_integrations_when_not_running(
        service: &Integrations<()>,
        env: Environments,
    ) {
        let result = service.set_all(env, vec![mock_integration(1)]).await;

        assert!(result.is_err());
    }

    // =========================
    // Entry point
    // =========================

    #[tokio::test]
    async fn cache_integrations_unit_responsibilities() {
        let env = Environments::DEV;
        let service = new_service().await;

        scenario_reset_env_clears_state(&service, env).await;
        scenario_set_integrations_cache(&service, env).await;
        scenario_upsert_integration_insert(&service, env).await;
        scenario_remove_integration(&service, env).await;
        scenario_get_cache_state(&service, env).await;
        scenario_cannot_remove_integrations_when_running(&service, env).await;
        scenario_cannot_set_integrations_when_not_running(&service, env).await;
    }
}
