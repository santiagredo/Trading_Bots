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
