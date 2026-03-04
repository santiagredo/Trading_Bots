use crate::{
    core::{
        delete_strategy_core, get_active_strategies_core, get_active_strategy_core,
        insert_strategy_core, select_strategies_core, select_strategy_core,
        start_active_strategies_core, stop_active_strategies_core, update_strategy_core,
    },
    models::{
        entities::strategies::Model,
        structs::{CacheStrategies, StrategyRequest},
    },
};

// db
#[tauri::command]
pub async fn insert_strategy(env: String, strategy: StrategyRequest) -> Result<Model, String> {
    insert_strategy_core(env, strategy).await
}

#[tauri::command]
pub async fn select_strategy(
    env: String,
    strategy: Option<StrategyRequest>,
) -> Result<Model, String> {
    select_strategy_core(env, strategy).await
}

#[tauri::command]
pub async fn select_strategies(
    env: String,
    strategy: Option<StrategyRequest>,
) -> Result<Vec<Model>, String> {
    select_strategies_core(env, strategy).await
}

#[tauri::command]
pub async fn update_strategy(env: String, strategy: StrategyRequest) -> Result<Model, String> {
    update_strategy_core(env, strategy).await
}

#[tauri::command]
pub async fn delete_strategy(env: String, strategy: StrategyRequest) -> Result<u64, String> {
    delete_strategy_core(env, strategy).await
}

// cache
#[tauri::command]
pub async fn get_active_strategy(
    env: String,
    strategy: Option<StrategyRequest>,
) -> Result<Model, String> {
    get_active_strategy_core(env, strategy).await
}

#[tauri::command]
pub async fn get_active_strategies(
    env: String,
    strategy: Option<StrategyRequest>,
) -> Result<Option<CacheStrategies>, String> {
    get_active_strategies_core(env, strategy).await
}

#[tauri::command]
pub async fn start_active_strategies(env: String) -> Result<Vec<Model>, String> {
    start_active_strategies_core(env).await
}

#[tauri::command]
pub async fn stop_active_strategies(env: String) -> Result<(), String> {
    stop_active_strategies_core(env).await
}
