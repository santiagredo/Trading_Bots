use std::{collections::HashMap, marker::PhantomData};

use chrono::NaiveDateTime;
use models::{
    entities::strategies::Model,
    structs::{CacheStrategy, Environments, StrategyRequest},
};

use crate::utils::{Response, Types};

#[derive(Debug, Default, Clone)]
pub struct Strategies<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
    pub model: StrategyRequest,
}

impl<Phase> Strategies<Phase> {
    pub fn next_phase<Next>(self) -> Strategies<Next> {
        Strategies {
            phase: PhantomData::<Next>,
            environment: self.environment,
            model: self.model,
        }
    }
}

impl Strategies {
    pub fn new(model: StrategyRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model: StrategyRequest {
                ..Default::default()
            },
        }
    }

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: self.phase,
            environment: environment,
            model: self.model,
        }
    }

    pub fn into_model(strategy: StrategyRequest) -> Model {
        Model {
            id: strategy.id.unwrap_or_default(),
            name: strategy.name.unwrap_or_default(),
            is_active: strategy.is_active.unwrap_or_default(),
            can_trade: strategy.can_trade.unwrap_or_default(),
            description: Some(strategy.description.unwrap_or_default()),
            last_execution: Some(strategy.last_execution.unwrap_or_default()),
            cooldown: Some(strategy.cooldown.unwrap_or_default()),
            error_cooldown: Some(strategy.error_cooldown.unwrap_or_default()),
            error_last_date: Some(strategy.error_last_date.unwrap_or_default()),
            last_update: Some(strategy.last_update.unwrap_or_default()),
        }
    }

    pub fn into_request(mut self, model: Model) -> Self {
        let strategy_request = StrategyRequest {
            id: Some(model.id),
            name: Some(model.name),
            is_active: Some(model.is_active),
            can_trade: Some(model.can_trade),
            description: model.description,
            last_execution: model.last_execution,
            cooldown: model.cooldown,
            error_cooldown: model.error_cooldown,
            error_last_date: model.error_last_date,
            last_update: model.last_update,
        };

        self.model = strategy_request;
        self
    }

    pub async fn insert_strategy(self) -> Result<Model, Response> {
        self.insert_strategy_core().await
    }

    pub async fn select_strategy(self) -> Result<Option<Model>, Response> {
        self.select_strategy_core().await
    }

    pub async fn select_strategies(self) -> Result<Vec<Model>, Response> {
        self.select_strategies_core().await
    }

    pub async fn update_strategy(self) -> Result<Model, Response> {
        self.update_strategy_core().await
    }

    pub async fn delete_strategy(self) -> Result<u64, Response> {
        self.delete_strategy_core().await
    }

    // cache
    pub async fn get_active_strategies(self) -> Option<HashMap<i32, CacheStrategy>> {
        self.get_active_strategies_core().await
    }

    pub async fn get_active_strategy(self) -> Option<CacheStrategy> {
        self.get_active_strategy_core().await
    }

    pub async fn set_active_strategy(self, is_remove: bool, error: Option<String>) -> Model {
        self.set_active_strategy_core(is_remove, error).await
    }

    pub async fn set_active_strategy_posting(self, is_posting: bool) -> Result<(), String> {
        self.set_active_strategy_posting_core(is_posting).await
    }

    pub async fn start_active_strategies(self) -> Result<(), Response> {
        self.start_active_strategies_core().await
    }

    pub async fn stop_active_strategies(self) {
        self.stop_active_strategies_core().await
    }

    // misc
    pub fn evaluate_cooldown(
        self,
        last_exec: Option<NaiveDateTime>,
        cooldown: Option<i32>,
    ) -> bool {
        self.evaluate_cooldown_core(last_exec, cooldown)
    }
}
