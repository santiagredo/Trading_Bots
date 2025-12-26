use std::collections::HashMap;

use models::{
    entities::{actions::Model, assets, pairs},
    structs::Ticker,
};

use crate::{
    handler::{Actions, Strategies, DBC},
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl Actions<Core> {
    // db
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

    // cache
    pub async fn get_active_actions_core(self) -> Option<HashMap<i32, Model>> {
        let env = self.environment;
        Actions::<Cache>::get_active_actions_cache(&env).await
    }

    pub async fn get_active_action_core(self) -> Option<Model> {
        let env = self.environment;

        let id = if self.model.id.is_some() {
            self.model.id.unwrap_or_default()
        } else {
            self.model.strategy_id.unwrap_or_default()
        };

        Actions::<Cache>::get_active_action_cache(&env, &id).await
    }

    pub async fn start_active_actions_core(self) -> Result<(), Response> {
        if !Actions::<Cache>::get_active_actions_status_cache(&self.environment).await {
            let env = self.environment;

            let mut strategies_request = Strategies::default().with_env(env);
            strategies_request.model.is_active = Some(true);

            let active_strategies = strategies_request
                .get_active_strategies()
                .await
                .unwrap_or_default();

            let mut actions_request = Actions::default().with_env(env);
            actions_request.model.is_active = Some(true);

            let active_actions = actions_request
                .select_actions()
                .await?
                .into_iter()
                .filter(|act_act| active_strategies.contains_key(&act_act.strategy_id))
                .collect::<Vec<_>>();

            Actions::<Cache>::set_active_actions_cache(&env, active_actions).await;
        }

        Ok(())
    }

    pub async fn stop_active_actions_core(self) {
        let env = self.environment;

        Actions::<Cache>::stop_active_actions_cache(&env).await
    }

    // misc
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
