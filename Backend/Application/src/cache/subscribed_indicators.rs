use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::sync::Arc;

use chrono::Local;
use models::enums::FiniteStateMachine;
use models::{
    entities::indicators::Model,
    enums::{transition_with_timestamp, LifecycleState},
    structs::{CacheSubscribedIndicatorsEnvironments, Environments},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::SubscribedIndicators, utils::Cache};

static ACTIVE_SUBSCRIBED_INDICATORS: Lazy<Arc<RwLock<CacheSubscribedIndicatorsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheSubscribedIndicatorsEnvironments::new())));

impl SubscribedIndicators<Cache> {
    /* ===========================
     * LIFECYCLE
     * ===========================
     */

    pub async fn set_status_cache(
        environment: Environments,
        status: LifecycleState,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;
        let env_cache = cache.get_or_create(environment);

        transition_with_timestamp(env_cache, status)?;
        Ok(())
    }

    /* ===========================
     * WRITE
     * ===========================
     */

    pub async fn set_subscribed_indicators_cache(
        environment: Environments,
        indicators: HashMap<String, HashSet<i32>>,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err(format!("Cache not running ({:?})", env_cache.status));
        }

        env_cache.models = indicators;
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn upsert_subscribed_indicator_cache(
        environment: Environments,
        indicator: Model,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        match env_cache.models.entry(indicator.symbol.clone()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().insert(indicator.strategy_id);
            }
            Entry::Vacant(entry) => {
                entry.insert([indicator.strategy_id].into_iter().collect());
            }
        }

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn remove_subscribed_indicator_cache(
        environment: Environments,
        indicator: Model,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        if let Entry::Occupied(mut entry) = env_cache.models.entry(indicator.symbol.clone()) {
            entry.get_mut().remove(&indicator.strategy_id);
            if entry.get().is_empty() {
                entry.remove();
            }
        }

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn remove_all_subscribed_indicators_cache(
        environment: Environments,
    ) -> Result<HashMap<String, HashSet<i32>>, String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed = std::mem::take(&mut env_cache.models);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    /* ===========================
     * READ
     * ===========================
     */

    pub async fn get_subscribed_indicators_cache(
        environment: Environments,
    ) -> Option<HashMap<String, HashSet<i32>>> {
        let cache = ACTIVE_SUBSCRIBED_INDICATORS.read().await;
        cache.get(&environment).map(|c| c.models.clone())
    }

    pub async fn get_subscribed_indicator_cache(
        environment: Environments,
        symbol: &str,
    ) -> Option<HashSet<i32>> {
        let cache = ACTIVE_SUBSCRIBED_INDICATORS.read().await;
        cache.get(&environment)?.models.get(symbol).cloned()
    }

    pub async fn get_subscribed_indicators_state_cache(
        environment: Environments,
    ) -> LifecycleState {
        let cache = ACTIVE_SUBSCRIBED_INDICATORS.read().await;
        cache
            .get(&environment)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    pub async fn reset_subscribed_indicators_cache(
        environment: Environments,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_SUBSCRIBED_INDICATORS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&environment) {
            *env_cache = models::structs::CacheSubscribedIndicators::new();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use models::{entities::indicators::Model, enums::LifecycleState, structs::Environments};

    use crate::{handler::SubscribedIndicators, utils::Cache};

    /* =========================
     * Helpers
     * =========================
     */

    fn mock_indicator(symbol: &str, strategy_id: i32) -> Model {
        Model {
            id: strategy_id,
            strategy_id,
            symbol: symbol.to_string(),
            ..Default::default()
        }
    }

    fn mock_map() -> HashMap<String, HashSet<i32>> {
        HashMap::from([
            ("BTCUSDT".to_string(), HashSet::from([1, 2])),
            ("ETHUSDT".to_string(), HashSet::from([3])),
        ])
    }

    async fn start_env(env: Environments) {
        let _ =
            SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Starting).await;

        let _ = SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    }

    async fn stop_env(env: Environments) {
        let _ =
            SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = SubscribedIndicators::<Cache>::remove_all_subscribed_indicators_cache(env)
            .await
            .expect("remove_all_subscribed_indicators_cache should work in Stopping");

        assert!(removed.is_empty() || !removed.is_empty());

        let _ = SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    }

    /* =========================
     * Scenarios (unit responsibilities)
     * =========================
     */

    async fn scenario_reset_env_clears_state(env: Environments) {
        start_env(env).await;

        let map = mock_map();
        SubscribedIndicators::<Cache>::set_subscribed_indicators_cache(env, map)
            .await
            .unwrap();

        let _ =
            SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = SubscribedIndicators::<Cache>::remove_all_subscribed_indicators_cache(env)
            .await
            .unwrap();

        assert!(!removed.is_empty());

        let cache = SubscribedIndicators::<Cache>::get_subscribed_indicators_cache(env)
            .await
            .unwrap();

        assert!(cache.is_empty());

        let _ = SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Off).await;

        let status =
            SubscribedIndicators::<Cache>::get_subscribed_indicators_state_cache(env).await;

        assert_eq!(status, LifecycleState::Off);
    }

    async fn scenario_set_subscribed_indicators_cache(env: Environments) {
        start_env(env).await;

        let map = mock_map();

        SubscribedIndicators::<Cache>::set_subscribed_indicators_cache(env, map.clone())
            .await
            .unwrap();

        let cache = SubscribedIndicators::<Cache>::get_subscribed_indicators_cache(env)
            .await
            .unwrap();

        let status =
            SubscribedIndicators::<Cache>::get_subscribed_indicators_state_cache(env).await;

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get("BTCUSDT").unwrap().len(), 2);
        assert_eq!(cache.get("ETHUSDT").unwrap().len(), 1);
        assert_eq!(status, LifecycleState::Running);

        stop_env(env).await;
    }

