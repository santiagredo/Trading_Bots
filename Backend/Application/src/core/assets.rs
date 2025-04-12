use models::entities::assets::Model;

use crate::{
    config::get_config,
    types::Assets,
    utils::{Core, Data, Logic, Outcome, OutcomeError},
};

impl Assets<Core> {
    pub async fn insert_asset(asset: Model) -> Outcome<Model, String, String> {
        let data_type =
            Assets::<Logic>::insert_asset(asset).map_err(|err| OutcomeError::Failure(err))?;

        Assets::<Data>::insert_asset(&get_config().await.db, data_type).await
    }

    pub async fn select_asset(asset: Model) -> Outcome<Model, String, String> {
        let data_type =
            Assets::<Logic>::select_asset(asset).map_err(|err| OutcomeError::Failure(err))?;

        Assets::<Data>::select_asset(&get_config().await.db, data_type).await
    }

    pub async fn select_assets() -> Outcome<Vec<Model>, String, String> {
        // Assets::<Logic>::select_assets();

        Assets::<Data>::select_assets(&get_config().await.db).await
    }

    pub async fn update_asset(asset: Model) -> Outcome<Model, String, String> {
        let data_type =
            Assets::<Logic>::update_asset(asset).map_err(|err| OutcomeError::Failure(err))?;

        Assets::<Data>::update_asset(&get_config().await.db, data_type).await
    }

    pub async fn delete_asset(asset: Model) -> Outcome<u64, String, String> {
        let data_type =
            Assets::<Logic>::delete_asset(asset).map_err(|err| OutcomeError::Failure(err))?;

        Assets::<Data>::delete_asset(&get_config().await.db, data_type).await
    }
}
