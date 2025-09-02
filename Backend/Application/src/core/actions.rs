use models::{
    entities::{actions::Model, assets, pairs},
    structs::{PairRequest, StrategyRequest, Ticker},
};

use crate::{
    config::get_config,
    handler::{Actions, Pairs, Strategies},
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl Actions<Core> {
    pub async fn insert_action_core(self) -> Result<Model, Response> {
        let Some(strategy) = Strategies::new(StrategyRequest {
            id: self.model.strategy_id,
            ..Default::default()
        })
        .select_strategy()
        .await?
        else {
            return Err(Response::not_found("Strategy".to_string()));
        };

        if Pairs::new(PairRequest {
            id: self.model.pair_id.clone(),
            ..Default::default()
        })
        .select_pair()
        .await?
        .is_none()
        {
            return Err(Response::not_found("Pair".to_string()));
        };

        let action = self
            .next_phase::<Logic>()
            .insert_action_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .insert_action_data(&get_config().await.db)
            .await?;

        match strategy.is_active {
            true => Ok(Actions::<Cache>::set_active_action(action).await),
            false => Ok(action),
        }
    }

    pub async fn select_action_core(self) -> Result<Option<Model>, Response> {
        let cache_action =
            Actions::<Cache>::get_active_action(&self.model.strategy_id.unwrap_or_default()).await;

        if cache_action.is_some() {
            return Ok(cache_action);
        }

        self.next_phase::<Data>()
            .select_action_data(&get_config().await.db)
            .await
    }

    pub async fn select_actions_core(self) -> Result<Vec<Model>, Response> {
        self.next_phase::<Data>()
            .select_actions_data(&get_config().await.db)
            .await
    }

    pub async fn update_action_core(self) -> Result<Model, Response> {
        let Some(strategy) = Strategies::new(StrategyRequest {
            id: self.model.strategy_id,
            ..Default::default()
        })
        .select_strategy()
        .await?
        else {
            return Err(Response::not_found("Strategy".to_string()));
        };

        if Pairs::new(PairRequest {
            id: self.model.pair_id.clone(),
            ..Default::default()
        })
        .select_pair()
        .await?
        .is_none()
        {
            return Err(Response::not_found("Pair".to_string()));
        };

        let action = self
            .next_phase::<Logic>()
            .update_action_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .update_action_data(&get_config().await.db)
            .await?;

        match strategy.is_active {
            true => Ok(Actions::<Cache>::set_active_action(action).await),
            false => Ok(action),
        }
    }

    pub async fn delete_action_core(self) -> Result<u64, Response> {
        let mut action = Actions::into_model(self.model.clone());
        action.is_active = false;

        Actions::<Cache>::set_active_action(action).await;

        self.next_phase::<Logic>()
            .delete_action_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .delete_action_data(&get_config().await.db)
            .await
    }

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
