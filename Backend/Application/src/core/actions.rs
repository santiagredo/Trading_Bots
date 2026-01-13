use std::collections::HashMap;

use models::{
    entities::{actions::Model, assets, pairs},
    enums::LifecycleState,
    structs::Ticker,
};

use crate::{
    handler::{Actions, Strategies, DBC},
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl Actions<Core> {
    /* ===========================
     * DB
     * ===========================
     */

    pub async fn insert_action_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase::<Logic>()
            .insert_action_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .insert_action_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_action_core(self) -> Result<Option<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_action_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_actions_core(self) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_actions_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn update_action_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase::<Logic>()
            .update_action_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .update_action_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn delete_action_core(self) -> Result<u64, Response> {
        let env = self.environment;

        self.next_phase::<Logic>()
            .delete_action_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .delete_action_data(&DBC::db(&env).await?)
            .await
    }

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_actions_core(self) -> Option<HashMap<i32, Model>> {
        Actions::<Cache>::get_actions_cache(self.environment)
            .await
            .map(|c| c.models)
    }

    pub async fn get_action_core(self) -> Option<Model> {
        let id = self.model.id.or(self.model.strategy_id).unwrap_or_default();

        Actions::<Cache>::get_action_cache(self.environment, id).await
    }

    pub async fn get_actions_state_core(self) -> LifecycleState {
        Actions::<Cache>::get_cache_state(self.environment).await
    }

    /* ===========================
     * CACHE (WRITE)
     * ===========================
     */

    pub async fn upsert_action_core(self) -> Result<(), Response> {
        let env = self.environment;
        let model = Actions::into_model(self.model);

        Actions::<Cache>::upsert_action_cache(env, model)
            .await
            .map_err(|err| Response {
                code: 500,
                message: err,
            })
    }

    pub async fn remove_action_core(self) -> Result<Option<Model>, Response> {
        let env = self.environment;
        let id = self.model.strategy_id.unwrap_or_default();

        Actions::<Cache>::remove_action_cache(env, id)
            .await
            .map_err(|err| Response {
                code: 500,
                message: err,
            })
    }

    /* ===========================
     * START ACTIVE ACTIONS
     * ===========================
     */

    pub async fn start_actions_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STARTING
        if let Err(err) = Actions::<Cache>::set_status_cache(env, LifecycleState::Starting).await {
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // Load active strategies
        let mut strategies_request = Strategies::default().with_env(env);
        strategies_request.model.is_active = Some(true);

        let active_strategies = strategies_request
            .get_strategies()
            .await
            .unwrap_or_default();

        // Load actions from DB
        let mut actions_request = Actions::default().with_env(env);
        actions_request.model.is_active = Some(true);

        let actions = match actions_request.select_actions().await {
            Ok(a) => a
                .into_iter()
                .filter(|act| active_strategies.models.contains_key(&act.strategy_id))
                .collect::<Vec<_>>(),
            Err(err) => {
                let _ = Actions::<Cache>::reset_actions_cache(env).await;
                return Err(err);
            }
        };

        // RUNNING
        if let Err(err) = Actions::<Cache>::set_status_cache(env, LifecycleState::Running).await {
            let _ = Actions::<Cache>::reset_actions_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // Populate cache
        if let Err(err) = Actions::<Cache>::set_actions_cache(env, actions).await {
            let _ = Actions::<Cache>::reset_actions_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        Ok(())
    }

    /* ===========================
     * STOP ACTIVE ACTIONS
     * ===========================
     */

    pub async fn stop_actions_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STOPPING
        if let Err(err) = Actions::<Cache>::set_status_cache(env, LifecycleState::Stopping).await {
            let _ = Actions::<Cache>::reset_actions_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // Remove actions
        if let Err(err) = Actions::<Cache>::remove_actions_cache(env).await {
            let _ = Actions::<Cache>::reset_actions_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // OFF
        if let Err(err) = Actions::<Cache>::set_status_cache(env, LifecycleState::Off).await {
            let _ = Actions::<Cache>::reset_actions_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        Ok(())
    }

    pub async fn reset_actions_core(self) -> Result<(), Response> {
        Actions::<Cache>::reset_actions_cache(self.environment)
            .await
            .map_err(Response::server_error)
    }

    /* ===========================
     * MISC
     * ===========================
     */

    pub fn evaluate_action_core(
        self,
        action: Model,
        pair: &pairs::Model,
        ticker: &Ticker,
        base_asset: &assets::Model,
        quote_asset: &assets::Model,
    ) -> Result<Model, String> {
        self.next_phase::<Logic>().evaluate_action_logic(
            action,
            pair,
            ticker,
            base_asset,
            quote_asset,
        )
    }
}
