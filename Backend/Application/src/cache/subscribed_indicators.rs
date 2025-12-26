use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    sync::Arc,
};

use models::{entities::indicators::Model, structs::Environments};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::SubscribedIndicators, utils::Cache};

#[derive(Default)]
struct CacheEnvironments {
    pub environments: HashMap<Environments, CacheSubscribedIndicators>,
}

#[derive(Default)]
struct CacheSubscribedIndicators {
    pub is_initialized: bool,
    pub models: HashMap<String, HashSet<i32>>,
}

static ACTIVE_SUBSCRIBED_INDICATORS: Lazy<Arc<RwLock<CacheEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheEnvironments::default())));

impl SubscribedIndicators<Cache> {
    pub async fn set_active_subscribed_indicators_cache(
        environment: &Environments,
        indicators_map: HashMap<String, HashSet<i32>>,
    ) -> HashMap<String, HashSet<i32>> {
        let mut cache_indicators = ACTIVE_SUBSCRIBED_INDICATORS.write().await;

        let env_map = cache_indicators
            .environments
            .entry(*environment)
            .or_insert_with(CacheSubscribedIndicators::default);

        for (ticker, indicators_id) in &indicators_map {
            env_map
                .models
                .insert(ticker.to_string(), indicators_id.clone());
        }

        env_map.is_initialized = true;

        indicators_map
    }

    pub async fn set_active_subscribed_indicator_cache(
        environment: &Environments,
        indicator: Model,
        is_remove: bool,
    ) -> Model {
        let mut cache_indicators = ACTIVE_SUBSCRIBED_INDICATORS.write().await;

        let Some(env_map) = cache_indicators.environments.get_mut(environment) else {
            return indicator;
        };

        match env_map.models.entry(indicator.symbol.clone()) {
            Entry::Occupied(mut entry) if is_remove => {
                entry.get_mut().remove(&indicator.strategy_id);

                if entry.get().is_empty() {
                    entry.remove_entry();
                }
            }
            Entry::Occupied(mut entry) if !is_remove => {
                entry.get_mut().insert(indicator.strategy_id);
            }
            Entry::Vacant(entry) if !is_remove => {
                entry.insert([indicator.strategy_id].into_iter().collect());
            }
            _ => {}
        }

        indicator
    }

    pub async fn get_active_subscribed_indicators_cache(
        environment: &Environments,
    ) -> Option<HashMap<String, HashSet<i32>>> {
        let cache_indicators = ACTIVE_SUBSCRIBED_INDICATORS.read().await;

        let env_map = cache_indicators.environments.get(&environment)?;

        Some(env_map.models.clone())
    }

    pub async fn get_active_subscribed_indicator_cache(
        environment: &Environments,
        key: String,
    ) -> Option<HashSet<i32>> {
        let cache_indicators = ACTIVE_SUBSCRIBED_INDICATORS.read().await;

        let env_map = cache_indicators.environments.get(&environment)?;

        let cache_indicator = env_map.models.get(&key)?;

        Some(cache_indicator.clone())
    }

    pub async fn get_active_subscribed_indicators_status_cache(environment: &Environments) -> bool {
        let cache_indicators = ACTIVE_SUBSCRIBED_INDICATORS.read().await;

        cache_indicators
            .environments
            .get(&environment)
            .map(|val| val.is_initialized)
            .unwrap_or(false)
    }

    pub async fn stop_active_subscribed_indicators_cache(environment: &Environments) {
        let mut cache_indicators = ACTIVE_SUBSCRIBED_INDICATORS.write().await;

        let Some(env_map) = cache_indicators.environments.get_mut(environment) else {
            return;
        };

        env_map.models = HashMap::new();

        env_map.is_initialized = false;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use models::{entities::indicators::Model, structs::Environments};

    use crate::{handler::SubscribedIndicators, utils::Cache};

    // Helpers
    fn mock_model(symbol: &str, strategy_id: i32) -> Model {
        Model {
            id: strategy_id,
            strategy_id,
            symbol: symbol.to_string(),
            ..Default::default()
        }
    }

    fn mock_map() -> HashMap<String, HashSet<i32>> {
        HashMap::from([
            ("BTCUSDT".to_string(), HashSet::from([1, 2])),
            ("ETHUSDT".to_string(), HashSet::from([3])),
        ])
    }

    async fn reset_env(env: Environments) {
        SubscribedIndicators::<Cache>::stop_active_subscribed_indicators_cache(&env).await;
    }

    // Scenarios (unit responsibilities)

    // Verifies that reset_env fully clears cache state
    async fn scenario_reset_env_clears_state(env: Environments) {
        let map = mock_map();

        SubscribedIndicators::<Cache>::set_active_subscribed_indicators_cache(&env, map).await;
        reset_env(env).await;

        let cache = SubscribedIndicators::<Cache>::get_active_subscribed_indicators_cache(&env)
            .await
            .expect("environment entry should exist");

        let status =
            SubscribedIndicators::<Cache>::get_active_subscribed_indicators_status_cache(&env)
                .await;

        assert!(cache.is_empty());
        assert!(!status);
    }

    // Verifies bulk insertion and initialization flag
    async fn scenario_set_active_subscribed_indicators_cache(env: Environments) {
        reset_env(env).await;

        let map = mock_map();

        SubscribedIndicators::<Cache>::set_active_subscribed_indicators_cache(&env, map.clone())
            .await;

        let cache = SubscribedIndicators::<Cache>::get_active_subscribed_indicators_cache(&env)
            .await
            .expect("cache should exist");

        let status =
            SubscribedIndicators::<Cache>::get_active_subscribed_indicators_status_cache(&env)
                .await;

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get("BTCUSDT").unwrap().len(), 2);
        assert_eq!(cache.get("ETHUSDT").unwrap().len(), 1);
        assert!(status);

        reset_env(env).await;
    }

