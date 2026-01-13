use std::collections::HashMap;

use models::{entities::pairs::Model, enums::LifecycleState, structs::AssetRequest};

use crate::{
    handler::{Assets, Pairs, DBC},
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl Pairs<Core> {
    /* ===========================
     * DB
     * ===========================
     */

    pub async fn insert_pair_core(self) -> Result<Model, Response> {
        let env = self.environment;

        let base_asset = Assets::from_request(AssetRequest {
            id: self.model.base_asset_id.clone(),
            ..Default::default()
        })
        .with_env(env)
        .select_asset()
        .await?;

        let quote_asset = Assets::from_request(AssetRequest {
            id: self.model.quote_asset_id.clone(),
            ..Default::default()
        })
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

        let base_asset = Assets::from_request(AssetRequest {
            id: self.model.base_asset_id.clone(),
            ..Default::default()
        })
        .with_env(env)
        .select_asset()
        .await?;

        let quote_asset = Assets::from_request(AssetRequest {
            id: self.model.quote_asset_id.clone(),
            ..Default::default()
        })
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

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_pairs_core(self) -> Option<HashMap<i32, Model>> {
        Pairs::<Cache>::get_pairs_cache(self.environment)
            .await
            .map(|c| c.models)
    }

    pub async fn get_pair_core(self) -> Option<Model> {
        let id = self.model.id.unwrap_or_default();
        Pairs::<Cache>::get_pair_cache(self.environment, id).await
    }

    pub async fn get_pairs_state_core(self) -> LifecycleState {
        Pairs::<Cache>::get_cache_state(self.environment).await
    }

    /* ===========================
     * CACHE (WRITE)
     * ===========================
     */

    pub async fn upsert_pair_core(self) -> Result<(), Response> {
        let env = self.environment;
        let model = Pairs::into_model(self.model);

        Pairs::<Cache>::upsert_pair_cache(env, model)
            .await
            .map_err(|err| Response {
                code: 500,
                message: err,
            })
    }

    pub async fn remove_pair_core(self) -> Result<Option<Model>, Response> {
        let env = self.environment;
        let id = self.model.id.unwrap_or_default();

        Pairs::<Cache>::remove_pair_cache(env, id)
            .await
            .map_err(|err| Response {
                code: 500,
                message: err,
            })
    }

    /* ===========================
     * START ACTIVE PAIRS
     * ===========================
     */

    pub async fn start_pairs_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STARTING
        if let Err(err) = Pairs::<Cache>::set_status_cache(env, LifecycleState::Starting).await {
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // Load active pairs from DB
        let pairs_request = Pairs::default().with_env(env);

        let pairs = match pairs_request.select_pairs().await {
            Ok(p) => p,
            Err(err) => {
                let _ = Pairs::<Cache>::reset_pairs_cache(env).await;
                return Err(err);
            }
        };

        // RUNNING
        if let Err(err) = Pairs::<Cache>::set_status_cache(env, LifecycleState::Running).await {
            let _ = Pairs::<Cache>::reset_pairs_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // Populate cache
        if let Err(err) = Pairs::<Cache>::set_pairs_cache(env, pairs).await {
            let _ = Pairs::<Cache>::reset_pairs_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        Ok(())
    }

    /* ===========================
     * STOP ACTIVE PAIRS
     * ===========================
     */

    pub async fn stop_pairs_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STOPPING
        if let Err(err) = Pairs::<Cache>::set_status_cache(env, LifecycleState::Stopping).await {
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // Remove pairs
        if let Err(err) = Pairs::<Cache>::remove_pairs_cache(env).await {
            let _ = Pairs::<Cache>::reset_pairs_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // OFF
        if let Err(err) = Pairs::<Cache>::set_status_cache(env, LifecycleState::Off).await {
            let _ = Pairs::<Cache>::reset_pairs_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        Ok(())
    }

    pub async fn reset_pairs_core(self) -> Result<(), Response> {
        Pairs::<Cache>::reset_pairs_cache(self.environment)
            .await
            .map_err(Response::server_error)
    }
}
