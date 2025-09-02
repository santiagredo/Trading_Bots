use std::marker::PhantomData;

use models::structs::{StrategyOverview, StrategyRequest};

use crate::{
    types::{Actions, Assets, Indicators, OrderStatus, Pairs, Strategies, Tickers},
    utils::{Core, Response, Types},
};

#[derive(Debug, Default)]
pub struct StrategiesOverview<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: StrategyRequest,
}

// static STRATEGIES: Lazy<Arc<RwLock<HashMap<i32, StrategyOverview>>>> =
//     Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

// static ACTIVE_STRATEGIES_HANDLE: Lazy<Arc<RwLock<Option<AbortHandle>>>> =
//     Lazy::new(|| Arc::new(RwLock::new(None)));

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

    pub async fn get_memory_strategy_overview(
        strategy_id: &i32,
        symbol: String,
    ) -> Option<StrategyOverview> {
        let Some(strategy) = Strategies::get_active_strategy(&strategy_id).await else {
            return None;
        };

        let Some(indicator) = Indicators::get_active_indicator(&strategy_id).await else {
            return None;
        };

        let Some(action) = Actions::get_active_action(&strategy_id).await else {
            return None;
        };

        let Some(pair) = Pairs::get_active_pair(&action.pair_id).await else {
            return None;
        };

        let Some(base_asset) = Assets::get_active_asset(&pair.base_asset_id).await else {
            return None;
        };

        let Some(quote_asset) = Assets::get_active_asset(&pair.quote_asset_id).await else {
            return None;
        };

        let Some(ticker) = Tickers::get_ticker(symbol).await else {
            return None;
        };

        let Some(order_status) = OrderStatus::get_active_status().await else {
            return None;
        };

        let strategy_overview = StrategyOverview {
            strategy,
            indicator,
            action,
            pair,
            base_asset,
            quote_asset,
            ticker,
            order_status,
        };

        Some(strategy_overview)
    }
}

impl<Phase> StrategiesOverview<Phase> {
    pub fn next_phase<Next>(self) -> StrategiesOverview<Next> {
        StrategiesOverview {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl StrategiesOverview<Types> {
    pub async fn select_strategies_overview(
        strategy: StrategyRequest,
    ) -> Result<Vec<StrategyOverview>, Response> {
        StrategiesOverview::<Core>::select_strategies_overview_core(strategy).await
    }
}
