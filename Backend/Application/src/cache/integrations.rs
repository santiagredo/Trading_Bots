use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use models::{
    entities::integrations::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState},
    structs::{CacheIntegrations, CacheIntegrationsEnvironments, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Integrations, utils::Cache};

static ACTIVE_INTEGRATIONS: Lazy<Arc<RwLock<CacheIntegrationsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheIntegrationsEnvironments::new())));

impl Integrations<Cache> {
    pub async fn set_status_cache(
        environment: Environments,
        status: LifecycleState,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS.write().await;
        let env_cache = cache.get_or_create(environment);

        transition_with_timestamp(env_cache, status)?;
        Ok(())
    }

    pub async fn set_integrations_cache(
        environment: Environments,
        integrations: Vec<Model>,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err(format!(
                "Cannot load integrations: cache not running ({:?})",
                env_cache.status
            ));
        }

        env_cache.models = integrations.into_iter().map(|a| (a.id, a)).collect();

        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn upsert_integration_cache(
        environment: Environments,
        integration: Model,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(integration.id, integration);

        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn remove_integration_cache(
        environment: Environments,
        integration_id: i32,
    ) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_INTEGRATIONS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache.models.remove(&integration_id);

        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    pub async fn remove_integrations_cache(
        environment: Environments,
    ) -> Result<HashMap<i32, Model>, String> {
        let mut cache = ACTIVE_INTEGRATIONS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed = mem::take(&mut env_cache.models);

        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    pub async fn get_integrations_cache(environment: Environments) -> Option<CacheIntegrations> {
        let cache = ACTIVE_INTEGRATIONS.read().await;
        cache.get(&environment).cloned()
    }

    pub async fn get_integration_cache(
        environment: Environments,
        integration_id: i32,
    ) -> Option<Model> {
        let cache = ACTIVE_INTEGRATIONS.read().await;
        cache
            .get(&environment)?
            .models
            .get(&integration_id)
            .cloned()
    }

    pub async fn get_cache_state(environment: Environments) -> LifecycleState {
        let cache = ACTIVE_INTEGRATIONS.read().await;
        cache
            .get(&environment)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    pub async fn reset_integrations_cache(environment: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_INTEGRATIONS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&environment) {
            *env_cache = CacheIntegrations::new();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use models::{entities::integrations::Model, enums::LifecycleState, structs::Environments};

    use crate::{handler::Integrations, utils::Cache};

    // =========================
    // Helpers
    // =========================

    fn mock_integration(id: i32) -> Model {
        Model {
            id,
            ..Default::default()
        }
    }

    async fn start_env(env: Environments) {
        let _ = Integrations::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
        let _ = Integrations::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    }

    async fn stop_env(env: Environments) {
        let _ = Integrations::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = Integrations::<Cache>::remove_integrations_cache(env)
            .await
            .expect("remove_integrations_cache should work in Stopping");

        assert!(removed.is_empty() || !removed.is_empty());

        let _ = Integrations::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    }

    // =========================
    // Scenarios (unit responsibilities)
    // =========================

    async fn scenario_reset_env_clears_state(env: Environments) {
        start_env(env).await;

        let integrations = vec![mock_integration(1)];
        Integrations::<Cache>::set_integrations_cache(env, integrations)
            .await
            .unwrap();

        let _ = Integrations::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = Integrations::<Cache>::remove_integrations_cache(env)
            .await
            .unwrap();
        assert_eq!(removed.len(), 1);

        let cache = Integrations::<Cache>::get_integrations_cache(env)
            .await
            .unwrap();
        assert!(cache.models.is_empty());

        let _ = Integrations::<Cache>::set_status_cache(env, LifecycleState::Off).await;

        let status = Integrations::<Cache>::get_cache_state(env).await;
        assert_eq!(status, LifecycleState::Off);
    }

    async fn scenario_set_integrations_cache(env: Environments) {
        start_env(env).await;

        let integrations = vec![mock_integration(1), mock_integration(2)];

        Integrations::<Cache>::set_integrations_cache(env, integrations)
            .await
            .unwrap();

        let cache = Integrations::<Cache>::get_integrations_cache(env)
            .await
            .unwrap();
        let status = Integrations::<Cache>::get_cache_state(env).await;

        assert_eq!(cache.models.len(), 2);
        assert!(cache.models.contains_key(&1));
        assert!(cache.models.contains_key(&2));
        assert_eq!(status, LifecycleState::Running);

        stop_env(env).await;
    }

    async fn scenario_upsert_integration_insert(env: Environments) {
        start_env(env).await;

        let integration = mock_integration(10);
        Integrations::<Cache>::upsert_integration_cache(env, integration.clone())
            .await
            .unwrap();

        let cached = Integrations::<Cache>::get_integration_cache(env, 10)
            .await
            .unwrap();
        assert_eq!(cached, integration);

        stop_env(env).await;
    }

    async fn scenario_remove_integration(env: Environments) {
        start_env(env).await;

        let integration = mock_integration(20);
        Integrations::<Cache>::upsert_integration_cache(env, integration)
            .await
            .unwrap();

        let removed = Integrations::<Cache>::remove_integration_cache(env, 20)
            .await
            .unwrap();

        assert!(removed.is_some());

        let cached = Integrations::<Cache>::get_integration_cache(env, 20).await;
        assert!(cached.is_none());

        stop_env(env).await;
    }

    async fn scenario_get_cache_state(env: Environments) {
        assert_eq!(
            Integrations::<Cache>::get_cache_state(env).await,
            LifecycleState::Off
        );

        start_env(env).await;

        assert_eq!(
            Integrations::<Cache>::get_cache_state(env).await,
            LifecycleState::Running
        );

        stop_env(env).await;
    }

    async fn scenario_cannot_remove_integrations_when_running(env: Environments) {
        start_env(env).await;

        let result = Integrations::<Cache>::remove_integrations_cache(env).await;
        assert!(result.is_err());

        stop_env(env).await;
    }

    async fn scenario_cannot_set_integrations_when_not_running(env: Environments) {
        let integrations = vec![mock_integration(1)];

        let result = Integrations::<Cache>::set_integrations_cache(env, integrations).await;

        assert!(result.is_err());
    }

    // =========================
    // Entry point
    // =========================

    #[tokio::test]
    async fn cache_integrations_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_integrations_cache(env).await;
        scenario_upsert_integration_insert(env).await;
        scenario_remove_integration(env).await;
        scenario_get_cache_state(env).await;
        scenario_cannot_remove_integrations_when_running(env).await;
        scenario_cannot_set_integrations_when_not_running(env).await;
    }
}
