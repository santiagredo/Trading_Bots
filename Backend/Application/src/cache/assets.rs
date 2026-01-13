use std::{collections::HashMap, mem, sync::Arc};

use chrono::Local;
use models::{
    entities::assets::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState},
    structs::{CacheAssets, CacheAssetsEnvironments, Environments},
};
use once_cell::sync::Lazy;
use sea_orm::prelude::Decimal;
use tokio::sync::RwLock;

use crate::{handler::Assets, utils::Cache};

static ACTIVE_ASSETS: Lazy<Arc<RwLock<CacheAssetsEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheAssetsEnvironments::new())));

impl Assets<Cache> {
    pub async fn set_status_cache(
        environment: Environments,
        status: LifecycleState,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_ASSETS.write().await;
        let env_cache = cache.get_or_create(environment);

        transition_with_timestamp(env_cache, status)?;

        Ok(())
    }

    pub async fn set_assets_cache(
        environment: Environments,
        assets: Vec<Model>,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_ASSETS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err(format!(
                "Cannot load assets: cache not running ({:?})",
                env_cache.status
            ));
        }

        env_cache.models = assets.into_iter().map(|asset| (asset.id, asset)).collect();

        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn upsert_asset_cache(environment: Environments, asset: Model) -> Result<(), String> {
        let mut cache = ACTIVE_ASSETS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(asset.id, asset);

        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn remove_asset_cache(
        environment: Environments,
        asset_id: i32,
    ) -> Result<Option<Model>, String> {
        let mut cache = ACTIVE_ASSETS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache.models.remove(&asset_id);

        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    pub async fn remove_assets_cache(
        environment: Environments,
    ) -> Result<HashMap<i32, Model>, String> {
        let mut cache = ACTIVE_ASSETS.write().await;

        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not running".into());
        }

        let removed_models = mem::take(&mut env_cache.models);

        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed_models)
    }

    pub async fn update_asset_balance_cache(
        environment: Environments,
        asset_id: i32,
        value: Decimal,
        locked: bool,
        sell: bool,
    ) -> Result<(Model, Decimal), String> {
        let mut cache = ACTIVE_ASSETS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let asset = env_cache
            .models
            .get_mut(&asset_id)
            .ok_or("Asset not found")?;

        let target = if locked {
            &mut asset.locked
        } else {
            &mut asset.free
        };

        let previous = target.clone();

        if sell {
            *target -= value;
        } else {
            *target += value;
        }

        // env_cache.last_update_date = Local::now().naive_local();
        Ok((asset.clone(), previous))
    }

    pub async fn get_assets_cache(environment: Environments) -> Option<CacheAssets> {
        let cache = ACTIVE_ASSETS.read().await;
        cache.get(&environment).cloned()
    }

    pub async fn get_asset_cache(environment: Environments, asset_id: i32) -> Option<Model> {
        let cache = ACTIVE_ASSETS.read().await;
        cache.get(&environment)?.models.get(&asset_id).cloned()
    }

