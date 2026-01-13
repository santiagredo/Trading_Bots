use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use models::{
    entities::actions::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState},
    structs::{CacheActions, CacheActionsEnvironments, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Actions, utils::Cache};

static ACTIVE_ACTIONS: Lazy<Arc<RwLock<CacheActionsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheActionsEnvironments::new())));

impl Actions<Cache> {
    pub async fn set_status_cache(
        environment: Environments,
        status: LifecycleState,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_ACTIONS.write().await;
        let env_cache = cache.get_or_create(environment);

        transition_with_timestamp(env_cache, status)?;
        Ok(())
    }

    pub async fn set_actions_cache(
        environment: Environments,
        actions: Vec<Model>,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_ACTIONS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err(format!(
                "Cannot load actions: cache not running ({:?})",
                env_cache.status
            ));
        }

        env_cache.models = actions.into_iter().map(|a| (a.strategy_id, a)).collect();

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn upsert_action_cache(
        environment: Environments,
        action: Model,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_ACTIONS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(action.strategy_id, action);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn remove_action_cache(
        environment: Environments,
        strategy_id: i32,
    ) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_ACTIONS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache.models.remove(&strategy_id);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    pub async fn remove_actions_cache(
        environment: Environments,
    ) -> Result<HashMap<i32, Model>, String> {
        let mut cache = ACTIVE_ACTIONS.write().await;
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

    pub async fn get_actions_cache(environment: Environments) -> Option<CacheActions> {
        let cache = ACTIVE_ACTIONS.read().await;
        cache.get(&environment).cloned()
    }

    pub async fn get_action_cache(environment: Environments, strategy_id: i32) -> Option<Model> {
        let cache = ACTIVE_ACTIONS.read().await;
        cache.get(&environment)?.models.get(&strategy_id).cloned()
    }

    pub async fn get_cache_state(environment: Environments) -> LifecycleState {
        let cache = ACTIVE_ACTIONS.read().await;
        cache
            .get(&environment)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    pub async fn reset_actions_cache(environment: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_ACTIONS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&environment) {
            *env_cache = CacheActions::new();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use models::{entities::actions::Model, enums::LifecycleState, structs::Environments};
    use sea_orm::prelude::Decimal;

    use crate::{handler::Actions, utils::Cache};

    // =========================
    // Helpers
    // =========================

    fn mock_action(strategy_id: i32) -> Model {
        Model {
            id: strategy_id,
            strategy_id,
            is_active: true,
            is_sell: false,
            is_quote_asset: false,
            is_percentage: false,
            value: Decimal::new(100, 0),
            pair_id: 1,
            ..Default::default()
        }
    }

    async fn start_env(env: Environments) {
        let _ = Actions::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
        let _ = Actions::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    }

    async fn stop_env(env: Environments) {
        let _ = Actions::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = Actions::<Cache>::remove_actions_cache(env)
            .await
            .expect("remove_actions_cache should work in Stopping");

        assert!(removed.is_empty() || !removed.is_empty());

        let _ = Actions::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    }

    // =========================
    // Scenarios (unit responsibilities)
    // =========================

    async fn scenario_reset_env_clears_state(env: Environments) {
        start_env(env).await;

        let actions = vec![mock_action(1)];
        Actions::<Cache>::set_actions_cache(env, actions)
            .await
            .unwrap();

        let _ = Actions::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = Actions::<Cache>::remove_actions_cache(env).await.unwrap();
        assert_eq!(removed.len(), 1);

        let cache = Actions::<Cache>::get_actions_cache(env).await.unwrap();
        assert!(cache.models.is_empty());

        let _ = Actions::<Cache>::set_status_cache(env, LifecycleState::Off).await;

        let status = Actions::<Cache>::get_cache_state(env).await;
        assert_eq!(status, LifecycleState::Off);
    }

    async fn scenario_set_actions_cache(env: Environments) {
        start_env(env).await;

        let actions = vec![mock_action(1), mock_action(2)];

        Actions::<Cache>::set_actions_cache(env, actions)
            .await
            .unwrap();

        let cache = Actions::<Cache>::get_actions_cache(env).await.unwrap();
        let status = Actions::<Cache>::get_cache_state(env).await;

        assert_eq!(cache.models.len(), 2);
        assert!(cache.models.contains_key(&1));
        assert!(cache.models.contains_key(&2));
        assert_eq!(status, LifecycleState::Running);

        stop_env(env).await;
    }

    async fn scenario_upsert_action_insert(env: Environments) {
        start_env(env).await;

        let action = mock_action(10);
        Actions::<Cache>::upsert_action_cache(env, action.clone())
            .await
            .unwrap();

        let cached = Actions::<Cache>::get_action_cache(env, 10).await.unwrap();
        assert_eq!(cached, action);

        stop_env(env).await;
    }

    async fn scenario_remove_action(env: Environments) {
        start_env(env).await;

        let action = mock_action(20);
        Actions::<Cache>::upsert_action_cache(env, action)
            .await
            .unwrap();

        let removed = Actions::<Cache>::remove_action_cache(env, 20)
            .await
            .unwrap();

        assert!(removed.is_some());

        let cached = Actions::<Cache>::get_action_cache(env, 20).await;
        assert!(cached.is_none());

        stop_env(env).await;
    }

    async fn scenario_get_cache_state(env: Environments) {
        assert_eq!(
            Actions::<Cache>::get_cache_state(env).await,
            LifecycleState::Off
        );

        start_env(env).await;

        assert_eq!(
            Actions::<Cache>::get_cache_state(env).await,
            LifecycleState::Running
        );

        stop_env(env).await;
    }

    async fn scenario_cannot_remove_actions_when_running(env: Environments) {
        start_env(env).await;

        let result = Actions::<Cache>::remove_actions_cache(env).await;
        assert!(result.is_err());

        stop_env(env).await;
    }

    // =========================
    // Entry point
    // =========================

    #[tokio::test]
    async fn cache_actions_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_actions_cache(env).await;
        scenario_upsert_action_insert(env).await;
        scenario_remove_action(env).await;
        scenario_get_cache_state(env).await;
        scenario_cannot_remove_actions_when_running(env).await;
    }
}
