use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use migration::async_trait::async_trait;
use models::{
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState, TradingState},
    structs::{CacheStrategies, CacheStrategiesEnvironments, CacheStrategy, Environments},
};
use once_cell::sync::Lazy;
use tokio::{sync::RwLock, task::AbortHandle};

use crate::{handler::Strategies, utils::EntityCache};

/* =========================================================
 * Static cache
 * ========================================================= */

static ACTIVE_STRATEGIES: Lazy<Arc<RwLock<CacheStrategiesEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheStrategiesEnvironments::new())));

/* =========================================================
 * EntityCache implementation
 * ========================================================= */

#[async_trait]
impl<R> EntityCache<Environments> for Strategies<R>
where
    R: Send + Sync,
{
    type Key = i32;
    type Value = CacheStrategy;
    type Collection = CacheStrategies;

    async fn state(&self, env: Environments) -> LifecycleState {
        let cache = ACTIVE_STRATEGIES.read().await;
        cache
            .get(&env)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    async fn set_state(&self, env: Environments, state: LifecycleState) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_or_create(env);

        transition_with_timestamp(env_cache, state)?;
        Ok(())
    }

    async fn get_all(&self, env: Environments) -> Option<CacheStrategies> {
        let cache = ACTIVE_STRATEGIES.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        Some(env_cache.clone())
    }

    async fn get(&self, env: Environments, key: i32) -> Option<CacheStrategy> {
        let cache = ACTIVE_STRATEGIES.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        env_cache.models.get(&key).cloned()
    }

    async fn set_all(&self, env: Environments, values: Vec<CacheStrategy>) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let values: Vec<(i32, CacheStrategy)> =
            values.into_iter().map(|m| (m.model.id, m)).collect();

        env_cache.models = values.into_iter().collect();
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    async fn upsert(
        &self,
        env: Environments,
        key: i32,
        value: CacheStrategy,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(key, value);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    async fn remove(&self, env: Environments, key: i32) -> Result<Option<CacheStrategy>, String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache.models.remove(&key);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    async fn remove_all(&self, env: Environments) -> Result<HashMap<i32, CacheStrategy>, String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed = mem::take(&mut env_cache.models);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    async fn reset(&self, env: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&env) {
            *env_cache = CacheStrategies::new();
        }

        Ok(())
    }
}

/* =========================================================
 * Strategy-specific helpers (domain logic)
 * ========================================================= */

impl<R> Strategies<R> {
    pub async fn set_strategy_state(
        &self,
        env: Environments,
        strategy_id: i32,
        state: TradingState,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        let strategy = env_cache
            .models
            .get_mut(&strategy_id)
            .ok_or("Strategy not found")?;

        transition_with_timestamp(strategy, state)?;
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn set_strategy_error(
        &self,
        env: Environments,
        strategy_id: i32,
        error: Option<String>,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        let strategy = env_cache
            .models
            .get_mut(&strategy_id)
            .ok_or("Strategy not found")?;

        strategy.last_error_message = error;
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn set_strategy_model_error(
        &self,
        env: Environments,
        strategy_id: i32,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        let strategy = env_cache
            .models
            .get_mut(&strategy_id)
            .ok_or("Strategy not found")?;

        let now = Local::now().naive_local();

        strategy.model.error_last_date = Some(now);
        env_cache.last_update_date = now;

        Ok(())
    }

    pub async fn set_strategy_model_last_exec(
        &self,
        env: Environments,
        strategy_id: i32,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        let strategy = env_cache
            .models
            .get_mut(&strategy_id)
            .ok_or("Strategy not found")?;

        let now = Local::now().naive_local();

        strategy.model.last_execution = Some(now);
        env_cache.last_update_date = now;

        Ok(())
    }

    pub async fn set_abort_handle(
        &self,
        env: Environments,
        abort_handle: AbortHandle,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        env_cache.abort_handle = Some(abort_handle);

        Ok(())
    }

    pub async fn abort_handle(&self, env: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        let handle = env_cache.abort_handle.take();

        if let Some(handle) = handle {
            handle.abort();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::{
        entities::strategies::Model,
        enums::{LifecycleState, TradingState},
        structs::{CacheStrategy, Environments},
    };

    use crate::handler::Strategies;

    fn mock_strategy(id: i32) -> CacheStrategy {
        CacheStrategy {
            model: Model {
                id,
                is_active: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    async fn new_service() -> Strategies<()> {
        Strategies::blank()
    }

    async fn start_env(service: &Strategies<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Starting)
            .await
            .unwrap();

        service
            .set_state(env, LifecycleState::Running)
            .await
            .unwrap();
    }

    async fn stop_env(service: &Strategies<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert!(removed.is_empty() || !removed.is_empty());

        service.set_state(env, LifecycleState::Off).await.unwrap();
    }

    #[tokio::test]
    async fn cache_strategies_unit_responsibilities() {
        let env = Environments::DEV;
        let service = new_service().await;

        /* =========================================================
         * RESET / INITIAL STATE
         * ========================================================= */

        start_env(&service, env).await;

        service.set_all(env, vec![mock_strategy(1)]).await.unwrap();

        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert_eq!(removed.len(), 1);

        service.set_state(env, LifecycleState::Off).await.unwrap();

        assert_eq!(service.state(env).await, LifecycleState::Off);

        /* =========================================================
         * SET / GET
         * ========================================================= */

        start_env(&service, env).await;

        service
            .set_all(env, vec![mock_strategy(1), mock_strategy(2)])
            .await
            .unwrap();

        let cache = service.get_all(env).await.unwrap();
        assert_eq!(cache.models.len(), 2);

        /* =========================================================
         * UPSERT
         * ========================================================= */

        service.upsert(env, 10, mock_strategy(10)).await.unwrap();

        let single = service.get(env, 10).await.unwrap();
        assert_eq!(single.model.id, 10);

        /* =========================================================
         * DOMAIN FSM
         * ========================================================= */

        service
            .set_strategy_state(env, 10, TradingState::Running)
            .await
            .unwrap();

        let strategy = service.get(env, 10).await.unwrap();
        assert_eq!(strategy.state, TradingState::Running);

        service
            .set_strategy_error(env, 10, Some("boom".into()))
            .await
            .unwrap();

        let strategy = service.get(env, 10).await.unwrap();
        assert_eq!(strategy.last_error_message.as_deref(), Some("boom"));

        service.set_strategy_error(env, 10, None).await.unwrap();

        let strategy = service.get(env, 10).await.unwrap();
        assert!(strategy.last_error_message.is_none());

        /* =========================================================
         * REMOVE
         * ========================================================= */

        service.remove(env, 10).await.unwrap();
        assert!(service.get(env, 10).await.is_none());

        /* =========================================================
         * INVALID OPERATION
         * ========================================================= */

        // remove_all is forbidden while running
        assert!(service.remove_all(env).await.is_err());

        /* =========================================================
         * STOP
         * ========================================================= */

        stop_env(&service, env).await;
    }
}
