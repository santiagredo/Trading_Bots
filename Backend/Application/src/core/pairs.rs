use std::collections::HashMap;

use models::{entities::pairs::Model, structs::AssetRequest};

use crate::{
    handler::{Assets, Pairs, DBC},
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl Pairs<Core> {
    // db
    pub async fn insert_pair_core(self) -> Result<Model, Response> {
        let env = self.environment;

        let base_asset_request = AssetRequest {
            id: self.model.base_asset_id.clone(),
            ..Default::default()
        };

        let base_asset = Assets::from_request(base_asset_request)
            .with_env(env)
            .select_asset()
            .await?;

        let quote_asset_request = AssetRequest {
            id: self.model.quote_asset_id.clone(),
            ..Default::default()
        };

        let quote_asset = Assets::from_request(quote_asset_request)
            .with_env(env)
            .select_asset()
            .await?;

        self.next_phase::<Logic>()
            .insert_pair_logic(base_asset, quote_asset)
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .insert_pair_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_pair_core(self) -> Result<Option<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_pair_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_pairs_core(self) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_pairs_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn update_pair_core(self) -> Result<Model, Response> {
        let env = self.environment;

        let base_asset_request = AssetRequest {
            id: self.model.base_asset_id.clone(),
            ..Default::default()
        };

        let base_asset = Assets::from_request(base_asset_request)
            .with_env(env)
            .select_asset()
            .await?;

        let quote_asset_request = AssetRequest {
            id: self.model.quote_asset_id.clone(),
            ..Default::default()
        };

        let quote_asset = Assets::from_request(quote_asset_request)
            .with_env(env)
            .select_asset()
            .await?;

        self.next_phase::<Logic>()
            .update_pair_logic(base_asset, quote_asset)
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .update_pair_data(&DBC::db(&env).await?)
            .await
    }

    // cache
    pub async fn get_active_pairs_core(self) -> Option<HashMap<i32, Model>> {
        let env = self.environment;
        Pairs::<Cache>::get_active_pairs_cache(&env).await
    }

    pub async fn get_active_pair_core(self) -> Option<Model> {
        let env = self.environment;
        let id = self.model.id.unwrap_or_default();

        Pairs::<Cache>::get_active_pair_cache(&env, &id).await
    }

    pub async fn set_active_pair_core(self, is_remove: bool) -> Model {
        let environment = self.environment;
        let pair = Pairs::into_model(self.model);

        Pairs::<Cache>::set_active_pair_cache(&environment, pair, is_remove).await
    }

    pub async fn start_active_pairs_core(self) -> Result<(), Response> {
        let env = self.environment;

        if !Pairs::<Cache>::get_active_pairs_status_cache(&env).await {
            let pairs_request = Pairs::default().with_env(env);

            let models = pairs_request.select_pairs().await?;

            Pairs::<Cache>::set_active_pairs_cache(&env, models).await;
        }

        Ok(())
    }

    pub async fn stop_active_pairs_core(self) {
        let env = self.environment;

        Pairs::<Cache>::stop_active_pairs_cache(&env).await
    }
}
