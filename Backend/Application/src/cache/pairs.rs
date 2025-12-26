use std::{collections::HashMap, sync::Arc};

use models::{entities::pairs::Model, structs::Environments};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Pairs, utils::Cache};

#[derive(Default)]
struct CacheEnvironments {
    pub environments: HashMap<Environments, CachePairs>,
}

#[derive(Default)]
struct CachePairs {
    pub is_initialized: bool,
    pub models: HashMap<i32, Model>,
}

static ACTIVE_PAIRS: Lazy<Arc<RwLock<CacheEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheEnvironments::default())));

impl Pairs<Cache> {
    pub async fn set_active_pairs_cache(
        environment: &Environments,
        pairs: Vec<Model>,
    ) -> Vec<Model> {
        let mut cache_pairs = ACTIVE_PAIRS.write().await;

        let env_map = cache_pairs
            .environments
            .entry(*environment)
            .or_insert_with(CachePairs::default);

        for pair in pairs.iter() {
            env_map.models.insert(pair.id, pair.clone());
        }

        env_map.is_initialized = true;

        pairs
    }

    pub async fn set_active_pair_cache(
        environment: &Environments,
        pair: Model,
        is_remove: bool,
    ) -> Model {
        let mut cache_pairs = ACTIVE_PAIRS.write().await;

        let env_map = cache_pairs
            .environments
            .entry(*environment)
            .or_insert_with(CachePairs::default);

        if is_remove {
            env_map.models.remove(&pair.id).unwrap_or(pair)
        } else {
            env_map.models.insert(pair.id, pair.clone());
            pair
        }
    }

    pub async fn get_active_pairs_cache(environment: &Environments) -> Option<HashMap<i32, Model>> {
        let cache_pairs = ACTIVE_PAIRS.read().await;

        let env_map = cache_pairs.environments.get(environment)?;

        Some(env_map.models.clone())
    }

    pub async fn get_active_pair_cache(environment: &Environments, key: &i32) -> Option<Model> {
        let cache_pairs = ACTIVE_PAIRS.read().await;

        let env_map = cache_pairs.environments.get(environment)?;
        let pair = env_map.models.get(key)?;

        Some(pair.clone())
    }

    pub async fn get_active_pairs_status_cache(environment: &Environments) -> bool {
        let cache_pairs = ACTIVE_PAIRS.read().await;

        cache_pairs
            .environments
            .get(environment)
            .map(|val| val.is_initialized)
            .unwrap_or(false)
    }

    pub async fn stop_active_pairs_cache(environment: &Environments) {
        let mut cache_pairs = ACTIVE_PAIRS.write().await;

        let Some(env_map) = cache_pairs.environments.get_mut(environment) else {
            return;
        };

        env_map.models = HashMap::new();
        env_map.is_initialized = false;
    }
}

#[cfg(test)]
mod tests {
    use models::{entities::pairs::Model, structs::Environments};

    use crate::{handler::Pairs, utils::Cache};

    // Helpers
    fn mock_pair(id: i32) -> Model {
        Model {
            id,
            ..Default::default()
        }
    }

    async fn reset_env(env: Environments) {
        Pairs::<Cache>::stop_active_pairs_cache(&env).await;
    }

    // Scenarios (unit responsibilities)
    async fn scenario_reset_env_clears_state(env: Environments) {
        let pairs = vec![mock_pair(1), mock_pair(2)];

        Pairs::<Cache>::set_active_pairs_cache(&env, pairs).await;
        reset_env(env).await;

        let cache = Pairs::<Cache>::get_active_pairs_cache(&env)
            .await
            .expect("environment entry should exist");

        let status = Pairs::<Cache>::get_active_pairs_status_cache(&env).await;

        assert!(cache.is_empty());
        assert!(!status);
    }

    async fn scenario_set_active_pairs_cache(env: Environments) {
        reset_env(env).await;

        let pairs = vec![mock_pair(1), mock_pair(2)];

        Pairs::<Cache>::set_active_pairs_cache(&env, pairs).await;

        let cache = Pairs::<Cache>::get_active_pairs_cache(&env)
            .await
            .expect("cache should exist");

        let status = Pairs::<Cache>::get_active_pairs_status_cache(&env).await;

        assert_eq!(cache.len(), 2);
        assert!(cache.contains_key(&1));
        assert!(cache.contains_key(&2));
        assert!(status);

        reset_env(env).await;
    }

    async fn scenario_set_active_pair_cache_insert(env: Environments) {
        reset_env(env).await;

        let pair = mock_pair(10);

        Pairs::<Cache>::set_active_pair_cache(&env, pair.clone(), false).await;

        let cached = Pairs::<Cache>::get_active_pair_cache(&env, &10).await;

        assert_eq!(cached, Some(pair));

        reset_env(env).await;
    }

    async fn scenario_set_active_pair_cache_remove(env: Environments) {
        reset_env(env).await;

        let pair = mock_pair(20);

        Pairs::<Cache>::set_active_pair_cache(&env, pair.clone(), false).await;
        Pairs::<Cache>::set_active_pair_cache(&env, pair, true).await;

        let cached = Pairs::<Cache>::get_active_pair_cache(&env, &20).await;

        assert!(cached.is_none());

        reset_env(env).await;
    }

    async fn scenario_get_active_pairs_cache(env: Environments) {
        reset_env(env).await;

        let pairs = vec![mock_pair(1), mock_pair(2), mock_pair(3)];

        Pairs::<Cache>::set_active_pairs_cache(&env, pairs).await;

        let cache = Pairs::<Cache>::get_active_pairs_cache(&env)
            .await
            .expect("cache should exist");

        assert_eq!(cache.len(), 3);

        reset_env(env).await;
    }

    async fn scenario_get_active_pair_cache(env: Environments) {
        reset_env(env).await;

        let pair = mock_pair(42);

        Pairs::<Cache>::set_active_pair_cache(&env, pair.clone(), false).await;

        let cached = Pairs::<Cache>::get_active_pair_cache(&env, &42).await;

        assert_eq!(cached, Some(pair));

        reset_env(env).await;
    }

    async fn scenario_get_active_pairs_status_cache(env: Environments) {
        reset_env(env).await;

        let initial_status = Pairs::<Cache>::get_active_pairs_status_cache(&env).await;
        assert!(!initial_status);

        let pairs = vec![mock_pair(1)];
        Pairs::<Cache>::set_active_pairs_cache(&env, pairs).await;

        let status = Pairs::<Cache>::get_active_pairs_status_cache(&env).await;
        assert!(status);

        reset_env(env).await;
    }

    #[tokio::test]
    async fn cache_pairs_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_active_pairs_cache(env).await;
        scenario_set_active_pair_cache_insert(env).await;
        scenario_set_active_pair_cache_remove(env).await;
        scenario_get_active_pairs_cache(env).await;
        scenario_get_active_pair_cache(env).await;
        scenario_get_active_pairs_status_cache(env).await;

        // Final safety cleanup
        reset_env(env).await;
    }
}
