use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use migration::async_trait::async_trait;
use models::{
    entities::pairs::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState},
    structs::{CachePairs, CachePairsEnvironments, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Pairs, utils::EntityCache};

static ACTIVE_PAIRS: Lazy<Arc<RwLock<CachePairsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CachePairsEnvironments::new())));

#[async_trait]
impl<R> EntityCache<Environments> for Pairs<R>
where
    R: Send + Sync,
{
    type Key = i32;
    type Value = Model;
    type Collection = CachePairs;

    // ============================
    // Lifecycle
    // ============================

    async fn state(&self, env: Environments) -> LifecycleState {
        let cache = ACTIVE_PAIRS.read().await;
        cache
            .get(&env)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    async fn set_state(&self, env: Environments, state: LifecycleState) -> Result<(), String> {
        let mut cache = ACTIVE_PAIRS.write().await;
        let env_cache = cache.get_or_create(env);

        transition_with_timestamp(env_cache, state)?;
        Ok(())
    }

    // ============================
    // Queries
    // ============================

    async fn get_all(&self, env: Environments) -> Option<CachePairs> {
        let cache = ACTIVE_PAIRS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        Some(env_cache.clone())
    }

    async fn get(&self, env: Environments, key: i32) -> Option<Model> {
        let cache = ACTIVE_PAIRS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        env_cache.models.get(&key).cloned()
    }

    // ============================
    // Mutations
    // ============================

    async fn set_all(&self, env: Environments, values: Vec<Model>) -> Result<(), String> {
        let mut cache = ACTIVE_PAIRS.write().await;
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
        let mut cache = ACTIVE_PAIRS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(key, value);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    async fn remove(&self, env: Environments, key: i32) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_PAIRS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache.models.remove(&key);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    async fn remove_all(&self, env: Environments) -> Result<HashMap<i32, Model>, String> {
        let mut cache = ACTIVE_PAIRS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed = mem::take(&mut env_cache.models);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    async fn reset(&self, env: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_PAIRS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&env) {
            *env_cache = CachePairs::new();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use models::{entities::pairs::Model, enums::LifecycleState, structs::Environments};

    use crate::{handler::Pairs, utils::EntityCache};

    /* =========================================================
     * Helpers
     * ========================================================= */

    fn mock_pair(id: i32) -> Model {
        Model {
            id,
            ..Default::default()
        }
    }

    async fn new_service() -> Pairs<()> {
        Pairs::blank()
    }

    async fn start_env(service: &Pairs<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Starting)
            .await
            .unwrap();
        service
            .set_state(env, LifecycleState::Running)
            .await
            .unwrap();
    }

    async fn stop_env(service: &Pairs<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert!(removed.is_empty() || !removed.is_empty());

        service.set_state(env, LifecycleState::Off).await.unwrap();
    }

    /* =========================================================
     * Scenarios
     * ========================================================= */

    #[tokio::test]
    async fn cache_pairs_unit_responsibilities() {
        let env = Environments::DEV;
        let service = new_service().await;

        // reset clears state
        start_env(&service, env).await;

        service.set_all(env, vec![mock_pair(1)]).await.unwrap();

        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert_eq!(removed.len(), 1);

        service.set_state(env, LifecycleState::Off).await.unwrap();

        assert_eq!(service.state(env).await, LifecycleState::Off);

        // set / get
        start_env(&service, env).await;

        service
            .set_all(env, vec![mock_pair(1), mock_pair(2)])
            .await
            .unwrap();

        let cache = service.get_all(env).await.unwrap();
        assert_eq!(cache.models.len(), 2);

        // upsert
        service.upsert(env, 10, mock_pair(10)).await.unwrap();
        assert!(service.get(env, 10).await.is_some());

        // remove
        service.remove(env, 10).await.unwrap();
        assert!(service.get(env, 10).await.is_none());

        // cannot remove_all while running
        assert!(service.remove_all(env).await.is_err());

        stop_env(&service, env).await;
    }
}
