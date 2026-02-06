use std::{
    collections::{HashMap, HashSet},
    mem,
    sync::Arc,
};

use crate::handler::SubscribedIndicators;
use chrono::Local;
use models::{
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState},
    structs::{CacheSubscribedIndicators, CacheSubscribedIndicatorsEnvironments, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

/* =========================================================
 * Static cache
 * ========================================================= */

static ACTIVE_SUBSCRIBED_INDICATORS: Lazy<Arc<RwLock<CacheSubscribedIndicatorsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheSubscribedIndicatorsEnvironments::new())));

impl<R> SubscribedIndicators<R>
where
    R: Send + Sync,
{
    pub async fn state(&self, env: Environments) -> LifecycleState {
        let cache = ACTIVE_SUBSCRIBED_INDICATORS.read().await;
        cache
            .get(&env)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    pub async fn set_state(&self, env: Environments, state: LifecycleState) -> Result<(), String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;
        let env_cache = cache.get_or_create(env);

        transition_with_timestamp(env_cache, state)?;
        Ok(())
    }

    pub async fn get_all(&self, env: Environments) -> Option<CacheSubscribedIndicators> {
        let cache = ACTIVE_SUBSCRIBED_INDICATORS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        Some(env_cache.clone())
    }

    pub async fn get(&self, env: Environments, key: String) -> Option<Vec<i32>> {
        let cache = ACTIVE_SUBSCRIBED_INDICATORS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        env_cache
            .models
            .get(&key)
            .map(|v| v.iter().copied().collect())
    }

    pub async fn set_all(
        &self,
        env: Environments,
        values: Vec<(String, Vec<i32>)>,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let mut models: HashMap<String, HashSet<i32>> = HashMap::new();

        for (symbol, strategy_ids) in values {
            models
                .entry(symbol)
                .or_insert_with(HashSet::new)
                .extend(strategy_ids);
        }

        env_cache.models = models;
        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn upsert(&self, env: Environments, key: String, value: Vec<i32>) -> Result<(), String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache
            .models
            .entry(key)
            .or_insert_with(HashSet::new)
            .extend(value);

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn remove(&self, env: Environments, key: String) -> Result<Option<Vec<i32>>, String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache
            .models
            .remove(&key)
            .map(|set| set.into_iter().collect());

        env_cache.last_update_date = Local::now().naive_local();
        Ok(removed)
    }

    pub async fn remove_all(&self, env: Environments) -> Result<HashMap<String, Vec<i32>>, String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed = mem::take(&mut env_cache.models)
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();

        env_cache.last_update_date = Local::now().naive_local();
        Ok(removed)
    }

    pub async fn reset(&self, env: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&env) {
            *env_cache = CacheSubscribedIndicators::new();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use models::{enums::LifecycleState, structs::Environments};

    use crate::handler::SubscribedIndicators;

    fn mock_values() -> Vec<(String, Vec<i32>)> {
        vec![
            ("BTCUSDT".to_string(), vec![1, 2]),
            ("ETHUSDT".to_string(), vec![3]),
        ]
    }

    fn service() -> SubscribedIndicators<()> {
        SubscribedIndicators::blank()
    }

    async fn start_env(service: &SubscribedIndicators<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Starting)
            .await
            .unwrap();

        service
            .set_state(env, LifecycleState::Running)
            .await
            .unwrap();
    }

    async fn stop_env(service: &SubscribedIndicators<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert!(!removed.is_empty());

        service.set_state(env, LifecycleState::Off).await.unwrap();
    }

    #[tokio::test]
    async fn subscribed_indicators_cache_should_behave_correctly() {
        let env = Environments::DEV;
        let service = service();

        /* =========================
         * SET / GET ALL
         * ========================= */

        start_env(&service, env).await;

        service.set_all(env, mock_values()).await.unwrap();

        let cache = service.get_all(env).await.unwrap();
        assert_eq!(cache.models.len(), 2);

        assert_eq!(cache.models.get("BTCUSDT"), Some(&HashSet::from([1, 2])));

        assert_eq!(cache.models.get("ETHUSDT"), Some(&HashSet::from([3])));

        /* =========================
         * GET (single key)
         * ========================= */

        let btc = service.get(env, "BTCUSDT".into()).await.unwrap();
        assert_eq!(HashSet::<i32>::from_iter(btc), HashSet::from([1, 2]));

        /* =========================
         * UPSERT
         * ========================= */

        service
            .upsert(env, "BTCUSDT".into(), vec![10, 2])
            .await
            .unwrap();

        let btc = service.get(env, "BTCUSDT".into()).await.unwrap();
        assert_eq!(HashSet::<i32>::from_iter(btc), HashSet::from([1, 2, 10]));

        /* =========================
         * INVALID OPERATION
         * ========================= */

        assert!(service.remove_all(env).await.is_err());

        /* =========================
         * STOP
         * ========================= */

        stop_env(&service, env).await;

        assert_eq!(service.state(env).await, LifecycleState::Off);
        assert!(service.get_all(env).await.is_none());
    }
}
