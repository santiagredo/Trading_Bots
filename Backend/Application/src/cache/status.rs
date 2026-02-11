use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use migration::async_trait::async_trait;
use models::{
    entities::status::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState, Status},
    structs::{CacheStatus, CacheStatusEnvironments, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::OrderStatus, utils::EntityCache};

static ACTIVE_STATUS: Lazy<Arc<RwLock<CacheStatusEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheStatusEnvironments::new())));

#[async_trait]
impl<R> EntityCache<Environments> for OrderStatus<R>
where
    R: Send + Sync,
{
    type Key = i32;
    type Value = Model;
    type Collection = CacheStatus;

    // =========================
    // Lifecycle
    // =========================

    async fn state(&self, env: Environments) -> LifecycleState {
        let cache = ACTIVE_STATUS.read().await;
        cache
            .get(&env)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    async fn set_state(&self, env: Environments, state: LifecycleState) -> Result<(), String> {
        let mut cache = ACTIVE_STATUS.write().await;
        let env_cache = cache.get_or_create(env);

        transition_with_timestamp(env_cache, state)?;
        Ok(())
    }

    // =========================
    // Queries
    // =========================

    async fn get_all(&self, env: Environments) -> Option<CacheStatus> {
        let cache = ACTIVE_STATUS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        Some(env_cache.clone())
    }

    async fn get(&self, env: Environments, key: i32) -> Option<Model> {
        let cache = ACTIVE_STATUS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        env_cache.models.get(&key).cloned()
    }

    // =========================
    // Mutations
    // =========================

    async fn set_all(&self, env: Environments, values: Vec<Model>) -> Result<(), String> {
        let mut cache = ACTIVE_STATUS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models = values.into_iter().map(|m| (m.id, m)).collect();
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    async fn upsert(&self, env: Environments, key: i32, value: Model) -> Result<(), String> {
        let mut cache = ACTIVE_STATUS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(key, value);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    async fn remove(&self, env: Environments, key: i32) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_STATUS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache.models.remove(&key);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    async fn remove_all(&self, env: Environments) -> Result<HashMap<i32, Model>, String> {
        let mut cache = ACTIVE_STATUS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed = mem::take(&mut env_cache.models);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    async fn reset(&self, env: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_STATUS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&env) {
            *env_cache = CacheStatus::new();
        }

        Ok(())
    }
}

impl<R> OrderStatus<R> {
    // Helper específico de dominio (status enum)
    pub async fn get_by_status(&self, env: Environments, status: Status) -> Option<Model> {
        let cache = ACTIVE_STATUS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        env_cache
            .models
            .values()
            .find(|m| Status::from_model(m) == status)
            .cloned()
    }
}
