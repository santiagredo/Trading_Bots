use std::marker::PhantomData;

use models::{entities::assets::Model, structs::AssetRequest};
use sea_orm::prelude::Decimal;

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct Assets<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: AssetRequest,
}

impl<Phase> Assets<Phase> {
    pub fn next_phase<Next>(self) -> Assets<Next> {
        Assets {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

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

    pub async fn update_asset_value(
        self,
        value: Decimal,
        is_locked: bool,
        is_sell: bool,
    ) -> Result<Model, Response> {
        self.next_phase()
            .update_asset_value_core(value, is_locked, is_sell)
            .await
    }

    pub async fn delete_asset(self) -> Result<u64, Response> {
        self.next_phase().delete_asset_core().await
    }
}
