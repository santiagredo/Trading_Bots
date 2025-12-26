use std::{collections::HashMap, sync::Arc};

use models::{entities::status::Model, enums::Status, structs::Environments};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::OrderStatus, utils::Cache};

#[derive(Default)]
struct CacheEnvironments {
    pub environments: HashMap<Environments, CacheStatus>,
}

#[derive(Default)]
struct CacheStatus {
    pub is_initialized: bool,
    pub models: HashMap<Status, i32>,
}

static ACTIVE_STATUS: Lazy<Arc<RwLock<CacheEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheEnvironments::default())));

impl OrderStatus<Cache> {
    pub async fn set_active_status_cache(
        environment: &Environments,
        status: Vec<Model>,
    ) -> Vec<Model> {
        let mut cache_status = ACTIVE_STATUS.write().await;

        let env_map = cache_status
            .environments
            .entry(*environment)
            .or_insert_with(CacheStatus::default);

        env_map.models.clear();

        for sta in status.iter() {
            env_map.models.insert(Status::from_model(sta), sta.id);
        }

        env_map.is_initialized = true;

        status
    }

    pub async fn get_active_status_cache(
        environment: &Environments,
    ) -> Option<HashMap<Status, i32>> {
        let cache_status = ACTIVE_STATUS.read().await;

        let env_map = cache_status.environments.get(environment)?;

        Some(env_map.models.clone())
    }

    pub async fn get_active_status(environment: &Environments, status: &Status) -> Option<i32> {
        let cache_status = ACTIVE_STATUS.read().await;

        let env_map = cache_status.environments.get(environment)?;

        env_map.models.get(status).copied()
    }

    pub async fn get_active_status_status_cache(environment: &Environments) -> bool {
        let cache_status = ACTIVE_STATUS.read().await;

        cache_status
            .environments
            .get(&environment)
            .map(|val| val.is_initialized)
            .unwrap_or(false)
    }

    pub async fn stop_active_status_cache(environment: &Environments) {
        let mut cache_status = ACTIVE_STATUS.write().await;

        if let Some(env_map) = cache_status.environments.get_mut(environment) {
            env_map.models.clear();
            env_map.is_initialized = false;
        }
    }
}
