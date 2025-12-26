use std::{collections::HashMap, sync::Arc};

use models::{
    entities::assets::Model,
    structs::{CacheAsset, Environments},
};
use once_cell::sync::Lazy;
use sea_orm::prelude::Decimal;
use tokio::{sync::RwLock, task::JoinHandle};

use crate::{handler::Assets, utils::Cache};

#[derive(Default)]
struct CacheEnvironments {
    pub environments: HashMap<Environments, CacheAssets>,
}

#[derive(Default)]
struct CacheAssets {
    pub is_initialized: bool,
    pub models: HashMap<i32, CacheAsset>,
    pub join_handle: Option<JoinHandle<()>>,
}

static ACTIVE_ASSETS: Lazy<Arc<RwLock<CacheEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheEnvironments::default())));

impl Assets<Cache> {
    pub async fn set_active_assets_cache(
        environment: &Environments,
        assets: Vec<Model>,
    ) -> Vec<Model> {
        let mut cache_assets = ACTIVE_ASSETS.write().await;

        let env_map = cache_assets
            .environments
            .entry(*environment)
            .or_insert_with(CacheAssets::default);

        for asset in assets.iter() {
            let cache_asset = CacheAsset {
                model: asset.clone(),
            };

            env_map.models.insert(asset.id, cache_asset);
        }

        env_map.is_initialized = true;

        assets
    }

    pub async fn set_active_asset_cache(
        environment: &Environments,
        asset: Model,
        is_remove: bool,
    ) -> Model {
        let mut active_assets = ACTIVE_ASSETS.write().await;

        let env_map = active_assets
            .environments
            .entry(*environment)
            .or_insert_with(CacheAssets::default);

        if is_remove {
            env_map
                .models
                .remove(&asset.id)
                .map(|val| val.model)
                .unwrap_or(asset)
        } else {
            let inner_asset = CacheAsset {
                model: asset.clone(),
            };

            env_map.models.insert(asset.id, inner_asset);

            asset
        }
    }

    pub async fn set_active_asset_value_cache(
        environment: &Environments,
        key: &i32,
        value: Decimal,
        is_locked: bool,
        is_sell: bool,
    ) -> Option<(Model, Decimal)> {
        let mut active_assets = ACTIVE_ASSETS.write().await;

        let Some(env_map) = active_assets.environments.get_mut(environment) else {
            return None;
        };

        if let Some(inner_asset) = env_map.models.get_mut(key) {
            let previous_balance = match is_locked {
                false => inner_asset.model.free.clone(),
                true => inner_asset.model.locked.clone(),
            };

            match (is_locked, is_sell) {
                (true, true) => inner_asset.model.locked -= value,
                (true, false) => inner_asset.model.locked += value,
                (false, true) => inner_asset.model.free -= value,
                (false, false) => inner_asset.model.free += value,
            }

            Some((inner_asset.model.clone(), previous_balance))
        } else {
            None
        }
    }

    pub async fn set_active_assets_join_handle_cache(
        environment: &Environments,
        join_handle: JoinHandle<()>,
    ) {
        let mut cache_assets = ACTIVE_ASSETS.write().await;

        let env_map = cache_assets
            .environments
            .entry(*environment)
            .or_insert_with(CacheAssets::default);

        if let Some(handle) = &env_map.join_handle {
            handle.abort();
        }

        env_map.join_handle = Some(join_handle);
    }

    pub async fn get_active_assets_cache(
        environment: &Environments,
    ) -> Option<HashMap<i32, CacheAsset>> {
        let assets = ACTIVE_ASSETS.read().await;

        let env_map = assets.environments.get(&environment)?;

        Some(env_map.models.clone())
    }

    pub async fn get_active_asset_cache(
        environment: &Environments,
        key: &i32,
    ) -> Option<CacheAsset> {
        let assets = ACTIVE_ASSETS.read().await;

        let env_map = assets.environments.get(&environment)?;

        let cache_asset = env_map.models.get(key)?;

        Some(cache_asset.clone())
    }

    pub async fn get_active_assets_status_cache(environment: &Environments) -> bool {
        let cache_assets = ACTIVE_ASSETS.read().await;

        cache_assets
            .environments
            .get(&environment)
            .map(|val| val.is_initialized)
            .unwrap_or(false)
    }

