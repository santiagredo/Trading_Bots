use models::enums::{transition_with_timestamp, FiniteStateMachine};
use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use models::{
    entities::strategies::Model,
    enums::LifecycleState,
    structs::{CacheStrategies, CacheStrategiesEnvironments, CacheStrategy, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Strategies, utils::Cache};

static ACTIVE_STRATEGIES: Lazy<Arc<RwLock<CacheStrategiesEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheStrategiesEnvironments::new())));

impl Strategies<Cache> {
    /* ===========================
     * Lifecycle
     * ===========================
     */

    pub async fn set_status_cache(
        environment: Environments,
        status: LifecycleState,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_or_create(environment);

        transition_with_timestamp(env_cache, status)?;
        Ok(())
    }

    pub async fn reset_strategies_cache(environment: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&environment) {
            *env_cache = CacheStrategies::new();
        }

        Ok(())
    }

    /* ===========================
     * Mutations
     * ===========================
     */

    pub async fn set_strategies_cache(
        environment: Environments,
        strategies: Vec<Model>,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models = strategies
            .into_iter()
            .map(|s| {
                (
                    s.id,
                    CacheStrategy {
                        is_posting: false,
                        model: s,
                        last_error_date: None,
                        last_error_message: None,
                    },
                )
            })
            .collect();

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn upsert_strategy_cache(
        environment: Environments,
        strategy: Model,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache
            .models
            .entry(strategy.id)
            .and_modify(|c| c.model = strategy.clone())
            .or_insert(CacheStrategy {
                is_posting: false,
                model: strategy,
                last_error_date: None,
                last_error_message: None,
            });

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn remove_strategy_cache(
        environment: Environments,
        strategy_id: i32,
    ) -> Result<Option<CacheStrategy>, String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
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

    pub async fn remove_strategies_cache(
        environment: Environments,
    ) -> Result<HashMap<i32, CacheStrategy>, String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;

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

    /* ===========================
     * Helpers (extra properties)
     * ===========================
     */

    pub async fn set_strategy_posting_cache(
        environment: Environments,
        strategy_id: i32,
        is_posting: bool,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        let strategy = env_cache
            .models
            .get_mut(&strategy_id)
            .ok_or("Strategy not found")?;

        if strategy.is_posting && is_posting {
            return Err("Strategy already posting".into());
        }

        strategy.is_posting = is_posting;
        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn set_strategy_error_cache(
        environment: Environments,
        strategy_id: i32,
        error: Option<String>,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_STRATEGIES.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        let strategy = env_cache
            .models
            .get_mut(&strategy_id)
            .ok_or("Strategy not found")?;

        strategy.last_error_date = error.as_ref().map(|_| Local::now().naive_local());
        strategy.last_error_message = error;

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    /* ===========================
     * Queries
     * ===========================
     */

    pub async fn get_strategies_cache(environment: Environments) -> Option<CacheStrategies> {
        let cache = ACTIVE_STRATEGIES.read().await;
        cache.get(&environment).cloned()
    }

    pub async fn get_strategy_cache(
        environment: Environments,
        strategy_id: i32,
    ) -> Option<CacheStrategy> {
        let cache = ACTIVE_STRATEGIES.read().await;
        cache.get(&environment)?.models.get(&strategy_id).cloned()
    }

    pub async fn get_strategies_state_cache(environment: Environments) -> LifecycleState {
        let cache = ACTIVE_STRATEGIES.read().await;
        cache
            .get(&environment)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }
}

#[cfg(test)]
mod tests {
    use models::{entities::strategies::Model, enums::LifecycleState, structs::Environments};

    use crate::{handler::Strategies, utils::Cache};

    /* ===========================
     * Helpers
     * ===========================
     */

    fn mock_strategy(id: i32) -> Model {
        Model {
            id,
            is_active: true,
            ..Default::default()
        }
    }

    async fn reset_env(env: Environments) {
        let _ = Strategies::<Cache>::reset_strategies_cache(env).await;
    }

    /* ===========================
     * Scenarios
     * ===========================
     */

    async fn scenario_initial_state(env: Environments) {
        reset_env(env).await;

        let state = Strategies::<Cache>::get_strategies_state_cache(env).await;
        assert_eq!(state, LifecycleState::Off);

        let cache = Strategies::<Cache>::get_strategies_cache(env).await;
        assert!(cache.is_some());
        assert!(cache.unwrap().models.is_empty());
    }

    async fn scenario_lifecycle_transitions(env: Environments) {
        reset_env(env).await;

        assert!(
            Strategies::<Cache>::set_status_cache(env, LifecycleState::Starting)
                .await
                .is_ok()
        );
        assert!(
            Strategies::<Cache>::set_status_cache(env, LifecycleState::Running)
                .await
                .is_ok()
        );

        let state = Strategies::<Cache>::get_strategies_state_cache(env).await;
        assert_eq!(state, LifecycleState::Running);

        assert!(
            Strategies::<Cache>::set_status_cache(env, LifecycleState::Stopping)
                .await
                .is_ok()
        );
        assert!(
            Strategies::<Cache>::set_status_cache(env, LifecycleState::Off)
                .await
                .is_ok()
        );
    }

    async fn scenario_set_strategies_cache(env: Environments) {
        reset_env(env).await;

        Strategies::<Cache>::set_status_cache(env, LifecycleState::Starting)
            .await
            .unwrap();
        Strategies::<Cache>::set_status_cache(env, LifecycleState::Running)
            .await
            .unwrap();

        let strategies = vec![mock_strategy(1), mock_strategy(2)];

        let result = Strategies::<Cache>::set_strategies_cache(env, strategies);
        assert!(result.await.is_ok());

        let cache = Strategies::<Cache>::get_strategies_cache(env)
            .await
            .unwrap();
        assert_eq!(cache.models.len(), 2);
        assert!(cache.models.contains_key(&1));
        assert!(cache.models.contains_key(&2));
    }

    async fn scenario_upsert_strategy(env: Environments) {
        reset_env(env).await;

        Strategies::<Cache>::set_status_cache(env, LifecycleState::Starting)
            .await
            .unwrap();
        Strategies::<Cache>::set_status_cache(env, LifecycleState::Running)
            .await
            .unwrap();

        let strategy = mock_strategy(10);

        Strategies::<Cache>::upsert_strategy_cache(env, strategy.clone())
            .await
            .unwrap();

        let cached = Strategies::<Cache>::get_strategy_cache(env, 10)
            .await
            .unwrap();

        assert_eq!(cached.model, strategy);
        assert!(!cached.is_posting);
        assert!(cached.last_error_date.is_none());
    }

    async fn scenario_remove_single_strategy(env: Environments) {
        reset_env(env).await;

        Strategies::<Cache>::set_status_cache(env, LifecycleState::Starting)
            .await
            .unwrap();
        Strategies::<Cache>::set_status_cache(env, LifecycleState::Running)
            .await
            .unwrap();

        let strategy = mock_strategy(20);
        Strategies::<Cache>::upsert_strategy_cache(env, strategy)
            .await
            .unwrap();

        let removed = Strategies::<Cache>::remove_strategy_cache(env, 20)
            .await
            .unwrap();

        assert!(removed.is_some());

        let cache = Strategies::<Cache>::get_strategies_cache(env)
            .await
            .unwrap();
        assert!(cache.models.is_empty());
    }

    async fn scenario_remove_all_strategies(env: Environments) {
        reset_env(env).await;

        Strategies::<Cache>::set_status_cache(env, LifecycleState::Starting)
            .await
            .unwrap();
        Strategies::<Cache>::set_status_cache(env, LifecycleState::Running)
            .await
            .unwrap();

        let strategies = vec![mock_strategy(1), mock_strategy(2)];
        Strategies::<Cache>::set_strategies_cache(env, strategies)
            .await
            .unwrap();

        Strategies::<Cache>::set_status_cache(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = Strategies::<Cache>::remove_strategies_cache(env)
            .await
            .unwrap();

        assert_eq!(removed.len(), 2);

        let cache = Strategies::<Cache>::get_strategies_cache(env)
            .await
            .unwrap();
        assert!(cache.models.is_empty());
    }

    async fn scenario_posting_flag(env: Environments) {
        reset_env(env).await;

        Strategies::<Cache>::set_status_cache(env, LifecycleState::Starting)
            .await
            .unwrap();
        Strategies::<Cache>::set_status_cache(env, LifecycleState::Running)
            .await
            .unwrap();

        let strategy = mock_strategy(30);
        Strategies::<Cache>::upsert_strategy_cache(env, strategy)
            .await
            .unwrap();

        // set posting true
        assert!(
            Strategies::<Cache>::set_strategy_posting_cache(env, 30, true)
                .await
                .is_ok()
        );

        // set posting true again (should fail)
        assert!(
            Strategies::<Cache>::set_strategy_posting_cache(env, 30, true)
                .await
                .is_err()
        );

        // set posting false
        assert!(
            Strategies::<Cache>::set_strategy_posting_cache(env, 30, false)
                .await
                .is_ok()
        );
    }

    async fn scenario_strategy_error(env: Environments) {
        reset_env(env).await;

        Strategies::<Cache>::set_status_cache(env, LifecycleState::Starting)
            .await
            .unwrap();
        Strategies::<Cache>::set_status_cache(env, LifecycleState::Running)
            .await
            .unwrap();

        let strategy = mock_strategy(40);
        Strategies::<Cache>::upsert_strategy_cache(env, strategy)
            .await
            .unwrap();

        Strategies::<Cache>::set_strategy_error_cache(env, 40, Some("boom".to_string()))
            .await
            .unwrap();

        let cached = Strategies::<Cache>::get_strategy_cache(env, 40)
            .await
            .unwrap();

        assert!(cached.last_error_date.is_some());
        assert_eq!(cached.last_error_message, Some("boom".to_string()));
    }

    /* ===========================
     * Entry test
     * ===========================
     */

    #[tokio::test]
    async fn strategies_cache_full_flow_should_work_correctly() {
        let env = Environments::DEV;

        scenario_initial_state(env).await;
        scenario_lifecycle_transitions(env).await;
        scenario_set_strategies_cache(env).await;
        scenario_upsert_strategy(env).await;
        scenario_remove_single_strategy(env).await;
        scenario_remove_all_strategies(env).await;
        scenario_posting_flag(env).await;
        scenario_strategy_error(env).await;

        reset_env(env).await;
    }
}
