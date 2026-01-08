use std::collections::HashMap;

use crate::{
    integration::{
        delete_strategy_integration, get_active_strategies_integration,
        get_active_strategy_integration, insert_strategy_integration,
        select_strategies_integration, select_strategy_integration,
        start_active_strategies_integration, stop_active_strategies_integration,
        update_strategy_integration,
    },
    models::{
        entities::strategies::Model,
        structs::{CacheStrategy, StrategyRequest},
    },
    utils::handle_response,
};

// db
pub async fn insert_strategy_core(env: String, strategy: StrategyRequest) -> Result<Model, String> {
    let response = insert_strategy_integration(env, strategy).await?;
    handle_response::<Model>(response).await
}

pub async fn select_strategy_core(
    env: String,
    strategy: Option<StrategyRequest>,
) -> Result<Model, String> {
    let response = select_strategy_integration(env, strategy).await?;
    handle_response::<Model>(response).await
}

pub async fn select_strategies_core(
    env: String,
    strategy: Option<StrategyRequest>,
) -> Result<Vec<Model>, String> {
    let response = select_strategies_integration(env, strategy).await?;

    handle_response::<Vec<Model>>(response).await
}

pub async fn update_strategy_core(env: String, strategy: StrategyRequest) -> Result<Model, String> {
    let response = update_strategy_integration(env, strategy).await?;
    handle_response::<Model>(response).await
}

pub async fn delete_strategy_core(env: String, strategy: StrategyRequest) -> Result<u64, String> {
    let response = delete_strategy_integration(env, strategy).await?;
    handle_response::<u64>(response).await
}

// cache
pub async fn get_active_strategy_core(
    env: String,
    strategy: Option<StrategyRequest>,
) -> Result<Model, String> {
    let response = get_active_strategy_integration(env, strategy).await?;
    handle_response::<Model>(response).await
}

pub async fn get_active_strategies_core(
    env: String,
    strategy: Option<StrategyRequest>,
) -> Result<Option<HashMap<i32, CacheStrategy>>, String> {
    let response = get_active_strategies_integration(env, strategy).await?;

    let models = handle_response::<Option<HashMap<i32, CacheStrategy>>>(response).await?;

    Ok(models)
}

pub async fn start_active_strategies_core(env: String) -> Result<Vec<Model>, String> {
    let response = start_active_strategies_integration(env).await?;
    handle_response::<Vec<Model>>(response).await
}

pub async fn stop_active_strategies_core(env: String) -> Result<(), String> {
    let response = stop_active_strategies_integration(env).await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to stop active strategies: {}",
            response.status()
        ))
    }
}
