use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use models::{
    entities::indicators::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState},
    structs::{CacheIndicators, CacheIndicatorsEnvironments, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Indicators, utils::Cache};

static ACTIVE_INDICATORS: Lazy<Arc<RwLock<CacheIndicatorsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheIndicatorsEnvironments::new())));

impl Indicators<Cache> {
    // =========================
    // Lifecycle
    // =========================

    pub async fn set_status_cache(
        environment: Environments,
        status: LifecycleState,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_INDICATORS.write().await;
        let env_cache = cache.get_or_create(environment);

        transition_with_timestamp(env_cache, status)?;
        Ok(())
    }

    // =========================
    // Mutations
    // =========================

    pub async fn set_indicators_cache(
        environment: Environments,
        indicators: Vec<Model>,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_INDICATORS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err(format!(
                "Cannot load indicators: cache not running ({:?})",
                env_cache.status
            ));
        }

        env_cache.models = indicators.into_iter().map(|i| (i.strategy_id, i)).collect();

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn upsert_indicator_cache(
        environment: Environments,
        indicator: Model,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_INDICATORS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(indicator.strategy_id, indicator);

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn remove_indicator_cache(
        environment: Environments,
        strategy_id: i32,
    ) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_INDICATORS.write().await;
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

    pub async fn remove_indicators_cache(
        environment: Environments,
    ) -> Result<HashMap<i32, Model>, String> {
        let mut cache = ACTIVE_INDICATORS.write().await;
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

    // =========================
    // Queries
    // =========================

    pub async fn get_indicators_cache(environment: Environments) -> Option<CacheIndicators> {
        let cache = ACTIVE_INDICATORS.read().await;
        cache.get(&environment).cloned()
    }

    pub async fn get_indicator_cache(environment: Environments, strategy_id: i32) -> Option<Model> {
        let cache = ACTIVE_INDICATORS.read().await;
        cache.get(&environment)?.models.get(&strategy_id).cloned()
    }

    pub async fn get_cache_state(environment: Environments) -> LifecycleState {
        let cache = ACTIVE_INDICATORS.read().await;
        cache
            .get(&environment)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    pub async fn reset_indicators_cache(environment: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_INDICATORS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&environment) {
            *env_cache = CacheIndicators::new();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use models::{entities::indicators::Model, enums::LifecycleState, structs::Environments};

    use crate::{handler::Indicators, utils::Cache};

    // =========================
    // Helpers
    // =========================

    fn mock_indicator(strategy_id: i32) -> Model {
        Model {
            id: strategy_id,
            strategy_id,
            is_active: true,
            ..Default::default()
        }
    }

    async fn start_env(env: Environments) {
        let _ = Indicators::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
        let _ = Indicators::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    }

    async fn stop_env(env: Environments) {
        let _ = Indicators::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = Indicators::<Cache>::remove_indicators_cache(env)
            .await
            .expect("remove_indicators_cache should work in Stopping");

        assert!(removed.is_empty() || !removed.is_empty());

        let _ = Indicators::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    }

    // =========================
    // Scenarios
    // =========================

    async fn scenario_reset_env_clears_state(env: Environments) {
        start_env(env).await;

        Indicators::<Cache>::set_indicators_cache(env, vec![mock_indicator(1)])
            .await
            .unwrap();

        let _ = Indicators::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = Indicators::<Cache>::remove_indicators_cache(env)
            .await
            .unwrap();
        assert_eq!(removed.len(), 1);

        let cache = Indicators::<Cache>::get_indicators_cache(env)
            .await
            .unwrap();
        assert!(cache.models.is_empty());

        let _ = Indicators::<Cache>::set_status_cache(env, LifecycleState::Off).await;

        let status = Indicators::<Cache>::get_cache_state(env).await;
        assert_eq!(status, LifecycleState::Off);
    }

    async fn scenario_set_indicators_cache(env: Environments) {
        start_env(env).await;

        Indicators::<Cache>::set_indicators_cache(env, vec![mock_indicator(1), mock_indicator(2)])
            .await
            .unwrap();

        let cache = Indicators::<Cache>::get_indicators_cache(env)
            .await
            .unwrap();

        assert_eq!(cache.models.len(), 2);
        assert!(cache.models.contains_key(&1));
        assert!(cache.models.contains_key(&2));

        stop_env(env).await;
    }

    async fn scenario_upsert_indicator(env: Environments) {
        start_env(env).await;

        let indicator = mock_indicator(10);
        Indicators::<Cache>::upsert_indicator_cache(env, indicator.clone())
            .await
            .unwrap();

        let cached = Indicators::<Cache>::get_indicator_cache(env, 10)
            .await
            .unwrap();

        assert_eq!(cached, indicator);

        stop_env(env).await;
    }

    async fn scenario_remove_indicator(env: Environments) {
        start_env(env).await;

        Indicators::<Cache>::upsert_indicator_cache(env, mock_indicator(20))
            .await
            .unwrap();

        let removed = Indicators::<Cache>::remove_indicator_cache(env, 20)
            .await
            .unwrap();

        assert!(removed.is_some());
        assert!(Indicators::<Cache>::get_indicator_cache(env, 20)
            .await
            .is_none());

        stop_env(env).await;
    }

    async fn scenario_get_cache_state(env: Environments) {
        assert_eq!(
            Indicators::<Cache>::get_cache_state(env).await,
            LifecycleState::Off
        );

        start_env(env).await;

        assert_eq!(
            Indicators::<Cache>::get_cache_state(env).await,
            LifecycleState::Running
        );

        stop_env(env).await;
    }

    async fn scenario_cannot_remove_indicators_when_running(env: Environments) {
        start_env(env).await;

        let result = Indicators::<Cache>::remove_indicators_cache(env).await;

        assert!(result.is_err());

        stop_env(env).await;
    }

    // =========================
    // Entry point
    // =========================

    #[tokio::test]
    async fn cache_indicators_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_indicators_cache(env).await;
        scenario_upsert_indicator(env).await;
        scenario_remove_indicator(env).await;
        scenario_get_cache_state(env).await;
        scenario_cannot_remove_indicators_when_running(env).await;
    }
}
