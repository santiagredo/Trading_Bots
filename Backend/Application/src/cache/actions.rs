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

        let env_map = cache_actions
            .environments
            .entry(*environment)
            .or_insert_with(CacheActions::default);

        if is_remove {
            env_map
                .models
                .remove(&action.strategy_id)
                .map(|val| val)
                .unwrap_or(action)
        } else {
            env_map.models.insert(action.strategy_id, action.clone());

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

#[cfg(test)]
mod tests {
    use models::{entities::actions::Model, structs::Environments};
    use sea_orm::prelude::Decimal;

    use crate::{handler::Actions, utils::Cache};

    // Helpers
    fn mock_model(strategy_id: i32) -> Model {
        Model {
            id: strategy_id,
            strategy_id,
            is_active: true,
            is_sell: false,
            is_quote_asset: false,
            is_percentage: false,
            value: Decimal::new(100, 0),
            pair_id: 1,
        }
    }

    async fn reset_env(env: Environments) {
        Actions::<Cache>::stop_active_actions_cache(&env).await;
    }

    // Scenarios (unit responsibilities)

    // Verifies that reset_env fully clears cache state
    async fn scenario_reset_env_clears_state(env: Environments) {
        let models = vec![mock_model(1), mock_model(2)];

        Actions::<Cache>::set_active_actions_cache(&env, models).await;
        reset_env(env).await;

        let cache = Actions::<Cache>::get_active_actions_cache(&env)
            .await
            .expect("environment entry should exist");

        let status = Actions::<Cache>::get_active_actions_status_cache(&env).await;

        assert!(cache.is_empty());
        assert!(!status);
    }

    // Verifies bulk insertion and initialization flag
    async fn scenario_set_active_actions_cache(env: Environments) {
        reset_env(env).await;

        let models = vec![mock_model(1), mock_model(2)];

        Actions::<Cache>::set_active_actions_cache(&env, models.clone()).await;

        let cache = Actions::<Cache>::get_active_actions_cache(&env)
            .await
            .expect("cache should exist");

        let status = Actions::<Cache>::get_active_actions_status_cache(&env).await;

        assert_eq!(cache.len(), 2);
        assert!(cache.contains_key(&1));
        assert!(cache.contains_key(&2));
        assert!(status);

        reset_env(env).await;
    }

    // Verifies single model insertion
    async fn scenario_set_active_action_cache_insert(env: Environments) {
        reset_env(env).await;

        let model = mock_model(10);

        Actions::<Cache>::set_active_action_cache(&env, model.clone(), false).await;

        let cached = Actions::<Cache>::get_active_action_cache(&env, &10).await;

        assert_eq!(cached, Some(model));

        reset_env(env).await;
    }

    // Verifies single model removal
    async fn scenario_set_active_action_cache_remove(env: Environments) {
        reset_env(env).await;

        let model = mock_model(20);

        Actions::<Cache>::set_active_action_cache(&env, model.clone(), false).await;
        Actions::<Cache>::set_active_action_cache(&env, model.clone(), true).await;

        let cached = Actions::<Cache>::get_active_action_cache(&env, &20).await;

        assert!(cached.is_none());

        reset_env(env).await;
    }

    // Verifies retrieval of all models
    async fn scenario_get_active_actions_cache(env: Environments) {
        reset_env(env).await;

        let models = vec![mock_model(1), mock_model(2), mock_model(3)];
        Actions::<Cache>::set_active_actions_cache(&env, models).await;

        let cache = Actions::<Cache>::get_active_actions_cache(&env)
            .await
            .expect("cache should exist");

        assert_eq!(cache.len(), 3);

        reset_env(env).await;
    }

    // Verifies retrieval of a single model
    async fn scenario_get_active_action_cache(env: Environments) {
        reset_env(env).await;

        let model = mock_model(42);
        Actions::<Cache>::set_active_action_cache(&env, model.clone(), false).await;

        let cached = Actions::<Cache>::get_active_action_cache(&env, &42).await;

        assert_eq!(cached, Some(model));

        reset_env(env).await;
    }

    // Verifies initialized status behavior
    async fn scenario_get_active_actions_status_cache(env: Environments) {
        reset_env(env).await;

        let initial_status = Actions::<Cache>::get_active_actions_status_cache(&env).await;
        assert!(!initial_status);

        let models = vec![mock_model(1)];
        Actions::<Cache>::set_active_actions_cache(&env, models).await;

        let status = Actions::<Cache>::get_active_actions_status_cache(&env).await;
        assert!(status);

        reset_env(env).await;
    }

    // Single test entry point
    #[tokio::test]
    async fn cache_actions_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_active_actions_cache(env).await;
        scenario_set_active_action_cache_insert(env).await;
        scenario_set_active_action_cache_remove(env).await;
        scenario_get_active_actions_cache(env).await;
        scenario_get_active_action_cache(env).await;
        scenario_get_active_actions_status_cache(env).await;

        // Final safety cleanup
        reset_env(env).await;
    }
}
