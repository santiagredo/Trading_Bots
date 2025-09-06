use std::collections::HashMap;

use models::entities::assets::Model;
use sea_orm::prelude::Decimal;

use crate::{
    config::get_config,
    handler::Assets,
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl Assets<Core> {
    pub async fn get_posting_assets_core() -> Option<HashMap<i32, bool>> {
        Assets::<Cache>::get_posting_assets_cache().await
    }

    pub async fn get_posting_asset_core(key: &i32) -> Option<bool> {
        Assets::<Cache>::get_posting_asset_cache(key).await
    }

    pub async fn set_posting_asset_core(asset: i32, is_posting: bool, is_remove: bool) -> i32 {
        Assets::<Cache>::set_posting_asset_cache(asset, is_posting, is_remove).await
    }

    pub async fn insert_asset_core(self) -> Result<Model, Response> {
        let asset = self
            .next_phase::<Logic>()
            .insert_asset_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .insert_asset_data(&get_config().await.db)
            .await?;

        Self::set_posting_asset_core(asset.id, false, false).await;

        Ok(Assets::<Cache>::set_active_asset(asset, false).await)
    }

    pub async fn select_asset_core(self) -> Result<Option<Model>, Response> {
        let memory_asset =
            Assets::<Cache>::get_active_asset(&self.model.id.unwrap_or_default()).await;

        if memory_asset.is_some() {
            return Ok(memory_asset);
        }

        self.next_phase::<Data>()
            .select_asset_data(&get_config().await.db)
            .await
    }

    pub async fn select_assets_core(self) -> Result<Vec<Model>, Response> {
        let memory_assets = Assets::<Cache>::get_active_assets().await;

        if let Some(memory_assets) = memory_assets {
            let results = memory_assets
                .into_iter()
                .map(|(_, val)| val.to_owned())
                .collect::<Vec<Model>>();

            return Ok(results);
        }

        self.next_phase::<Data>()
            .select_assets_data(&get_config().await.db)
            .await
    }

    pub async fn update_asset_core(self) -> Result<Model, Response> {
        let asset = self
            .next_phase::<Logic>()
            .update_asset_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .update_asset_data(&get_config().await.db)
            .await?;

        Ok(Assets::<Cache>::set_active_asset(asset, false).await)
    }

    pub async fn update_asset_value_core(
        self,
        value: Decimal,
        is_locked: bool,
        is_sell: bool,
    ) -> Result<Model, Response> {
        let Some(memory_asset) = Assets::<Cache>::set_active_asset_value(
            &self.model.id.unwrap_or_default(),
            value,
            is_locked,
            is_sell,
        )
        .await
        else {
            return Err(Response::not_found("Memory asset".to_string()));
        };

        Assets::default()
            .into_request(memory_asset)
            .next_phase()
            .update_asset_core()
            .await
    }

    pub async fn delete_asset_core(self) -> Result<u64, Response> {
        let asset = Assets::into_model(self.model.clone());
        Self::set_posting_asset_core(asset.id, false, true).await;

        Assets::<Cache>::set_active_asset(asset, true).await;

        self.next_phase::<Logic>()
            .delete_asset_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .delete_asset_data(&get_config().await.db)
            .await
    }
}
