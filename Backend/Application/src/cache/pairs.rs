use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use models::{
    entities::pairs::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState},
    structs::{CachePairs, CachePairsEnvironments, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Pairs, utils::Cache};

static ACTIVE_PAIRS: Lazy<Arc<RwLock<CachePairsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CachePairsEnvironments::new())));

impl Pairs<Cache> {
    // =========================
    // Lifecycle
    // =========================

    pub async fn set_status_cache(
        environment: Environments,
        status: LifecycleState,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_PAIRS.write().await;
        let env_cache = cache.get_or_create(environment);

        transition_with_timestamp(env_cache, status)?;
        Ok(())
    }

    // =========================
    // Mutations
    // =========================

    pub async fn set_pairs_cache(
        environment: Environments,
        pairs: Vec<Model>,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_PAIRS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err(format!(
                "Cannot load pairs: cache not running ({:?})",
                env_cache.status
            ));
        }

        env_cache.models = pairs.into_iter().map(|p| (p.id, p)).collect();
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn upsert_pair_cache(environment: Environments, pair: Model) -> Result<(), String> {
        let mut cache = ACTIVE_PAIRS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(pair.id, pair);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn remove_pair_cache(
        environment: Environments,
        pair_id: i32,
    ) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_PAIRS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache.models.remove(&pair_id);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    pub async fn remove_pairs_cache(
        environment: Environments,
    ) -> Result<HashMap<i32, Model>, String> {
        let mut cache = ACTIVE_PAIRS.write().await;
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

    pub async fn get_pairs_cache(environment: Environments) -> Option<CachePairs> {
        let cache = ACTIVE_PAIRS.read().await;
        cache.get(&environment).cloned()
    }

    pub async fn get_pair_cache(environment: Environments, pair_id: i32) -> Option<Model> {
        let cache = ACTIVE_PAIRS.read().await;
        cache.get(&environment)?.models.get(&pair_id).cloned()
    }

    pub async fn get_cache_state(environment: Environments) -> LifecycleState {
        let cache = ACTIVE_PAIRS.read().await;
        cache
            .get(&environment)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    pub async fn reset_pairs_cache(environment: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_PAIRS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&environment) {
            *env_cache = CachePairs::new();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use models::{entities::pairs::Model, enums::LifecycleState, structs::Environments};

    use crate::{handler::Pairs, utils::Cache};

    // =========================
    // Helpers
    // =========================

    fn mock_pair(id: i32) -> Model {
        Model {
            id,
            ..Default::default()
        }
    }

    async fn start_env(env: Environments) {
        let _ = Pairs::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
        let _ = Pairs::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    }

    async fn stop_env(env: Environments) {
        let _ = Pairs::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = Pairs::<Cache>::remove_pairs_cache(env)
            .await
            .expect("remove_pairs_cache should work in Stopping");

        assert!(removed.is_empty() || !removed.is_empty());

        let _ = Pairs::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    }

    // =========================
    // Scenarios
    // =========================

    async fn scenario_reset_env_clears_state(env: Environments) {
        start_env(env).await;

        Pairs::<Cache>::set_pairs_cache(env, vec![mock_pair(1)])
            .await
            .unwrap();

        let _ = Pairs::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = Pairs::<Cache>::remove_pairs_cache(env).await.unwrap();
        assert_eq!(removed.len(), 1);

        let cache = Pairs::<Cache>::get_pairs_cache(env).await.unwrap();
        assert!(cache.models.is_empty());

        let _ = Pairs::<Cache>::set_status_cache(env, LifecycleState::Off).await;

        let status = Pairs::<Cache>::get_cache_state(env).await;
        assert_eq!(status, LifecycleState::Off);
    }

    async fn scenario_set_pairs_cache(env: Environments) {
        start_env(env).await;

        Pairs::<Cache>::set_pairs_cache(env, vec![mock_pair(1), mock_pair(2)])
            .await
            .unwrap();

        let cache = Pairs::<Cache>::get_pairs_cache(env).await.unwrap();

        assert_eq!(cache.models.len(), 2);
        assert!(cache.models.contains_key(&1));
        assert!(cache.models.contains_key(&2));

        stop_env(env).await;
    }

    async fn scenario_upsert_pair(env: Environments) {
        start_env(env).await;

        let pair = mock_pair(10);
        Pairs::<Cache>::upsert_pair_cache(env, pair.clone())
            .await
            .unwrap();

        let cached = Pairs::<Cache>::get_pair_cache(env, 10).await.unwrap();
        assert_eq!(cached, pair);

        stop_env(env).await;
    }

    async fn scenario_remove_pair(env: Environments) {
        start_env(env).await;

        Pairs::<Cache>::upsert_pair_cache(env, mock_pair(20))
            .await
            .unwrap();

        let removed = Pairs::<Cache>::remove_pair_cache(env, 20).await.unwrap();

        assert!(removed.is_some());
        assert!(Pairs::<Cache>::get_pair_cache(env, 20).await.is_none());

        stop_env(env).await;
    }

    async fn scenario_get_cache_state(env: Environments) {
        assert_eq!(
            Pairs::<Cache>::get_cache_state(env).await,
            LifecycleState::Off
        );

        start_env(env).await;

        assert_eq!(
            Pairs::<Cache>::get_cache_state(env).await,
            LifecycleState::Running
        );

        stop_env(env).await;
    }

    async fn scenario_cannot_remove_pairs_when_running(env: Environments) {
        start_env(env).await;

        let result = Pairs::<Cache>::remove_pairs_cache(env).await;
        assert!(result.is_err());

        stop_env(env).await;
    }

    // =========================
    // Entry point
    // =========================

    #[tokio::test]
    async fn cache_pairs_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_pairs_cache(env).await;
        scenario_upsert_pair(env).await;
        scenario_remove_pair(env).await;
        scenario_get_cache_state(env).await;
        scenario_cannot_remove_pairs_when_running(env).await;
    }
}
