use std::{collections::HashMap, sync::Arc};

use models::{entities::actions::Model, structs::Environments};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Actions, utils::Cache};

#[derive(Default)]
struct CacheEnvironments {
    pub environments: HashMap<Environments, CacheActions>,
}

#[derive(Default)]
struct CacheActions {
    pub is_initialized: bool,
    pub models: HashMap<i32, Model>,
}

static ACTIVE_ACTIONS: Lazy<Arc<RwLock<CacheEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheEnvironments::default())));

impl Actions<Cache> {
    pub async fn set_active_actions_cache(
        environment: &Environments,
        actions: Vec<Model>,
    ) -> Vec<Model> {
        let mut cache_actions = ACTIVE_ACTIONS.write().await;

        let env_map = cache_actions
            .environments
            .entry(*environment)
            .or_insert_with(CacheActions::default);

        for action in actions.iter() {
            env_map.models.insert(action.strategy_id, action.clone());
        }

        env_map.is_initialized = true;

        actions
    }

    pub async fn set_active_action_cache(
        environment: &Environments,
        action: Model,
        is_remove: bool,
    ) -> Model {
        let mut cache_actions = ACTIVE_ACTIONS.write().await;

        let Some(env_map) = cache_actions.environments.get_mut(environment) else {
            return action;
        };

        if is_remove {
            env_map
                .models
                .remove(&action.id)
                .map(|val| val)
                .unwrap_or(action)
        } else {
            env_map.models.insert(action.id, action.clone());

            action
        }
    }

    pub async fn get_active_actions_cache(
        environment: &Environments,
    ) -> Option<HashMap<i32, Model>> {
        let cache_actions = ACTIVE_ACTIONS.read().await;

        let env_map = cache_actions.environments.get(&environment)?;

        Some(env_map.models.clone())
    }

    pub async fn get_active_action_cache(environment: &Environments, key: &i32) -> Option<Model> {
        let cache_actions = ACTIVE_ACTIONS.read().await;

        let env_map = cache_actions.environments.get(&environment)?;

        let cache_action = env_map.models.get(key)?;

        Some(cache_action.clone())
    }

    pub async fn get_active_actions_status_cache(environment: &Environments) -> bool {
        let cache_actions = ACTIVE_ACTIONS.read().await;

        cache_actions
            .environments
            .get(&environment)
            .map(|val| val.is_initialized)
            .unwrap_or(false)
    }

    pub async fn stop_active_actions_cache(environment: &Environments) {
        let mut cache_actions = ACTIVE_ACTIONS.write().await;

        let Some(env_map) = cache_actions.environments.get_mut(environment) else {
            return;
        };

        env_map.models = HashMap::new();

        env_map.is_initialized = false;
    }
}
