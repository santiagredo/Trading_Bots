use models::entities::pair_assets::Model;

use crate::{
    config::get_config,
    types::PairAssets,
    utils::{Core, Data, Logic, Outcome, OutcomeError},
};

impl PairAssets<Core> {
    pub async fn insert_pair_asset(model: Model) -> Outcome<Model, String, String> {
        let data_type = PairAssets::<Logic>::insert_pair_asset(model)
            .map_err(|err| OutcomeError::Failure(err))?;

        PairAssets::<Data>::insert_pair_asset(&get_config().await.db, data_type).await
    }

    pub async fn select_pair_asset(model: Model) -> Outcome<Model, String, String> {
        let data_type = PairAssets::<Logic>::select_pair_asset(model)
            .map_err(|err| OutcomeError::Failure(err))?;

        PairAssets::<Data>::select_pair_asset(&get_config().await.db, data_type).await
    }

    pub async fn select_all_pair_assets() -> Outcome<Vec<Model>, String, String> {
        PairAssets::<Data>::select_all_pair_assets(&get_config().await.db).await
    }

    pub async fn update_pair_asset(model: Model) -> Outcome<Model, String, String> {
        let data_type = PairAssets::<Logic>::update_pair_asset(model)
            .map_err(|err| OutcomeError::Failure(err))?;

        PairAssets::<Data>::update_pair_asset(&get_config().await.db, data_type).await
    }
}
