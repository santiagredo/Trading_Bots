use std::marker::PhantomData;

use models::{
    entities::assets::Model,
    enums::LifecycleState,
    structs::{AssetRequest, CacheAssets, Environments},
};
use sea_orm::prelude::Decimal;
use tokio_util::sync::CancellationToken;

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct Assets<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
    pub model: AssetRequest,
}

impl<Phase> Assets<Phase> {
    pub fn next_phase<Next>(self) -> Assets<Next> {
        Assets {
            phase: PhantomData::<Next>,
            environment: self.environment,
            model: self.model,
        }
    }
}

impl Assets {
    pub fn new(model: AssetRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model,
        }
    }

    pub fn from_request(request: AssetRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: request,
            environment: Environments::DEV,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model: AssetRequest {
                ..Default::default()
            },
        }
    }

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: self.phase,
            environment: environment,
            model: self.model,
        }
    }

    pub fn into_request(mut self, model: Model) -> Self {
        let asset_request = AssetRequest {
            id: Some(model.id),
            name: Some(model.name),
            ticker: Some(model.ticker),
            free: Some(model.free),
            locked: Some(model.locked),
            last_update: model.last_update,
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
            last_update: asset.last_update,
        }
    }

    // db
    pub async fn insert_asset(self) -> Result<Model, Response> {
        self.next_phase().insert_asset_core().await
    }

    pub async fn select_asset(self) -> Result<Option<Model>, Response> {
        self.next_phase().select_asset_core().await
    }

    pub async fn select_assets(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_assets_core().await
    }

    pub async fn update_asset(self) -> Result<Model, Response> {
        self.next_phase().update_asset_core().await
    }

    pub async fn delete_asset(self) -> Result<u64, Response> {
        self.next_phase().delete_asset_core().await
    }

    // cache
    pub async fn get_assets(self) -> Option<CacheAssets> {
        self.next_phase().get_assets_core().await
    }

    pub async fn get_asset(self) -> Option<Model> {
        self.next_phase().get_asset_core().await
    }

    pub async fn get_assets_state(self) -> LifecycleState {
        self.next_phase().get_assets_state_core().await
    }

    pub async fn upsert_asset(self) -> Result<(), String> {
        self.next_phase().upsert_asset_core().await
    }

    pub async fn remove_asset(self) -> Result<Option<Model>, String> {
        self.next_phase().remove_asset_core().await
    }

    pub async fn set_asset_value(
        self,
        value: Decimal,
        is_locked: bool,
        is_sell: bool,
    ) -> Result<(Model, Decimal), Response> {
        self.next_phase()
            .set_asset_value_core(value, is_locked, is_sell)
            .await
    }

    pub async fn start_assets(self, token: &CancellationToken) -> Result<(), Response> {
        self.next_phase().start_assets_core(&token).await
    }

    pub async fn stop_assets(self) -> Result<(), Response> {
        self.next_phase().stop_assets_core().await
    }

    pub async fn reset_assets(self) -> Result<(), Response> {
        self.next_phase().reset_assets_core().await
    }
}