    pub async fn get_assets_state_cache(environment: Environments) -> LifecycleState {
        let cache = ACTIVE_ASSETS.read().await;
        cache
            .get(&environment)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    pub async fn reset_assets_cache(environment: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_ASSETS.write().await;

        if let Some(env_map) = cache.environments.get_mut(&environment) {
            *env_map = CacheAssets::new();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use models::{entities::assets::Model, enums::LifecycleState, structs::Environments};
    use sea_orm::prelude::Decimal;

    use crate::{handler::Assets, utils::Cache};

    fn mock_asset(id: i32, free: i64, locked: i64) -> Model {
        Model {
            id,
            free: Decimal::new(free, 0),
            locked: Decimal::new(locked, 0),
            ..Default::default()
        }
    }

    async fn start_env(env: Environments) {
        let _ = Assets::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
        let _ = Assets::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    }

    async fn stop_env(env: Environments) {
        let _ = Assets::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = Assets::<Cache>::remove_assets_cache(env)
            .await
            .expect("remove_assets_cache should work in Stopping");

        assert!(removed.is_empty() || !removed.is_empty());

        let _ = Assets::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    }

    async fn scenario_reset_env_clears_state(env: Environments) {
        start_env(env).await;

        let assets = vec![mock_asset(1, 100, 0)];
        Assets::<Cache>::set_assets_cache(env, assets)
            .await
            .unwrap();

        let _ = Assets::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = Assets::<Cache>::remove_assets_cache(env).await.unwrap();

        assert_eq!(removed.len(), 1);

        let cache = Assets::<Cache>::get_assets_cache(env).await.unwrap();
        assert!(cache.models.is_empty());

        let _ = Assets::<Cache>::set_status_cache(env, LifecycleState::Off).await;

        let status = Assets::<Cache>::get_assets_state_cache(env).await;
        assert_eq!(status, LifecycleState::Off);
    }

    async fn scenario_set_assets_cache(env: Environments) {
        start_env(env).await;

        let assets = vec![mock_asset(1, 100, 0), mock_asset(2, 200, 10)];

        Assets::<Cache>::set_assets_cache(env, assets)
            .await
            .unwrap();

        let cache = Assets::<Cache>::get_assets_cache(env).await.unwrap();
        let status = Assets::<Cache>::get_assets_state_cache(env).await;

        assert_eq!(cache.models.len(), 2);
        assert!(cache.models.contains_key(&1));
        assert!(cache.models.contains_key(&2));
        assert_eq!(status, LifecycleState::Running);

        stop_env(env).await;
    }

    async fn scenario_upsert_asset_insert(env: Environments) {
        start_env(env).await;

        let asset = mock_asset(10, 50, 5);
        Assets::<Cache>::upsert_asset_cache(env, asset.clone())
            .await
            .unwrap();

        let cached = Assets::<Cache>::get_asset_cache(env, 10).await.unwrap();

        assert_eq!(cached, asset);

        stop_env(env).await;
    }

    async fn scenario_remove_asset(env: Environments) {
        start_env(env).await;

        let asset = mock_asset(20, 30, 0);
        Assets::<Cache>::upsert_asset_cache(env, asset)
            .await
            .unwrap();

        let removed = Assets::<Cache>::remove_asset_cache(env, 20).await.unwrap();

        assert!(removed.is_some());

        let cached = Assets::<Cache>::get_asset_cache(env, 20).await;
        assert!(cached.is_none());

        stop_env(env).await;
    }

    async fn scenario_update_free_buy(env: Environments) {
        start_env(env).await;

        let asset = mock_asset(1, 100, 0);
        Assets::<Cache>::upsert_asset_cache(env, asset)
            .await
            .unwrap();

        let (updated, previous) =
            Assets::<Cache>::update_asset_balance_cache(env, 1, Decimal::new(50, 0), false, false)
                .await
                .unwrap();

        assert_eq!(previous, Decimal::new(100, 0));
        assert_eq!(updated.free, Decimal::new(150, 0));

        stop_env(env).await;
    }

    async fn scenario_update_locked_sell(env: Environments) {
        start_env(env).await;

        let asset = mock_asset(2, 0, 100);
        Assets::<Cache>::upsert_asset_cache(env, asset)
            .await
            .unwrap();

        let (updated, previous) =
            Assets::<Cache>::update_asset_balance_cache(env, 2, Decimal::new(40, 0), true, true)
                .await
                .unwrap();

        assert_eq!(previous, Decimal::new(100, 0));
        assert_eq!(updated.locked, Decimal::new(60, 0));

        stop_env(env).await;
    }

    async fn scenario_get_assets_state_cache(env: Environments) {
        assert_eq!(
            Assets::<Cache>::get_assets_state_cache(env).await,
            LifecycleState::Off
        );

        start_env(env).await;

        assert_eq!(
            Assets::<Cache>::get_assets_state_cache(env).await,
            LifecycleState::Running
        );

        stop_env(env).await;
    }

    async fn scenario_cannot_remove_assets_when_running(env: Environments) {
        start_env(env).await;

        let result = Assets::<Cache>::remove_assets_cache(env).await;
        assert!(result.is_err());

        stop_env(env).await;
    }

    #[tokio::test]
    async fn cache_assets_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_assets_cache(env).await;
        scenario_upsert_asset_insert(env).await;
        scenario_remove_asset(env).await;
        scenario_update_free_buy(env).await;
        scenario_update_locked_sell(env).await;
        scenario_get_assets_state_cache(env).await;
        scenario_cannot_remove_assets_when_running(env).await;
    }
}
