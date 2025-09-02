use std::{collections::HashMap, sync::Arc};

use models::{
    entities::{
        actions::{self, Model},
        assets, pairs,
    },
    structs::Ticker,
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{
    handler::{Actions, Strategies},
    utils::{Cache, Response},
};

static ACTIVE_ACTIONS: Lazy<Arc<RwLock<Option<HashMap<i32, Model>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Actions<Cache> {
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
            let mut actions_request = Actions::default();
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
