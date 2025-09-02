use models::{
    entities::orders,
    structs::{AssetRequest, LedgerRequest, StrategyOverview, StrategyRequest},
};

use crate::{
    config::get_config,
    handler::{Orders, StrategiesOverview},
    utils::{Cache, Core, Data, Logic, Response},
};

impl StrategiesOverview<Core> {
    pub async fn select_strategies_overview_core(
        strategy: StrategyRequest,
    ) -> Result<Vec<StrategyOverview>, Response> {
        let results = StrategiesOverview::<Data>::select_strategies_overview_data(
            &get_config().await.db,
            strategy,
        )
        .await?;

        Ok(StrategiesOverview::<Logic>::select_strategies_overview_logic(results))
    }

    pub async fn get_active_strategy_overview_core(
        strategy_id: &i32,
        symbol: String,
    ) -> Option<StrategyOverview> {
        StrategiesOverview::<Cache>::get_active_strategy_overview_cache(strategy_id, symbol).await
    }

    pub fn evaluate_strategy_overview_core(
        strategy_overview: &StrategyOverview,
    ) -> Result<Orders, String> {
        StrategiesOverview::<Logic>::evaluate_strategy_overview_logic(strategy_overview)
    }

    pub fn build_asset_ledger_request_core(
        strategy_overview: &StrategyOverview,
        order: &orders::Model,
        is_base: bool,
    ) -> (AssetRequest, LedgerRequest) {
        StrategiesOverview::<Logic>::build_asset_ledger_request_logic(
            strategy_overview,
            order,
            is_base,
        )
    }
}
