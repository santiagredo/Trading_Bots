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
