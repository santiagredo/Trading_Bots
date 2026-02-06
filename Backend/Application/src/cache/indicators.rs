use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use migration::async_trait::async_trait;
use models::{
    entities::indicators::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState},
    structs::{CacheIndicators, CacheIndicatorsEnvironments, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Indicators, utils::EntityCache};

/* =========================================================
 * Static cache
 * ========================================================= */

static ACTIVE_INDICATORS: Lazy<Arc<RwLock<CacheIndicatorsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheIndicatorsEnvironments::new())));

#[async_trait]
impl<R> EntityCache<Environments> for Indicators<R>
where
    R: Send + Sync,
{
    type Key = i32;
    type Value = Model;
    type Collection = CacheIndicators;

    async fn state(&self, env: Environments) -> LifecycleState {
        let cache = ACTIVE_INDICATORS.read().await;
        cache
            .get(&env)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    async fn set_state(&self, env: Environments, state: LifecycleState) -> Result<(), String> {
        let mut cache = ACTIVE_INDICATORS.write().await;
        let env_cache = cache.get_or_create(env);

        transition_with_timestamp(env_cache, state)?;
        Ok(())
    }

    async fn get_all(&self, env: Environments) -> Option<CacheIndicators> {
        let cache = ACTIVE_INDICATORS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        Some(env_cache.clone())
    }

    async fn get(&self, env: Environments, key: i32) -> Option<Model> {
        let cache = ACTIVE_INDICATORS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        env_cache.models.get(&key).cloned()
    }

    async fn set_all(&self, env: Environments, values: Vec<Model>) -> Result<(), String> {
        let mut cache = ACTIVE_INDICATORS.write().await;
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
        let mut cache = ACTIVE_INDICATORS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(key, value);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    async fn remove(&self, env: Environments, key: i32) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_INDICATORS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache.models.remove(&key);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    async fn remove_all(&self, env: Environments) -> Result<HashMap<i32, Model>, String> {
        let mut cache = ACTIVE_INDICATORS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed = mem::take(&mut env_cache.models);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    async fn reset(&self, env: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_INDICATORS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&env) {
            *env_cache = CacheIndicators::new();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::{entities::indicators::Model, enums::LifecycleState, structs::Environments};

    use crate::handler::Indicators;

    fn mock_indicator(strategy_id: i32) -> Model {
        Model {
            id: strategy_id,
            strategy_id,
            is_active: true,
            ..Default::default()
        }
    }

    async fn new_service() -> Indicators<()> {
        Indicators::blank()
    }

    async fn start_env(service: &Indicators<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Starting)
            .await
            .unwrap();

        service
            .set_state(env, LifecycleState::Running)
            .await
            .unwrap();
    }

    async fn stop_env(service: &Indicators<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert!(removed.is_empty() || !removed.is_empty());

        service.set_state(env, LifecycleState::Off).await.unwrap();
    }

    #[tokio::test]
    async fn cache_indicators_unit_responsibilities() {
        let env = Environments::DEV;
        let service = new_service().await;

        /* =========================
         * RESET / INITIAL STATE
         * ========================= */

        start_env(&service, env).await;

        service.set_all(env, vec![mock_indicator(1)]).await.unwrap();

        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert_eq!(removed.len(), 1);

        service.set_state(env, LifecycleState::Off).await.unwrap();
        assert_eq!(service.state(env).await, LifecycleState::Off);

        /* =========================
         * SET / GET
         * ========================= */

        start_env(&service, env).await;

        service
            .set_all(env, vec![mock_indicator(1), mock_indicator(2)])
            .await
            .unwrap();

        let cache = service.get_all(env).await.unwrap();
        assert_eq!(cache.models.len(), 2);

        /* =========================
         * UPSERT
         * ========================= */

        service.upsert(env, 10, mock_indicator(10)).await.unwrap();
        let single = service.get(env, 10).await.unwrap();
        assert_eq!(single.strategy_id, 10);

        /* =========================
         * REMOVE
         * ========================= */

        service.remove(env, 10).await.unwrap();
        assert!(service.get(env, 10).await.is_none());

        /* =========================
         * INVALID OPERATION
         * ========================= */

        assert!(service.remove_all(env).await.is_err());

        /* =========================
         * STOP
         * ========================= */

        stop_env(&service, env).await;
    }
}
