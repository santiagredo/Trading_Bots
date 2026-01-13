use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use models::{
    entities::status::Model,
    enums::{FiniteStateMachine, LifecycleState, Status, transition_with_timestamp},
    structs::{CacheStatus, CacheStatusEnvironments, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::OrderStatus, utils::Cache};

static ACTIVE_STATUS: Lazy<Arc<RwLock<CacheStatusEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheStatusEnvironments::new())));

impl OrderStatus<Cache> {
    // =========================
    // Lifecycle
    // =========================

    pub async fn set_status_cache(
        environment: Environments,
        status: LifecycleState,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STATUS.write().await;
        let env_cache = cache.get_or_create(environment);

        transition_with_timestamp(env_cache, status)?;
        Ok(())
    }

    // =========================
    // Mutations
    // =========================

    pub async fn set_statuses_cache(
        environment: Environments,
        models: Vec<Model>,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STATUS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err(format!(
                "Cannot load status: cache not running ({:?})",
                env_cache.status
            ));
        }

        env_cache.models = models.into_iter().map(|m| (m.id, m)).collect();
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn upsert_status_cache(
        environment: Environments,
        model: Model,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STATUS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(model.id, model);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn remove_status_cache(
        environment: Environments,
        status_id: i32,
    ) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_STATUS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache.models.remove(&status_id);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    pub async fn remove_statuses_cache(
        environment: Environments,
    ) -> Result<HashMap<i32, Model>, String> {
        let mut cache = ACTIVE_STATUS.write().await;
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

    // =========================
    // Queries
    // =========================

    pub async fn get_status_cache(environment: Environments) -> Option<CacheStatus> {
        let cache = ACTIVE_STATUS.read().await;
        cache.get(&environment).cloned()
    }

    pub async fn get_status_by_enum(environment: Environments, status: Status) -> Option<Model> {
        let cache = ACTIVE_STATUS.read().await;
        let env_cache = cache.get(&environment)?;

        env_cache
            .models
            .values()
            .find(|m| Status::from_model(m) == status)
            .cloned()
    }

    pub async fn get_cache_state(environment: Environments) -> LifecycleState {
        let cache = ACTIVE_STATUS.read().await;
        cache
            .get(&environment)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    pub async fn reset_status_cache(environment: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_STATUS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&environment) {
            *env_cache = CacheStatus::new();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use models::{
        entities::status::Model,
        enums::{LifecycleState, Status},
        structs::Environments,
    };

    use crate::{handler::OrderStatus, utils::Cache};

    // =========================
    // Helpers
    // =========================

    fn mock_status(id: i32, name: &str) -> Model {
        Model {
            id,
            name: name.to_string(),
        }
    }

    async fn start_env(env: Environments) {
        let _ = OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
        let _ = OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    }

    async fn stop_env(env: Environments) {
        let _ = OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let _ = OrderStatus::<Cache>::remove_statuses_cache(env)
            .await
            .expect("remove_statuses_cache should work in Stopping");

        let _ = OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    }

    // =========================
    // Scenarios
    // =========================

    async fn scenario_reset_env_clears_state(env: Environments) {
        start_env(env).await;

        OrderStatus::<Cache>::set_statuses_cache(env, vec![mock_status(1, "OPEN")])
            .await
            .unwrap();

        let _ = OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = OrderStatus::<Cache>::remove_statuses_cache(env)
            .await
            .unwrap();

        assert_eq!(removed.len(), 1);

        let cache = OrderStatus::<Cache>::get_status_cache(env).await.unwrap();
        assert!(cache.models.is_empty());

        let _ = OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Off).await;

        let state = OrderStatus::<Cache>::get_cache_state(env).await;
        assert_eq!(state, LifecycleState::Off);
    }

    async fn scenario_set_status_cache(env: Environments) {
        start_env(env).await;

        OrderStatus::<Cache>::set_statuses_cache(
            env,
            vec![
                mock_status(1, "OPEN"),
                mock_status(2, "COMPLETED"),
                mock_status(3, "ABORTED"),
            ],
        )
        .await
        .unwrap();

        let cache = OrderStatus::<Cache>::get_status_cache(env).await.unwrap();

        assert_eq!(cache.models.len(), 3);
        assert!(cache.models.contains_key(&1));
        assert!(cache.models.contains_key(&2));
        assert!(cache.models.contains_key(&3));

        stop_env(env).await;
    }

    async fn scenario_get_status_by_enum(env: Environments) {
        start_env(env).await;

        OrderStatus::<Cache>::set_statuses_cache(env, vec![mock_status(10, "COMPLETED")])
            .await
            .unwrap();

        let cached = OrderStatus::<Cache>::get_status_by_enum(env, Status::Completed)
            .await
            .unwrap();

        assert_eq!(cached.id, 10);

        stop_env(env).await;
    }

    async fn scenario_upsert_status(env: Environments) {
        start_env(env).await;

        let status = mock_status(20, "OPEN");

        OrderStatus::<Cache>::upsert_status_cache(env, status.clone())
            .await
            .unwrap();

        let cached = OrderStatus::<Cache>::get_status_by_enum(env, Status::Open)
            .await
            .unwrap();

        assert_eq!(cached, status);

        stop_env(env).await;
    }

    async fn scenario_remove_single_status(env: Environments) {
        start_env(env).await;

        OrderStatus::<Cache>::upsert_status_cache(env, mock_status(30, "ABORTED"))
            .await
            .unwrap();

        let removed = OrderStatus::<Cache>::remove_status_cache(env, 30)
            .await
            .unwrap();

        assert!(removed.is_some());

        let missing = OrderStatus::<Cache>::get_status_by_enum(env, Status::Aborted).await;
        assert!(missing.is_none());

        stop_env(env).await;
    }

    async fn scenario_get_cache_state(env: Environments) {
        assert_eq!(
            OrderStatus::<Cache>::get_cache_state(env).await,
            LifecycleState::Off
        );

        start_env(env).await;

        assert_eq!(
            OrderStatus::<Cache>::get_cache_state(env).await,
            LifecycleState::Running
        );

        stop_env(env).await;
    }

    async fn scenario_cannot_mutate_when_not_running(env: Environments) {
        let result =
            OrderStatus::<Cache>::set_statuses_cache(env, vec![mock_status(1, "OPEN")]).await;

        assert!(result.is_err());
    }

    async fn scenario_cannot_remove_all_when_running(env: Environments) {
        start_env(env).await;

        let result = OrderStatus::<Cache>::remove_statuses_cache(env).await;
        assert!(result.is_err());

        stop_env(env).await;
    }

    // =========================
    // Entry point
    // =========================

    #[tokio::test]
    async fn cache_order_status_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_status_cache(env).await;
        scenario_get_status_by_enum(env).await;
        scenario_upsert_status(env).await;
        scenario_remove_single_status(env).await;
        scenario_get_cache_state(env).await;
        scenario_cannot_mutate_when_not_running(env).await;
        scenario_cannot_remove_all_when_running(env).await;
    }
}
