use std::marker::PhantomData;

use models::{
    entities::{actions::Model, assets, pairs},
    structs::{ActionRequest, Ticker},
};

use crate::utils::{Core, Response, Types};

#[derive(Debug, Default)]
pub struct Actions<Phase = Types> {
    pub phase: PhantomData<Phase>,
    pub model: ActionRequest,
}

impl<Phase> Actions<Phase> {
    pub fn next_phase<Next>(self) -> Actions<Next> {
        Actions {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl Actions {
    pub fn new(model: ActionRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: ActionRequest {
                ..Default::default()
            },
        }
    }

    pub fn into_model(action: ActionRequest) -> Model {
        Model {
            id: action.id.unwrap_or_default(),
            strategy_id: action.strategy_id.unwrap_or_default(),
            is_active: action.is_active.unwrap_or_default(),
            is_sell: action.is_sell.unwrap_or_default(),
            is_quote_asset: action.is_quote_asset.unwrap_or_default(),
            is_percentage: action.is_percentage.unwrap_or_default(),
            value: action.value.unwrap_or_default(),
            pair_id: action.pair_id.unwrap_or_default(),
        }
    }

    pub async fn insert_action(self) -> Result<Model, Response> {
        self.next_phase().insert_action_core().await
    }

    pub async fn select_action(self) -> Result<Option<Model>, Response> {
        self.next_phase().select_action_core().await
    }

    pub async fn select_actions(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_actions_core().await
    }

    pub async fn update_action(self) -> Result<Model, Response> {
        self.next_phase().update_action_core().await
    }

    pub async fn delete_action(self) -> Result<u64, Response> {
        self.next_phase().delete_action_core().await
    }

    pub fn evaluate_action(
        self,
        action: Model,
        pair: &pairs::Model,
        ticker: &Ticker,
        base_asset: &assets::Model,
        quote_asset: &assets::Model,
    ) -> Result<Model, String> {
        self.next_phase::<Core>().evaluate_action_core(
            action,
            pair,
            ticker,
            base_asset,
            quote_asset,
        )
    }
}