    pub async fn stop_active_assets_cache(environment: &Environments) -> Result<(), String> {
        let mut active_assets = ACTIVE_ASSETS.write().await;

        let Some(env_map) = active_assets.environments.get_mut(&environment) else {
            return Ok(());
        };

        if let Some(handle) = &env_map.join_handle {
            handle.abort();
        }

        env_map.models = HashMap::new();

        env_map.is_initialized = false;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use models::{entities::assets::Model, structs::Environments};
    use sea_orm::prelude::Decimal;

    use crate::{handler::Assets, utils::Cache};

    // Helpers
    fn mock_asset(id: i32, free: i64, locked: i64) -> Model {
        Model {
            id,
            free: Decimal::new(free, 0),
            locked: Decimal::new(locked, 0),
            ..Default::default()
        }
    }

    async fn reset_env(env: Environments) {
        let _ = Assets::<Cache>::stop_active_assets_cache(&env).await;
    }

    // Scenarios (unit responsibilities)
    async fn scenario_reset_env_clears_state(env: Environments) {
        let assets = vec![mock_asset(1, 100, 0)];

        Assets::<Cache>::set_active_assets_cache(&env, assets).await;
        reset_env(env).await;

        let cache = Assets::<Cache>::get_active_assets_cache(&env)
            .await
            .expect("environment should exist");

        let status = Assets::<Cache>::get_active_assets_status_cache(&env).await;

        assert!(cache.is_empty());
        assert!(!status);
    }

    async fn scenario_set_active_assets_cache(env: Environments) {
        reset_env(env).await;

        let assets = vec![mock_asset(1, 100, 0), mock_asset(2, 200, 10)];

        Assets::<Cache>::set_active_assets_cache(&env, assets).await;

        let cache = Assets::<Cache>::get_active_assets_cache(&env)
            .await
            .expect("cache should exist");

        let status = Assets::<Cache>::get_active_assets_status_cache(&env).await;

        assert_eq!(cache.len(), 2);
        assert!(cache.contains_key(&1));
        assert!(cache.contains_key(&2));
        assert!(status);

        reset_env(env).await;
    }

    async fn scenario_set_active_asset_cache_insert(env: Environments) {
        reset_env(env).await;

        let asset = mock_asset(10, 50, 5);

        Assets::<Cache>::set_active_asset_cache(&env, asset.clone(), false).await;

        let cached = Assets::<Cache>::get_active_asset_cache(&env, &10).await;

        assert_eq!(cached.unwrap().model, asset);

        reset_env(env).await;
    }

    async fn scenario_set_active_asset_cache_remove(env: Environments) {
        reset_env(env).await;

        let asset = mock_asset(20, 30, 0);

        Assets::<Cache>::set_active_asset_cache(&env, asset.clone(), false).await;
        Assets::<Cache>::set_active_asset_cache(&env, asset, true).await;

        let cached = Assets::<Cache>::get_active_asset_cache(&env, &20).await;

        assert!(cached.is_none());

        reset_env(env).await;
    }

    async fn scenario_set_active_asset_value_cache_free_buy(env: Environments) {
        reset_env(env).await;

        let asset = mock_asset(1, 100, 0);
        Assets::<Cache>::set_active_asset_cache(&env, asset, false).await;

        let (updated, previous) = Assets::<Cache>::set_active_asset_value_cache(
            &env,
            &1,
            Decimal::new(50, 0),
            false,
            false,
        )
        .await
        .expect("asset should exist");

        assert_eq!(previous, Decimal::new(100, 0));
        assert_eq!(updated.free, Decimal::new(150, 0));

        reset_env(env).await;
    }

    async fn scenario_set_active_asset_value_cache_locked_sell(env: Environments) {
        reset_env(env).await;

        let asset = mock_asset(2, 0, 100);
        Assets::<Cache>::set_active_asset_cache(&env, asset, false).await;

        let (updated, previous) = Assets::<Cache>::set_active_asset_value_cache(
            &env,
            &2,
            Decimal::new(40, 0),
            true,
            true,
        )
        .await
        .expect("asset should exist");

        assert_eq!(previous, Decimal::new(100, 0));
        assert_eq!(updated.locked, Decimal::new(60, 0));

        reset_env(env).await;
    }

    async fn scenario_get_active_assets_status_cache(env: Environments) {
        reset_env(env).await;

        assert!(!Assets::<Cache>::get_active_assets_status_cache(&env).await);

        Assets::<Cache>::set_active_assets_cache(&env, vec![mock_asset(1, 10, 0)]).await;

        assert!(Assets::<Cache>::get_active_assets_status_cache(&env).await);

        reset_env(env).await;
    }

    #[tokio::test]
    async fn cache_assets_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_active_assets_cache(env).await;
        scenario_set_active_asset_cache_insert(env).await;
        scenario_set_active_asset_cache_remove(env).await;
        scenario_set_active_asset_value_cache_free_buy(env).await;
        scenario_set_active_asset_value_cache_locked_sell(env).await;
        scenario_get_active_assets_status_cache(env).await;

        reset_env(env).await;
    }
}
