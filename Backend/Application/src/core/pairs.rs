use models::{entities::pairs::Model, structs::AssetRequest};

use crate::{
    config::get_config,
    handler::{Assets, Pairs},
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl Pairs<Core> {
    pub async fn insert_pair_core(self) -> Result<Model, Response> {
        let base_asset = Assets::new(AssetRequest {
            id: self.model.base_asset_id.clone(),
            ..Default::default()
        })
        .select_asset()
        .await?;

        let quote_asset = Assets::new(AssetRequest {
            id: self.model.quote_asset_id.clone(),
            ..Default::default()
        })
        .select_asset()
        .await?;

        let pair = self
            .next_phase::<Logic>()
            .insert_pair_logic(base_asset, quote_asset)
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .insert_pair_data(&get_config().await.db)
            .await?;

        Ok(Pairs::<Cache>::set_active_pair(pair, false).await)
    }

    pub async fn select_pair_core(self) -> Result<Option<Model>, Response> {
        let memory_pair = Pairs::<Cache>::get_active_pair(&self.model.id.unwrap_or_default()).await;

        if memory_pair.is_some() {
            return Ok(memory_pair);
        }

        self.next_phase::<Data>()
            .select_pair_data(&get_config().await.db)
            .await
    }

    pub async fn select_pairs_core(self) -> Result<Vec<Model>, Response> {
        let memory_pairs = Pairs::<Cache>::get_active_pairs().await;

        if let Some(pairs) = memory_pairs {
            let pairs: Vec<Model> = pairs
                .values()
                .into_iter()
                .map(|val| val.to_owned())
                .collect();

            return Ok(pairs);
        }

        self.next_phase::<Data>()
            .select_pairs_data(&get_config().await.db)
            .await
    }

    pub async fn update_pair_core(self) -> Result<Model, Response> {
        let base_asset = Assets::new(AssetRequest {
            id: self.model.base_asset_id.clone(),
            ..Default::default()
        })
        .select_asset()
        .await?;

        let quote_asset = Assets::new(AssetRequest {
            id: self.model.quote_asset_id.clone(),
            ..Default::default()
        })
        .select_asset()
        .await?;

        let pair = self
            .next_phase::<Logic>()
            .update_pair_logic(base_asset, quote_asset)
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .update_pair_data(&get_config().await.db)
            .await?;

        Ok(Pairs::<Cache>::set_active_pair(pair, false).await)
    }
}
