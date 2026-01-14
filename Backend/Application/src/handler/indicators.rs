use std::{collections::HashMap, marker::PhantomData};

use models::{
    entities::{
        indicators::{self, Model},
        pairs,
    },
    enums::LifecycleState,
    structs::{Environments, IndicatorRequest, QueryOptions, Ticker},
};

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct Indicators<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
    pub model: IndicatorRequest,
}

impl<Phase> Indicators<Phase> {
    pub fn next_phase<Next>(self) -> Indicators<Next> {
        Indicators {
            phase: PhantomData::<Next>,
            environment: self.environment,
            model: self.model,
        }
    }
}

impl Indicators {
    pub fn new(model: IndicatorRequest) -> Self {
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
            model: IndicatorRequest {
                ..Default::default()
            },
        }
    }

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: self.phase,
            environment,
            model: self.model,
        }
    }

    pub fn into_model(indicator: IndicatorRequest) -> Model {
        Model {
            id: indicator.id.unwrap_or_default(),
            strategy_id: indicator.strategy_id.unwrap_or_default(),
            is_active: indicator.is_active.unwrap_or_default(),
            symbol: indicator.symbol.unwrap_or_default(),
            nick: indicator.nick.unwrap_or_default(),
            direction: indicator.direction.unwrap_or_default(),
            is_percentage: indicator.is_percentage.unwrap_or_default(),
            value: indicator.value.unwrap_or_default(),
            last_update: indicator.last_update,
        }
    }

    // db
    pub async fn insert_indicator(self) -> Result<Model, Response> {
        self.next_phase().insert_indicator_core().await
    }

    pub async fn select_indicator(self) -> Result<Option<Model>, Response> {
        self.next_phase().select_indicator_core().await
    }

    pub async fn select_indicators(
        self,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.next_phase().select_indicators_core(query).await
    }

    pub async fn update_indicator(self) -> Result<Model, Response> {
        self.next_phase().update_indicator_core().await
    }

    pub async fn delete_indicator(self) -> Result<u64, Response> {
        self.next_phase().delete_indicator_core().await
    }

    // cache
    pub async fn get_indicators(self) -> Option<HashMap<i32, Model>> {
        self.next_phase().get_indicators_core().await
    }

    pub async fn get_indicator(self) -> Option<Model> {
        self.next_phase().get_indicator_core().await
    }

    pub async fn get_indicators_state(self) -> LifecycleState {
        self.next_phase().get_indicators_state_core().await
    }

    pub async fn start_indicators(self) -> Result<(), Response> {
        self.next_phase().start_indicators_core().await
    }

    pub async fn stop_indicators(self) -> Result<(), Response> {
        self.next_phase().stop_indicators_core().await
    }

    pub async fn reset_indicators(self) -> Result<(), Response> {
        self.next_phase().reset_indicators_core().await
    }

    // misc
    pub fn evalute_indicators(
        ticker: &Ticker,
        indicator: &indicators::Model,
        pair: &pairs::Model,
    ) -> Result<bool, String> {
        Self::default()
            .next_phase()
            .evaluate_indicator_core(&indicator, &ticker, &pair)
    }
}
