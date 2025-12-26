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

#[cfg(test)]
mod tests {
    use models::{entities::status::Model, enums::Status, structs::Environments};

    use crate::{handler::OrderStatus, utils::Cache};

    // Helpers
    fn mock_model(id: i32, name: &str) -> Model {
        Model {
            id,
            name: name.to_string(),
        }
    }

    async fn reset_env(env: Environments) {
        OrderStatus::<Cache>::stop_active_status_cache(&env).await;
    }

    // Scenarios (unit responsibilities)

    // Verifies that reset clears cache state
    async fn scenario_reset_env_clears_state(env: Environments) {
        let models = vec![mock_model(1, "OPEN"), mock_model(2, "COMPLETED")];

        OrderStatus::<Cache>::set_active_status_cache(&env, models).await;
        reset_env(env).await;

        let cache = OrderStatus::<Cache>::get_active_status_cache(&env)
            .await
            .expect("environment entry should exist");

        let status = OrderStatus::<Cache>::get_active_status_status_cache(&env).await;

        assert!(cache.is_empty());
        assert!(!status);
    }

    // Verifies bulk insertion and initialization flag
    async fn scenario_set_active_status_cache(env: Environments) {
        reset_env(env).await;

        let models = vec![
            mock_model(1, "OPEN"),
            mock_model(2, "COMPLETED"),
            mock_model(3, "ABORTED"),
        ];

        OrderStatus::<Cache>::set_active_status_cache(&env, models).await;

        let cache = OrderStatus::<Cache>::get_active_status_cache(&env)
            .await
            .expect("cache should exist");

        let status_flag = OrderStatus::<Cache>::get_active_status_status_cache(&env).await;

        assert_eq!(cache.len(), 3);
        assert!(cache.contains_key(&Status::Open));
        assert!(cache.contains_key(&Status::Completed));
        assert!(cache.contains_key(&Status::Aborted));
        assert!(status_flag);

        reset_env(env).await;
    }

    // Verifies single status retrieval
    async fn scenario_get_active_status(env: Environments) {
        reset_env(env).await;

        let model = mock_model(10, "COMPLETED");
        OrderStatus::<Cache>::set_active_status_cache(&env, vec![model]).await;

        let cached = OrderStatus::<Cache>::get_active_status(&env, &Status::Completed).await;

        assert_eq!(cached, Some(10));

        reset_env(env).await;
    }

    // Verifies overwrite behavior (models are cleared before insert)
    async fn scenario_overwrite_active_status_cache(env: Environments) {
        reset_env(env).await;

        let first = vec![mock_model(1, "OPEN"), mock_model(2, "COMPLETED")];

        OrderStatus::<Cache>::set_active_status_cache(&env, first).await;

        let second = vec![mock_model(3, "ABORTED")];

        OrderStatus::<Cache>::set_active_status_cache(&env, second).await;

        let cache = OrderStatus::<Cache>::get_active_status_cache(&env)
            .await
            .unwrap();

        assert_eq!(cache.len(), 1);
        assert!(cache.contains_key(&Status::Aborted));
        assert!(!cache.contains_key(&Status::Open));
        assert!(!cache.contains_key(&Status::Completed));

        reset_env(env).await;
    }

    // Verifies initialized flag behavior
    async fn scenario_get_active_status_status_cache(env: Environments) {
        reset_env(env).await;

        let initial = OrderStatus::<Cache>::get_active_status_status_cache(&env).await;
        assert!(!initial);

        let models = vec![mock_model(1, "OPEN")];
        OrderStatus::<Cache>::set_active_status_cache(&env, models).await;

        let status = OrderStatus::<Cache>::get_active_status_status_cache(&env).await;
        assert!(status);

        reset_env(env).await;
    }

    #[tokio::test]
    async fn order_status_cache_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_active_status_cache(env).await;
        scenario_get_active_status(env).await;
        scenario_overwrite_active_status_cache(env).await;
        scenario_get_active_status_status_cache(env).await;

        reset_env(env).await;
    }
}
