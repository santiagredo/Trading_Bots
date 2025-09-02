use std::marker::PhantomData;

use models::{
    entities::orders,
    structs::{AssetRequest, LedgerRequest, StrategyOverview, StrategyRequest},
};

use crate::{
    handler::Orders,
    utils::{Core, Response, Types},
};

#[derive(Debug, Default)]
pub struct StrategiesOverview<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: StrategyRequest,
}

impl<Phase> StrategiesOverview<Phase> {
    pub fn next_phase<Next>(self) -> StrategiesOverview<Next> {
        StrategiesOverview {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl StrategiesOverview {
    pub fn new(strategy: StrategyRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: strategy,
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

    pub async fn get_active_strategy_overview(
        strategy_id: &i32,
        symbol: String,
    ) -> Option<StrategyOverview> {
        StrategiesOverview::<Core>::get_active_strategy_overview_core(strategy_id, symbol).await
    }

    pub async fn select_strategies_overview(
        strategy: StrategyRequest,
    ) -> Result<Vec<StrategyOverview>, Response> {
        StrategiesOverview::<Core>::select_strategies_overview_core(strategy).await
    }

    pub fn evaluate_strategy_overview(
        strategy_overview: &StrategyOverview,
    ) -> Result<Orders, String> {
        StrategiesOverview::<Core>::evaluate_strategy_overview_core(strategy_overview)
    }

    pub fn build_asset_ledger_request(
        strategy_overview: &StrategyOverview,
        order: &orders::Model,
        is_base: bool,
    ) -> (AssetRequest, LedgerRequest) {
        StrategiesOverview::<Core>::build_asset_ledger_request_core(
            strategy_overview,
            order,
            is_base,
        )
    }
}
