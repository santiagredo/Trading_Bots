use std::{collections::HashMap, sync::Arc};

use models::entities::assets::Model;
use once_cell::sync::Lazy;
use sea_orm::prelude::Decimal;
use tokio::sync::RwLock;

use crate::{
    handler::Assets,
    utils::{Cache, Response},
};

static ACTIVE_ASSETS: Lazy<Arc<RwLock<Option<HashMap<i32, Model>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

static POSTING_ASSETS: Lazy<Arc<RwLock<Option<HashMap<i32, bool>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Assets<Cache> {
    async fn set_active_assets(assets: Option<Vec<Model>>) -> Option<Vec<Model>> {
        let mut active_assets = ACTIVE_ASSETS.write().await;

        let Some(assets) = assets else {
            *active_assets = None;
            return None;
        };

        let mut assets_map: HashMap<i32, Model> = HashMap::new();

        for asset in assets.iter() {
            assets_map.insert(asset.id, asset.clone());
        }

        *active_assets = Some(assets_map);

        Some(assets)
    }

    pub async fn set_active_asset(asset: Model, is_remove: bool) -> Model {
        let mut active_assets = ACTIVE_ASSETS.write().await;

        let Some(assets_map) = active_assets.as_mut() else {
            return asset;
        };

        if is_remove {
            assets_map.remove(&asset.id);
        } else {
            assets_map.insert(asset.id, asset.clone());
        }

        asset
    }

    pub async fn set_active_asset_value(
        key: &i32,
        value: Decimal,
        is_locked: bool,
        is_sell: bool,
    ) -> Option<Model> {
        let mut assets_guard = ACTIVE_ASSETS.write().await;
        let assets = assets_guard.as_mut()?;

        let asset = assets.get_mut(key)?;

        match (is_locked, is_sell) {
            (true, true) => asset.locked -= value,
            (true, false) => asset.locked += value,
            (false, true) => asset.free -= value,
            (false, false) => asset.free += value,
        }

        Some(asset.clone())
    }

    pub async fn get_active_assets() -> Option<HashMap<i32, Model>> {
        let assets = ACTIVE_ASSETS.read().await;
        assets.clone()
    }

    pub async fn get_active_asset(key: &i32) -> Option<Model> {
        let assets_guard = ACTIVE_ASSETS.read().await;
        let assets = assets_guard.as_ref()?;
        assets.get(key).cloned()
    }

    pub async fn set_posting_assets_cache(assets: Option<Vec<i32>>) -> Option<Vec<i32>> {
        let mut posting_assets = POSTING_ASSETS.write().await;

        let Some(assets) = assets else {
            *posting_assets = None;
            return None;
        };

        let mut posting_assets_map: HashMap<i32, bool> = HashMap::new();

        for asset in assets.iter() {
            posting_assets_map.insert(*asset, false);
        }

        *posting_assets = Some(posting_assets_map);

        Some(assets)
    }

    pub async fn set_posting_asset_cache(asset: i32, is_posting: bool, is_remove: bool) -> i32 {
        let mut posting_assets = POSTING_ASSETS.write().await;

        let Some(assets_map) = posting_assets.as_mut() else {
            return asset;
        };

        if is_remove {
            assets_map.remove(&asset);
        } else {
            assets_map.insert(asset, is_posting);
        }

        asset
    }

    pub async fn get_posting_assets_cache() -> Option<HashMap<i32, bool>> {
        let posting_assets = POSTING_ASSETS.read().await;

        posting_assets.clone()
    }

    pub async fn get_posting_asset_cache(key: &i32) -> Option<bool> {
        let posting_assets = POSTING_ASSETS.read().await;

        let Some(assets_map) = posting_assets.as_ref() else {
            return None;
        };

        assets_map.get(key).cloned()
    }

    pub async fn start_active_assets() -> Result<(), Response> {
        if Self::get_active_assets()
            .await
            .is_none_or(|assets| assets.is_empty())
        {
            let assets = Assets::default().select_assets().await?;
            let assets_ids: Vec<i32> = assets.iter().map(|asset| asset.id.clone()).collect();

            Self::set_posting_assets_cache(Some(assets_ids)).await;
            Self::set_active_assets(Some(assets)).await;
        }

        Ok(())
    }

    pub async fn stop_active_assets() {
        Self::set_posting_assets_cache(None).await;
        Self::set_active_assets(None).await;
    }
}
