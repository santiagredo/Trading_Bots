use std::{collections::HashMap, marker::PhantomData};

use chrono::NaiveDateTime;
use models::{entities::strategies::Model, structs::StrategyRequest};

use crate::utils::{Core, Response, Types};

#[derive(Debug, Default)]
pub struct Strategies<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: StrategyRequest,
}

impl<Phase> Strategies<Phase> {
    pub fn next_phase<Next>(self) -> Strategies<Next> {
        Strategies {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl Strategies {
    pub fn new(model: StrategyRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: StrategyRequest {
                ..Default::default()
            },
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
        };

        self.model = strategy_request;
        self
    }

    pub async fn get_active_strategies() -> Option<HashMap<i32, Model>> {
        Strategies::<Core>::get_active_strategies_core().await
    }

    pub async fn get_active_strategy(key: &i32) -> Option<Model> {
        Strategies::<Core>::get_active_strategy_core(key).await
    }

    pub async fn start_active_strategies() -> Result<(), Response> {
        Strategies::<Core>::start_active_strategies_core().await
    }

    pub async fn stop_active_strategies() {
        Strategies::<Core>::stop_active_strategies_core().await
    }

    pub async fn start_strategies_evaluation_loop() {
        Strategies::<Core>::start_strategies_evaluation_loop_core().await
    }

    pub async fn stop_strategies_evaluation_loop() {
        Strategies::<Core>::stop_strategies_evaluation_loop_core().await
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

    pub fn evaluate_cooldown(
        self,
        last_exec: Option<NaiveDateTime>,
        cooldown: Option<i32>,
    ) -> bool {
        self.evaluate_cooldown_core(last_exec, cooldown)
    }
}
