use std::marker::PhantomData;

use models::{
    entities::{
        indicators::{self, Model},
        pairs,
    },
    structs::{IndicatorRequest, Ticker},
};

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct Indicators<Phase = Types> {
    pub phase: PhantomData<Phase>,
    pub model: IndicatorRequest,
}

impl<Phase> Indicators<Phase> {
    pub fn next_phase<Next>(self) -> Indicators<Next> {
        Indicators {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl Indicators {
    pub fn new(model: IndicatorRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: IndicatorRequest {
                ..Default::default()
            },
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
        }
    }

    pub async fn insert_indicator(self) -> Result<Model, Response> {
        self.next_phase().insert_indicator_core().await
    }

    pub async fn select_indicator(self) -> Result<Option<Model>, Response> {
        self.next_phase().select_indicator_core().await
    }

    pub async fn select_indicators(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_indicators_core().await
    }

    pub async fn update_indicator(self) -> Result<Model, Response> {
        self.next_phase().update_indicator_core().await
    }

    pub async fn delete_indicator(self) -> Result<u64, Response> {
        self.next_phase().delete_indicator_core().await
    }

    pub fn evalute_active_indicators(
        ticker: &Ticker,
        indicator: &indicators::Model,
        pair: &pairs::Model,
    ) -> Result<bool, String> {
        Self::default()
            .next_phase()
            .evaluate_indicator_core(&indicator, &ticker, &pair)
    }
}
