use models::{entities::pairs::Model, structs::AssetRequest};

use crate::{
    config::get_config,
    types::{Assets, Pairs},
    utils::{handle_user_err, Core, Data, Logic, Response},
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

        let logic_type = self
            .next_phase::<Logic>()
            .insert_pair_logic(base_asset, quote_asset)
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .insert_pair_data(&get_config().await.db)
            .await
    }

    pub async fn select_pair_core(self) -> Result<Option<Model>, Response> {
        self.next_phase::<Data>()
            .select_pair_data(&get_config().await.db)
            .await
    }

    pub async fn select_pairs_core(self) -> Result<Vec<Model>, Response> {
        self.next_phase::<Data>()
            .select_pairs_data(&get_config().await.db)
            .await
    }

    // pub async fn select_all_pairs() -> Outcome<Vec<Model>, String, String> {
    //     Pairs::<Data>::select_all_pairs(&get_config().await.db).await
    // }

    // pub async fn select_pairs_by_ids(ids: Vec<i32>) -> Outcome<Vec<Model>, String, String> {
    //     Pairs::<Data>::select_pairs_by_ids(&get_config().await.db, ids).await
    // }

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

        let logic_type = self
            .next_phase::<Logic>()
            .update_pair_logic(base_asset, quote_asset)
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .update_pair_data(&get_config().await.db)
            .await
    }
}