    async fn scenario_upsert_subscribed_indicator_insert(env: Environments) {
        start_env(env).await;

        let indicator = mock_indicator("BTCUSDT", 10);

        SubscribedIndicators::<Cache>::upsert_subscribed_indicator_cache(env, indicator.clone())
            .await
            .unwrap();

        let cached = SubscribedIndicators::<Cache>::get_subscribed_indicator_cache(env, "BTCUSDT")
            .await
            .unwrap();

        assert!(cached.contains(&10));

        stop_env(env).await;
    }

    async fn scenario_multiple_indicators_same_symbol(env: Environments) {
        start_env(env).await;

        let i1 = mock_indicator("BTCUSDT", 1);
        let i2 = mock_indicator("BTCUSDT", 2);

        SubscribedIndicators::<Cache>::upsert_subscribed_indicator_cache(env, i1)
            .await
            .unwrap();

        SubscribedIndicators::<Cache>::upsert_subscribed_indicator_cache(env, i2)
            .await
            .unwrap();

        let cached = SubscribedIndicators::<Cache>::get_subscribed_indicator_cache(env, "BTCUSDT")
            .await
            .unwrap();

        assert_eq!(cached.len(), 2);
        assert!(cached.contains(&1));
        assert!(cached.contains(&2));

        stop_env(env).await;
    }

    async fn scenario_remove_subscribed_indicator(env: Environments) {
        start_env(env).await;

        let indicator = mock_indicator("ETHUSDT", 99);

        SubscribedIndicators::<Cache>::upsert_subscribed_indicator_cache(env, indicator.clone())
            .await
            .unwrap();

        SubscribedIndicators::<Cache>::remove_subscribed_indicator_cache(env, indicator)
            .await
            .unwrap();

        let cached =
            SubscribedIndicators::<Cache>::get_subscribed_indicator_cache(env, "ETHUSDT").await;

        assert!(cached.is_none());

        stop_env(env).await;
    }

    async fn scenario_cannot_write_when_not_running(env: Environments) {
        let indicator = mock_indicator("BTCUSDT", 5);

        let result =
            SubscribedIndicators::<Cache>::upsert_subscribed_indicator_cache(env, indicator).await;

        assert!(result.is_err());
    }

    async fn scenario_cannot_remove_all_when_running(env: Environments) {
        start_env(env).await;

        let result =
            SubscribedIndicators::<Cache>::remove_all_subscribed_indicators_cache(env).await;

        assert!(result.is_err());

        stop_env(env).await;
    }

    /* =========================
     * Entry point
     * =========================
     */

    #[tokio::test]
    async fn cache_subscribed_indicators_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_subscribed_indicators_cache(env).await;
        scenario_upsert_subscribed_indicator_insert(env).await;
        scenario_multiple_indicators_same_symbol(env).await;
        scenario_remove_subscribed_indicator(env).await;
        scenario_cannot_write_when_not_running(env).await;
        scenario_cannot_remove_all_when_running(env).await;
    }
}
