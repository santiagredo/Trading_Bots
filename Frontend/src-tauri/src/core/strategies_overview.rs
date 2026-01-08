use crate::{
    integration::select_strategies_overview_integration,
    models::structs::{StrategyOverview, StrategyRequest},
    utils::handle_response,
};

pub async fn select_strategies_overview_core(
    env: String,
    query: StrategyRequest,
) -> Result<Vec<StrategyOverview>, String> {
    let response = select_strategies_overview_integration(env, query).await?;

    handle_response::<Vec<StrategyOverview>>(response).await
}
