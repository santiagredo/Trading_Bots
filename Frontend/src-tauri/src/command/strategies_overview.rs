use crate::core::select_strategies_overview_core;
use crate::models::structs::StrategyRequest;

#[tauri::command]
pub async fn select_strategies_overview(
    env: String,
    query: StrategyRequest,
) -> Result<Vec<crate::models::structs::StrategyOverview>, String> {
    select_strategies_overview_core(env, query).await
}
