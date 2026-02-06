use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use models::{
    entities::status::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState, Status},
    structs::{CacheStatus, CacheStatusEnvironments, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::handler::OrderStatus;

static ACTIVE_STATUS: Lazy<Arc<RwLock<CacheStatusEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheStatusEnvironments::new())));

impl OrderStatus {
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
