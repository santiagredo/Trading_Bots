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

    pub async fn start_active_assets() -> Result<(), Response> {
        if Self::get_active_assets()
            .await
            .is_none_or(|assets| assets.is_empty())
        {
            let assets = Assets::default().select_assets().await?;
            Self::set_active_assets(Some(assets)).await;
        }

        Ok(())
    }

    pub async fn stop_active_assets() {
        Self::set_active_assets(None).await;
    }
}