    // Verifies single indicator insertion
    async fn scenario_set_active_subscribed_indicator_insert(env: Environments) {
        reset_env(env).await;

        let model = mock_model("BTCUSDT", 10);

        SubscribedIndicators::<Cache>::set_active_subscribed_indicator_cache(
            &env,
            model.clone(),
            false,
        )
        .await;

        let cached = SubscribedIndicators::<Cache>::get_active_subscribed_indicator_cache(
            &env,
            "BTCUSDT".to_string(),
        )
        .await
        .unwrap();

        assert!(cached.contains(&10));

        reset_env(env).await;
    }

    // Verifies multiple inserts under same symbol
    async fn scenario_multiple_indicators_same_symbol(env: Environments) {
        reset_env(env).await;

        let m1 = mock_model("BTCUSDT", 1);
        let m2 = mock_model("BTCUSDT", 2);

        SubscribedIndicators::<Cache>::set_active_subscribed_indicator_cache(&env, m1, false).await;
        SubscribedIndicators::<Cache>::set_active_subscribed_indicator_cache(&env, m2, false).await;

        let cached = SubscribedIndicators::<Cache>::get_active_subscribed_indicator_cache(
            &env,
            "BTCUSDT".to_string(),
        )
        .await
        .unwrap();

        assert_eq!(cached.len(), 2);
        assert!(cached.contains(&1));
        assert!(cached.contains(&2));

        reset_env(env).await;
    }

    // Verifies indicator removal
    async fn scenario_set_active_subscribed_indicator_remove(env: Environments) {
        reset_env(env).await;

        let model = mock_model("ETHUSDT", 99);

        SubscribedIndicators::<Cache>::set_active_subscribed_indicator_cache(
            &env,
            model.clone(),
            false,
        )
        .await;

        SubscribedIndicators::<Cache>::set_active_subscribed_indicator_cache(
            &env,
            model.clone(),
            true,
        )
        .await;

        let cached = SubscribedIndicators::<Cache>::get_active_subscribed_indicator_cache(
            &env,
            "ETHUSDT".to_string(),
        )
        .await;

        assert!(cached.is_none());

        reset_env(env).await;
    }

    // Verifies retrieval of all subscribed indicators
    async fn scenario_get_active_subscribed_indicators_cache(env: Environments) {
        reset_env(env).await;

        let map = mock_map();

        SubscribedIndicators::<Cache>::set_active_subscribed_indicators_cache(&env, map.clone())
            .await;

        let cache = SubscribedIndicators::<Cache>::get_active_subscribed_indicators_cache(&env)
            .await
            .unwrap();

        assert_eq!(cache.len(), map.len());

        reset_env(env).await;
    }

    // Verifies initialized status behavior
    async fn scenario_get_active_subscribed_indicators_status_cache(env: Environments) {
        reset_env(env).await;

        let initial_status =
            SubscribedIndicators::<Cache>::get_active_subscribed_indicators_status_cache(&env)
                .await;
        assert!(!initial_status);

        let map = mock_map();
        SubscribedIndicators::<Cache>::set_active_subscribed_indicators_cache(&env, map).await;

        let status =
            SubscribedIndicators::<Cache>::get_active_subscribed_indicators_status_cache(&env)
                .await;
        assert!(status);

        reset_env(env).await;
    }

    #[tokio::test]
    async fn cache_subscribed_indicators_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_active_subscribed_indicators_cache(env).await;
        scenario_set_active_subscribed_indicator_insert(env).await;
        scenario_multiple_indicators_same_symbol(env).await;
        scenario_set_active_subscribed_indicator_remove(env).await;
        scenario_get_active_subscribed_indicators_cache(env).await;
        scenario_get_active_subscribed_indicators_status_cache(env).await;

        // Final safety cleanup
        reset_env(env).await;
    }
}
