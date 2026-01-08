use std::collections::{HashMap, HashSet};

use crate::{
    integration::{
        delete_indicator_integration, get_active_indicator_integration,
        get_active_indicators_integration, get_subscribed_indicators_integration,
        insert_indicator_integration, select_indicator_integration, select_indicators_integration,
        start_active_indicators_integration, stop_active_indicators_integration,
        update_indicator_integration,
    },
    models::{entities::indicators::Model, structs::IndicatorRequest},
    utils::handle_response,
};

// db
pub async fn insert_indicator_core(
    env: String,
    indicator: IndicatorRequest,
) -> Result<Model, String> {
    let response = insert_indicator_integration(env, indicator).await?;
    handle_response::<Model>(response).await
}

pub async fn select_indicator_core(
    env: String,
    indicator: Option<IndicatorRequest>,
) -> Result<Model, String> {
    let response = select_indicator_integration(env, indicator).await?;
    handle_response::<Model>(response).await
}

pub async fn select_indicators_core(
    env: String,
    indicator: Option<IndicatorRequest>,
) -> Result<Vec<Model>, String> {
    let response = select_indicators_integration(env, indicator).await?;
    handle_response::<Vec<Model>>(response).await
}

pub async fn update_indicator_core(
    env: String,
    indicator: IndicatorRequest,
) -> Result<Model, String> {
    let response = update_indicator_integration(env, indicator).await?;
    handle_response::<Model>(response).await
}

pub async fn delete_indicator_core(
    env: String,
    indicator: IndicatorRequest,
) -> Result<u64, String> {
    let response = delete_indicator_integration(env, indicator).await?;
    handle_response::<u64>(response).await
}

// cache
pub async fn get_active_indicator_core(
    env: String,
    indicator: Option<IndicatorRequest>,
) -> Result<Model, String> {
    let response = get_active_indicator_integration(env, indicator).await?;

    let result = handle_response::<Option<Model>>(response)
        .await?
        .unwrap_or_default();

    Ok(result)
}

pub async fn get_active_indicators_core(env: String) -> Result<Vec<Model>, String> {
    let response = get_active_indicators_integration(env).await?;

    let models = handle_response::<Option<HashMap<i32, Model>>>(response)
        .await?
        .unwrap_or_default()
        .into_iter()
        .map(|(_, val)| val)
        .collect();

    Ok(models)
}

pub async fn get_subscribed_indicators_core(
    env: String,
) -> Result<Option<HashMap<String, HashSet<i32>>>, String> {
    let response = get_subscribed_indicators_integration(env).await?;
    handle_response::<Option<HashMap<String, HashSet<i32>>>>(response).await
}

pub async fn start_active_indicators_core(env: String) -> Result<(), String> {
    let response = start_active_indicators_integration(env).await?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to start active indicators: {}",
            response.status()
        ))
    }
}

pub async fn stop_active_indicators_core(env: String) -> Result<(), String> {
    let response = stop_active_indicators_integration(env).await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to stop active indicators: {}",
            response.status()
        ))
    }
}
