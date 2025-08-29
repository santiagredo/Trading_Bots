use models::{
    entities::{actions::Model, assets, pairs},
    structs::{PairRequest, StrategyRequest, Ticker},
};

use crate::{
    config::get_config,
    types::{Actions, Pairs, Strategies},
    utils::{handle_user_err, Core, Data, Logic, Response},
};

impl Actions<Core> {
    pub async fn insert_action_core(self) -> Result<Model, Response> {
        Strategies::new(StrategyRequest {
            id: self.model.strategy_id,
            ..Default::default()
        })
        .select_strategy()
        .await?;

        Pairs::new(PairRequest {
            id: self.model.pair_id.clone(),
            ..Default::default()
        })
        .select_pair()
        .await?;

        let logic_type = self
            .next_phase::<Logic>()
            .insert_action_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .insert_action_data(&get_config().await.db)
            .await
    }

    pub async fn select_action_core(self) -> Result<Option<Model>, Response> {
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
        Strategies::new(StrategyRequest {
            id: self.model.strategy_id,
            ..Default::default()
        })
        .select_strategy()
        .await?;

        Pairs::new(PairRequest {
            id: self.model.pair_id.clone(),
            ..Default::default()
        })
        .select_pair()
        .await?;

        let logic_type = self
            .next_phase::<Logic>()
            .update_action_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .update_action_data(&get_config().await.db)
            .await
    }

    pub async fn delete_action_core(self) -> Result<u64, Response> {
        let logic_type = self
            .next_phase::<Logic>()
            .delete_action_logic()
            .map_err(handle_user_err)?;

        logic_type
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

    // pub async fn select_actions_by_strategy_id(id: i32) -> Outcome<Vec<Model>, String, String> {
    //     Actions::<Data>::select_actions_by_strategy_id(&get_config().await.db, id).await
    // }

    // pub async fn select_actions_by_strategies_id(
    //     strategies_ids: Vec<i32>,
    // ) -> Outcome<Vec<Model>, String, String> {
    //     Actions::<Data>::select_actions_by_strategies_id(&get_config().await.db, strategies_ids)
    //         .await
    // }
}
