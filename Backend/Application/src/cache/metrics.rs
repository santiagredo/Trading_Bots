use std::{collections::HashMap, mem, sync::Arc, time::Duration};

use chrono::Local;
use migration::async_trait::async_trait;
use models::{
    entities::critical_metrics::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState},
    structs::{CacheMetrics, CacheMetricsEnvironments, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Metrics, utils::EntityCache};

static ACTIVE_METRICS: Lazy<Arc<RwLock<CacheMetricsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheMetricsEnvironments::new())));

#[async_trait]
impl<R> EntityCache<Environments> for Metrics<R>
where
    R: Send + Sync,
{
    type Key = ();
    type Value = Model;
    type Collection = CacheMetrics;

    async fn state(&self, env: Environments) -> LifecycleState {
        let cache = ACTIVE_METRICS.read().await;
        cache
            .get(&env)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    async fn set_state(&self, env: Environments, state: LifecycleState) -> Result<(), String> {
        let mut cache = ACTIVE_METRICS.write().await;
        let env_cache = cache.get_or_create(env);

        transition_with_timestamp(env_cache, state)?;
        Ok(())
    }

    async fn get_all(&self, env: Environments) -> Option<CacheMetrics> {
        let cache = ACTIVE_METRICS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        Some(env_cache.clone())
    }

    async fn get(&self, env: Environments, _key: ()) -> Option<Model> {
        let cache = ACTIVE_METRICS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        Some(env_cache.model.clone())
    }

    async fn set_all(&self, env: Environments, values: Vec<Model>) -> Result<(), String> {
        let mut cache = ACTIVE_METRICS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        if let Some(model) = values.into_iter().next() {
            env_cache.model = model;
            env_cache.last_update_date = Local::now().naive_local();
        }

        Ok(())
    }

    async fn upsert(&self, env: Environments, _key: (), value: Model) -> Result<(), String> {
        let mut cache = ACTIVE_METRICS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.model = value;
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    async fn remove(&self, env: Environments, _key: ()) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_METRICS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = mem::take(&mut env_cache.model);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(Some(removed))
    }

    async fn remove_all(&self, env: Environments) -> Result<HashMap<(), Model>, String> {
        let mut cache = ACTIVE_METRICS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed = mem::take(&mut env_cache.model);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(HashMap::from([((), removed)]))
    }

    async fn reset(&self, env: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_METRICS.write().await;

        if let Some(env_cache) = cache.get_mut(&env) {
            *env_cache = CacheMetrics::new();
        }

        Ok(())
    }
}

impl<R> Metrics<R>
where
    R: Send + Sync,
{
    pub async fn set_execution_metrics(&self, env: Environments, elapsed: Duration, success: bool) {
        let mut cache = ACTIVE_METRICS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return;
        }

        let now = chrono::Local::now().naive_local();

        if success {
            env_cache.model.executions_ok += 1;
            env_cache.model.consecutive_errors = 0;
            env_cache.model.last_success = Some(now);
        } else {
            env_cache.model.executions_err += 1;
            env_cache.model.consecutive_errors += 1;
            env_cache.model.last_error = Some(now);
        }

        env_cache.model.total_execution_time += elapsed.as_millis().try_into().unwrap_or(0);

        if elapsed.as_millis().try_into().unwrap_or(-1) > env_cache.model.max_execution_time {
            env_cache.model.max_execution_time = elapsed.as_millis().try_into().unwrap_or(-1);
        }
    }

    pub async fn set_posting_metrics(&self, env: Environments, increase: bool) {
        let mut cache = ACTIVE_METRICS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return;
        }

        if increase {
            env_cache.model.active_posting += 1;

            env_cache.model.max_active_posting = env_cache
                .model
                .max_active_posting
                .max(env_cache.model.active_posting);

            return;
        }

        if env_cache.model.active_posting > 0 {
            env_cache.model.active_posting -= 1
        }
    }

    pub async fn set_skipped_metrics(&self, env: Environments) {
        let mut cache = ACTIVE_METRICS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return;
        }

        env_cache.model.skipped_due_to_lock += 1;
    }
}
