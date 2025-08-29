use models::entities::assets::Model;

use crate::{
    config::get_config,
    types::Assets,
    utils::{handle_user_err, Core, Data, Logic, Response},
};

impl Assets<Core> {
    pub async fn insert_asset_core(self) -> Result<Model, Response> {
        let logic_type = self
            .next_phase::<Logic>()
            .insert_asset_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .insert_asset_data(&get_config().await.db)
            .await
    }

    pub async fn select_asset_core(self) -> Result<Option<Model>, Response> {
        self.next_phase::<Data>()
            .select_asset_data(&get_config().await.db)
            .await
    }

    pub async fn select_assets_core(self) -> Result<Vec<Model>, Response> {
        self.next_phase::<Data>()
            .select_assets_data(&get_config().await.db)
            .await
    }

    pub async fn update_asset_core(self) -> Result<Model, Response> {
        let logic_type = self
            .next_phase::<Logic>()
            .update_asset_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .update_asset_data(&get_config().await.db)
            .await
    }

    pub async fn delete_asset_core(self) -> Result<u64, Response> {
        let logic_type = self
            .next_phase::<Logic>()
            .delete_asset_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .delete_asset_data(&get_config().await.db)
            .await
    }
}
