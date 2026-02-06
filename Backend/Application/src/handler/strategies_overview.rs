use models::structs::{Environments, StrategyOverview, StrategyRequest};

use crate::{handler::DBC, utils::Response};

#[derive(Debug, Default)]
pub struct StrategiesOverview {
    pub model: StrategyRequest,
}

impl StrategiesOverview {
    pub fn new(strategy: StrategyRequest) -> Self {
        Self { model: strategy }
    }

    pub async fn select(
        environment: Environments,
        strategy: StrategyRequest,
    ) -> Result<StrategyOverview, Response> {
        let db = DBC::db(&environment).await?;

        Self::select_strategy_overview(&db, strategy).await
    }
}
