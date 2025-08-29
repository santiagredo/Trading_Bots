use models::structs::{StrategyOverview, StrategyRequest};

use crate::{
    config::get_config,
    types::StrategiesOverview,
    utils::{Core, Data, Logic, Response},
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
}
