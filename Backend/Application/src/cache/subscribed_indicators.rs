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
