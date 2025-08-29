use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use models::{entities::assets::Model, structs::AssetRequest};
use once_cell::sync::Lazy;
use sea_orm::prelude::Decimal;
use tokio::sync::RwLock;

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct Assets<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: AssetRequest,
}

static ACTIVE_ASSETS: Lazy<Arc<RwLock<Option<HashMap<i32, Model>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Assets {
    pub fn new(model: AssetRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: AssetRequest {
                ..Default::default()
            },
        }
    }

    pub fn into_request(mut self, model: Model) -> Self {
        let asset_request = AssetRequest {
            id: Some(model.id),
            name: Some(model.name),
            ticker: Some(model.ticker),
            free: Some(model.free),
            locked: Some(model.locked),
        };

        self.model = asset_request;
        self
    }

    pub fn into_model(asset: AssetRequest) -> Model {
        Model {
            id: asset.id.unwrap_or_default(),
            name: asset.name.unwrap_or_default(),
            ticker: asset.ticker.unwrap_or_default(),
            free: asset.free.unwrap_or_default(),
            locked: asset.locked.unwrap_or_default(),
        }
    }

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

impl<Phase> Assets<Phase> {
    pub fn next_phase<Next>(self) -> Assets<Next> {
        Assets {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl Assets<Types> {
    pub async fn insert_asset(self) -> Result<Model, Response> {
        let asset = self.next_phase().insert_asset_core().await?;

        Ok(Self::set_active_asset(asset, false).await)
    }

    pub async fn select_asset(self) -> Result<Option<Model>, Response> {
        let memory_asset = Self::get_active_asset(&self.model.id.unwrap_or_default()).await;

        if memory_asset.is_some() {
            return Ok(memory_asset);
        }

        self.next_phase().select_asset_core().await
    }

    pub async fn select_assets(self) -> Result<Vec<Model>, Response> {
        let memory_assets = Self::get_active_assets().await;

        if let Some(memory_assets) = memory_assets {
            let results = memory_assets
                .into_iter()
                .map(|(_, val)| val.to_owned())
                .collect::<Vec<Model>>();

            return Ok(results);
        }

        self.next_phase().select_assets_core().await
    }

    pub async fn update_asset(self) -> Result<Model, Response> {
        let asset = self.next_phase().update_asset_core().await?;

        Ok(Self::set_active_asset(asset, false).await)
    }

    pub async fn update_asset_value(
        self,
        value: Decimal,
        is_locked: bool,
        is_sell: bool,
    ) -> Result<Model, Response> {
        let Some(memory_asset) = Self::set_active_asset_value(
            &self.model.id.unwrap_or_default(),
            value,
            is_locked,
            is_sell,
        )
        .await
        else {
            return Err(Response {
                code: 500,
                message: format!("Memory asset not found"),
            });
        };

        let asset = self
            .into_request(memory_asset)
            .next_phase()
            .update_asset_core()
            .await?;

        Ok(asset)
    }

    pub async fn delete_asset(self) -> Result<u64, Response> {
        let asset = Self::into_model(self.model.clone());
        Self::set_active_asset(asset, true).await;

        self.next_phase().delete_asset_core().await
    }
}
