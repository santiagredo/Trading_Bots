use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use models::{
    entities::{
        actions::{self, Model},
        assets, pairs,
    },
    structs::{ActionRequest, Ticker},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{
    types::Strategies,
    utils::{Core, Response, Types},
};

#[derive(Debug, Default)]
pub struct Actions<Phase = Types> {
    pub phase: PhantomData<Phase>,
    pub model: ActionRequest,
}

static ACTIVE_ACTIONS: Lazy<Arc<RwLock<Option<HashMap<i32, Model>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

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

    pub async fn set_active_actions(actions: Option<Vec<Model>>) -> Option<Vec<Model>> {
        let mut active_actions = ACTIVE_ACTIONS.write().await;

        let Some(actions) = actions else {
            *active_actions = None;
            return None;
        };

        let mut active_actions_map: HashMap<i32, Model> = HashMap::new();

        for action in actions.iter() {
            active_actions_map.insert(action.strategy_id, action.clone());
        }

        *active_actions = Some(active_actions_map);

        Some(actions)
    }

    pub async fn set_active_action(action: Model) -> Model {
        let mut active_actions = ACTIVE_ACTIONS.write().await;

        // early return if active actions is none
        let Some(actions_map) = active_actions.as_mut() else {
            return action;
        };

        if !action.is_active {
            actions_map.remove(&action.id);
        } else {
            actions_map.insert(action.id, action.clone());
        }

        action
    }

    pub async fn get_active_actions() -> Option<HashMap<i32, Model>> {
        let active_actions = ACTIVE_ACTIONS.read().await;

        active_actions.clone()
    }

    pub async fn get_active_action(key: &i32) -> Option<Model> {
        let active_actions = ACTIVE_ACTIONS.read().await;

        let Some(actions_map) = active_actions.as_ref() else {
            return None;
        };

        actions_map.get(key).cloned()
    }

    pub async fn start_active_actions() -> Result<(), Response> {
        let active_strategies = Strategies::get_active_strategies()
            .await
            .unwrap_or_default();

        if Self::get_active_actions()
            .await
            .is_none_or(|active_actions| active_actions.is_empty())
        {
            let mut actions_request = Self::default();
            actions_request.model.is_active = Some(true);

            let active_actions = actions_request
                .select_actions()
                .await?
                .into_iter()
                .filter(|act_act| active_strategies.contains_key(&act_act.strategy_id))
                .collect::<Vec<_>>();

            Self::set_active_actions(Some(active_actions)).await;
        }

        Ok(())
    }

    pub async fn stop_active_actions() {
        Self::set_active_actions(None).await;
    }

    pub fn evaluate_active_actions(
        action: actions::Model,
        pair: &pairs::Model,
        base_asset: &assets::Model,
        quote_asset: &assets::Model,
        ticker: &Ticker,
    ) -> Result<Model, String> {
        Actions::default().evaluate_action(action, &pair, &ticker, &base_asset, &quote_asset)
    }
}

impl<Phase> Actions<Phase> {
    pub fn next_phase<Next>(self) -> Actions<Next> {
        Actions {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl Actions<Types> {
    pub async fn insert_action(self) -> Result<Model, Response> {
        let action = self.next_phase().insert_action_core().await?;

        if Strategies::get_active_strategy(&action.strategy_id)
            .await
            .is_some_and(|strategy| strategy.is_active)
        {
            return Ok(Self::set_active_action(action).await);
        }

        Ok(action)
    }

    pub async fn select_action(self) -> Result<Option<Model>, Response> {
        let memory_action = Self::get_active_action(&self.model.id.unwrap_or_default()).await;

        if memory_action.is_some() {
            return Ok(memory_action);
        }

        self.next_phase().select_action_core().await
    }

    pub async fn select_actions(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_actions_core().await
    }

    pub async fn update_action(self) -> Result<Model, Response> {
        let mut action = self.next_phase().update_action_core().await?;

        if Strategies::get_active_strategy(&action.strategy_id)
            .await
            .is_none_or(|strategy| !strategy.is_active)
        {
            action.is_active = false;
        }

        Ok(Self::set_active_action(action).await)
    }

    pub async fn delete_action(self) -> Result<u64, Response> {
        let mut action = Self::into_model(self.model.clone());
        action.is_active = false;

        Self::set_active_action(action).await;

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
