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
