use std::{collections::HashMap, sync::Arc};

use models::{entities::indicators::Model, structs::Environments};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Indicators, utils::Cache};

#[derive(Default)]
struct CacheEnvironments {
    pub environments: HashMap<Environments, CacheIndicators>,
}

#[derive(Default)]
struct CacheIndicators {
    pub is_initialized: bool,
    pub models: HashMap<i32, Model>,
}

static ACTIVE_INDICATORS: Lazy<Arc<RwLock<CacheEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheEnvironments::default())));

impl Indicators<Cache> {
    pub async fn set_active_indicators_cache(
        environment: &Environments,
        indicators: Vec<Model>,
    ) -> Vec<Model> {
        let mut cache_indicators = ACTIVE_INDICATORS.write().await;

        let env_map = cache_indicators
            .environments
            .entry(*environment)
            .or_insert_with(CacheIndicators::default);

        for indicator in indicators.iter() {
            env_map
                .models
                .insert(indicator.strategy_id, indicator.clone());
        }

        // dbg!(&active_indicators_map);
        env_map.is_initialized = true;

        indicators
    }

    pub async fn set_active_indicator_cache(
        environment: &Environments,
        indicator: Model,
        is_remove: bool,
    ) -> Model {
        let mut active_indicators = ACTIVE_INDICATORS.write().await;

        let env_map = active_indicators
            .environments
            .entry(*environment)
            .or_insert_with(CacheIndicators::default);

        if is_remove {
            env_map
                .models
                .remove(&indicator.strategy_id)
                .map(|val| val)
                .unwrap_or(indicator)
        } else {
            env_map
                .models
                .insert(indicator.strategy_id, indicator.clone());

            indicator
        }
    }

    pub async fn get_active_indicators_cache(
        environment: &Environments,
    ) -> Option<HashMap<i32, Model>> {
        let active_indicators = ACTIVE_INDICATORS.read().await;

        let env_map = active_indicators.environments.get(&environment)?;

        Some(env_map.models.clone())
    }

    pub async fn get_active_indicator_cache(
        environment: &Environments,
        key: &i32,
    ) -> Option<Model> {
        let active_indicators = ACTIVE_INDICATORS.read().await;

        let env_map = active_indicators.environments.get(&environment)?;

        let cache_indicator = env_map.models.get(key)?;

        Some(cache_indicator.clone())
    }

    pub async fn get_active_indicators_status_cache(environment: &Environments) -> bool {
        let cache_indicators = ACTIVE_INDICATORS.read().await;

        cache_indicators
            .environments
            .get(&environment)
            .map(|val| val.is_initialized)
            .unwrap_or(false)
    }

    pub async fn stop_active_indicators_cache(environment: &Environments) {
        let mut cache_indicators = ACTIVE_INDICATORS.write().await;

        let Some(env_map) = cache_indicators.environments.get_mut(environment) else {
            return;
        };

        env_map.models = HashMap::new();

        env_map.is_initialized = false;
    }
}

#[cfg(test)]
mod tests {
    use crate::{handler::Indicators, utils::Cache};
    use models::{entities::indicators::Model, structs::Environments};

    // Helpers
    fn mock_indicator(strategy_id: i32) -> Model {
        Model {
            id: strategy_id,
            strategy_id,
            is_active: true,
            ..Default::default()
        }
    }

    async fn reset_env(env: Environments) {
        Indicators::<Cache>::stop_active_indicators_cache(&env).await;
    }

    // Scenarios (unit responsibilities)
    async fn scenario_reset_env_clears_state(env: Environments) {
        let indicators = vec![mock_indicator(1), mock_indicator(2)];

        Indicators::<Cache>::set_active_indicators_cache(&env, indicators).await;
        reset_env(env).await;

        let cache = Indicators::<Cache>::get_active_indicators_cache(&env)
            .await
            .expect("environment entry should exist");

        let status = Indicators::<Cache>::get_active_indicators_status_cache(&env).await;

        assert!(cache.is_empty());
        assert!(!status);
    }

    async fn scenario_set_active_indicators_cache(env: Environments) {
        reset_env(env).await;

        let indicators = vec![mock_indicator(1), mock_indicator(2)];

        Indicators::<Cache>::set_active_indicators_cache(&env, indicators).await;

        let cache = Indicators::<Cache>::get_active_indicators_cache(&env)
            .await
            .expect("cache should exist");

        let status = Indicators::<Cache>::get_active_indicators_status_cache(&env).await;

        assert_eq!(cache.len(), 2);
        assert!(cache.contains_key(&1));
        assert!(cache.contains_key(&2));
        assert!(status);

        reset_env(env).await;
    }

    async fn scenario_set_active_indicator_cache_insert(env: Environments) {
        reset_env(env).await;

        let indicator = mock_indicator(10);

        Indicators::<Cache>::set_active_indicator_cache(&env, indicator.clone(), false).await;

        let cached = Indicators::<Cache>::get_active_indicator_cache(&env, &10).await;

        assert_eq!(cached, Some(indicator));

        reset_env(env).await;
    }

    async fn scenario_set_active_indicator_cache_remove(env: Environments) {
        reset_env(env).await;

        let indicator = mock_indicator(20);

        Indicators::<Cache>::set_active_indicator_cache(&env, indicator.clone(), false).await;
        Indicators::<Cache>::set_active_indicator_cache(&env, indicator, true).await;

        let cached = Indicators::<Cache>::get_active_indicator_cache(&env, &20).await;

        assert!(cached.is_none());

        reset_env(env).await;
    }

    async fn scenario_get_active_indicators_cache(env: Environments) {
        reset_env(env).await;

        let indicators = vec![mock_indicator(1), mock_indicator(2), mock_indicator(3)];

        Indicators::<Cache>::set_active_indicators_cache(&env, indicators).await;

        let cache = Indicators::<Cache>::get_active_indicators_cache(&env)
            .await
            .expect("cache should exist");

        assert_eq!(cache.len(), 3);

        reset_env(env).await;
    }

    async fn scenario_get_active_indicator_cache(env: Environments) {
        reset_env(env).await;

        let indicator = mock_indicator(42);

        Indicators::<Cache>::set_active_indicator_cache(&env, indicator.clone(), false).await;

        let cached = Indicators::<Cache>::get_active_indicator_cache(&env, &42).await;

        assert_eq!(cached, Some(indicator));

        reset_env(env).await;
    }

    async fn scenario_get_active_indicators_status_cache(env: Environments) {
        reset_env(env).await;

        let initial_status = Indicators::<Cache>::get_active_indicators_status_cache(&env).await;
        assert!(!initial_status);

        let indicators = vec![mock_indicator(1)];
        Indicators::<Cache>::set_active_indicators_cache(&env, indicators).await;

        let status = Indicators::<Cache>::get_active_indicators_status_cache(&env).await;
        assert!(status);

        reset_env(env).await;
    }

    #[tokio::test]
    async fn cache_indicators_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_active_indicators_cache(env).await;
        scenario_set_active_indicator_cache_insert(env).await;
        scenario_set_active_indicator_cache_remove(env).await;
        scenario_get_active_indicators_cache(env).await;
        scenario_get_active_indicator_cache(env).await;
        scenario_get_active_indicators_status_cache(env).await;

        // Final safety cleanup
        reset_env(env).await;
    }
}
