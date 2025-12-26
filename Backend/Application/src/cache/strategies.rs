use std::{collections::HashMap, sync::Arc};

use chrono::Local;
use models::{
    entities::strategies::Model,
    structs::{CacheStrategy, Environments},
};
use once_cell::sync::Lazy;
use tokio::{sync::RwLock, task::JoinHandle};

use crate::{handler::Strategies, utils::Cache};

#[derive(Default)]
struct CacheEnvironments {
    pub environments: HashMap<Environments, CacheStrategies>,
}

#[derive(Default)]
struct CacheStrategies {
    pub is_initialized: bool,
    pub models: HashMap<i32, CacheStrategy>,
    pub join_handle: Option<JoinHandle<()>>,
}

static ACTIVE_STRATEGIES: Lazy<Arc<RwLock<CacheEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheEnvironments::default())));

impl Strategies<Cache> {
    pub async fn set_active_strategies_cache(
        environment: &Environments,
        strategies: Vec<Model>,
    ) -> Vec<Model> {
        let mut cache_strategies = ACTIVE_STRATEGIES.write().await;

        let env_map = cache_strategies
            .environments
            .entry(*environment)
            .or_insert_with(CacheStrategies::default);

        for strategy in strategies.iter() {
            let cache_strategy = CacheStrategy {
                is_posting: false,
                model: strategy.clone(),
                last_error_date: None,
                last_error_message: None,
            };

            env_map.models.insert(strategy.id, cache_strategy);
        }

        env_map.is_initialized = true;

        strategies
    }

    pub async fn set_active_strategy_cache(
        environment: &Environments,
        strategy: Model,
        is_remove: bool,
        error: Option<String>,
    ) -> Model {
        let mut cache_strategies = ACTIVE_STRATEGIES.write().await;

        let Some(env_map) = cache_strategies.environments.get_mut(environment) else {
            return strategy;
        };

        if is_remove {
            return env_map
                .models
                .remove(&strategy.id)
                .map(|strategy| strategy.model)
                .unwrap_or(strategy);
        }

        let now = if error.is_some() {
            Some(Local::now().naive_local())
        } else {
            None
        };

        let Some(cache_strategy) = env_map.models.get_mut(&strategy.id) else {
            let cache_strategy = CacheStrategy {
                is_posting: false,
                model: strategy.clone(),
                last_error_date: now,
                last_error_message: error,
            };

            env_map.models.insert(strategy.id, cache_strategy);

            return strategy;
        };

        cache_strategy.model = strategy.clone();

        if now.is_some() {
            cache_strategy.last_error_date = now;
        }

        if error.is_some() {
            cache_strategy.last_error_message = error;
        }

        strategy
    }

    pub async fn set_active_strategy_posting_cache(
        environment: Environments,
        strategy: i32,
        is_posting: bool,
    ) -> Result<(), String> {
        let mut cache_strategies = ACTIVE_STRATEGIES.write().await;

        let Some(env_map) = cache_strategies.environments.get_mut(&environment) else {
            return Err(format!("Environment not found in cache"));
        };

        let Some(cache_strategy) = env_map.models.get_mut(&strategy) else {
            return Err(format!("Strategy not found in cache"));
        };

        if cache_strategy.is_posting && is_posting {
            return Err(format!(
                "Strategy {strategy} is posting, received {is_posting} signal"
            ));
        }

        cache_strategy.is_posting = is_posting;

        Ok(())
    }

    pub async fn get_active_strategies_cache(
        environment: &Environments,
    ) -> Option<HashMap<i32, CacheStrategy>> {
        let cache_strategies = ACTIVE_STRATEGIES.read().await;

        let env_map = cache_strategies.environments.get(&environment)?;

        Some(env_map.models.clone())
    }

    pub async fn get_active_strategy_cache(
        environment: &Environments,
        key: &i32,
    ) -> Option<CacheStrategy> {
        let cache_strategies = ACTIVE_STRATEGIES.read().await;

        let env_map = cache_strategies.environments.get(&environment)?;

        let cache_strategy = env_map.models.get(key)?;

        Some(cache_strategy.clone())
    }

    pub async fn get_active_strategies_status_cache(environment: &Environments) -> bool {
        let cache_strategies = ACTIVE_STRATEGIES.read().await;

        cache_strategies
            .environments
            .get(&environment)
            .map(|val| val.is_initialized)
            .unwrap_or(false)
    }

    pub async fn stop_active_strategies_cache(environment: &Environments) {
        let mut cache_strategies = ACTIVE_STRATEGIES.write().await;

        let Some(env_map) = cache_strategies.environments.get_mut(environment) else {
            return;
        };

        if let Some(handle) = &env_map.join_handle {
            handle.abort();
        }

        env_map.join_handle = None;

        env_map.models = HashMap::new();

        env_map.is_initialized = false;
    }

    pub async fn set_active_strategies_join_handle_cache(
        environment: &Environments,
        join_handle: JoinHandle<()>,
    ) {
        let mut cache_strategies = ACTIVE_STRATEGIES.write().await;

        let env_map = cache_strategies
            .environments
            .entry(*environment)
            .or_insert_with(CacheStrategies::default);

        if let Some(handle) = &env_map.join_handle {
            handle.abort();
        }

        env_map.join_handle = Some(join_handle);
    }
}

#[cfg(test)]
mod tests {
    use models::{entities::strategies::Model, structs::Environments};

    use crate::{handler::Strategies, utils::Cache};

    // Helpers
    fn mock_strategy(id: i32) -> Model {
        Model {
            id,
            is_active: true,

            ..Default::default()
        }
    }

    async fn reset_env(env: Environments) {
        Strategies::<Cache>::stop_active_strategies_cache(&env).await;
    }

    // Scenarios (unit responsibilities)
    async fn scenario_reset_env_clears_state(env: Environments) {
        let strategies = vec![mock_strategy(1), mock_strategy(2)];

        Strategies::<Cache>::set_active_strategies_cache(&env, strategies).await;
        reset_env(env).await;

        let cache = Strategies::<Cache>::get_active_strategies_cache(&env)
            .await
            .expect("environment entry should exist");

        let status = Strategies::<Cache>::get_active_strategies_status_cache(&env).await;

        assert!(cache.is_empty());
        assert!(!status);
    }

    async fn scenario_set_active_strategies_cache(env: Environments) {
        reset_env(env).await;

        let strategies = vec![mock_strategy(1), mock_strategy(2)];

        Strategies::<Cache>::set_active_strategies_cache(&env, strategies).await;

        let cache = Strategies::<Cache>::get_active_strategies_cache(&env)
            .await
            .expect("cache should exist");

        let status = Strategies::<Cache>::get_active_strategies_status_cache(&env).await;

        assert_eq!(cache.len(), 2);
        assert!(cache.contains_key(&1));
        assert!(cache.contains_key(&2));
        assert!(status);

        reset_env(env).await;
    }

    async fn scenario_set_active_strategy_cache_insert(env: Environments) {
        reset_env(env).await;

        let strategy = mock_strategy(10);

        Strategies::<Cache>::set_active_strategy_cache(&env, strategy.clone(), false, None).await;

        let cached = Strategies::<Cache>::get_active_strategy_cache(&env, &10)
            .await
            .unwrap();

        assert_eq!(cached.model, strategy);
        assert!(cached.last_error_date.is_none());
        assert!(cached.last_error_message.is_none());

        reset_env(env).await;
    }

    async fn scenario_set_active_strategy_cache_remove(env: Environments) {
        reset_env(env).await;

        let strategy = mock_strategy(20);

        Strategies::<Cache>::set_active_strategy_cache(&env, strategy.clone(), false, None).await;

        Strategies::<Cache>::set_active_strategy_cache(&env, strategy, true, None).await;

        let cached = Strategies::<Cache>::get_active_strategy_cache(&env, &20).await;

        assert!(cached.is_none());

        reset_env(env).await;
    }

    async fn scenario_set_active_strategy_cache_with_error(env: Environments) {
        reset_env(env).await;

        let strategy = mock_strategy(30);

        Strategies::<Cache>::set_active_strategy_cache(
            &env,
            strategy.clone(),
            false,
            Some("error".to_string()),
        )
        .await;

        let cached = Strategies::<Cache>::get_active_strategy_cache(&env, &30)
            .await
            .unwrap();

        assert_eq!(cached.model, strategy);
        assert!(cached.last_error_date.is_some());
        assert_eq!(cached.last_error_message, Some("error".to_string()));

        reset_env(env).await;
    }

    async fn scenario_set_active_strategy_posting_cache(env: Environments) {
        reset_env(env).await;

        let strategy = mock_strategy(40);

        Strategies::<Cache>::set_active_strategies_cache(&env, vec![strategy]).await;

        // First posting should succeed
        let result = Strategies::<Cache>::set_active_strategy_posting_cache(env, 40, true).await;
        assert!(result.is_ok());

        // Second posting while already posting should fail
        let result = Strategies::<Cache>::set_active_strategy_posting_cache(env, 40, true).await;
        assert!(result.is_err());

        // Turning posting off should succeed
        let result = Strategies::<Cache>::set_active_strategy_posting_cache(env, 40, false).await;
        assert!(result.is_ok());

        reset_env(env).await;
    }

    async fn scenario_get_active_strategies_status_cache(env: Environments) {
        reset_env(env).await;

        assert!(!Strategies::<Cache>::get_active_strategies_status_cache(&env).await);

        Strategies::<Cache>::set_active_strategies_cache(&env, vec![mock_strategy(1)]).await;

        assert!(Strategies::<Cache>::get_active_strategies_status_cache(&env).await);

        reset_env(env).await;
    }

    #[tokio::test]
    async fn cache_strategies_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_active_strategies_cache(env).await;
        scenario_set_active_strategy_cache_insert(env).await;
        scenario_set_active_strategy_cache_remove(env).await;
        scenario_set_active_strategy_cache_with_error(env).await;
        scenario_set_active_strategy_posting_cache(env).await;
        scenario_get_active_strategies_status_cache(env).await;

        reset_env(env).await;
    }
}
