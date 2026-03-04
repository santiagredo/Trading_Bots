use std::collections::{HashMap, HashSet};

use crate::{
    core::{
        delete_indicator_core, get_active_indicator_core, get_active_indicators_core,
        get_subscribed_indicators_core, insert_indicator_core, select_indicator_core,
        select_indicators_core, start_active_indicators_core, stop_active_indicators_core,
        update_indicator_core,
    },
    models::{
        entities::indicators::Model,
        structs::{CacheIndicators, IndicatorRequest},
    },
};

// db
#[tauri::command]
pub async fn insert_indicator(env: String, indicator: IndicatorRequest) -> Result<Model, String> {
    insert_indicator_core(env, indicator).await
}

#[tauri::command]
pub async fn select_indicator(
    env: String,
    indicator: Option<IndicatorRequest>,
) -> Result<Model, String> {
    select_indicator_core(env, indicator).await
}

#[tauri::command]
pub async fn select_indicators(
    env: String,
    indicator: Option<IndicatorRequest>,
) -> Result<Vec<Model>, String> {
    select_indicators_core(env, indicator).await
}

#[tauri::command]
pub async fn update_indicator(env: String, indicator: IndicatorRequest) -> Result<Model, String> {
    update_indicator_core(env, indicator).await
}

#[tauri::command]
pub async fn delete_indicator(env: String, indicator: IndicatorRequest) -> Result<u64, String> {
    delete_indicator_core(env, indicator).await
}

// cache
#[tauri::command]
pub async fn get_active_indicator(
    env: String,
    indicator: Option<IndicatorRequest>,
) -> Result<Model, String> {
    get_active_indicator_core(env, indicator).await
}

#[tauri::command]
pub async fn get_active_indicators(env: String) -> Result<Option<CacheIndicators>, String> {
    get_active_indicators_core(env).await
}

#[tauri::command]
pub async fn get_subscribed_indicators(
    env: String,
) -> Result<Option<HashMap<String, HashSet<i32>>>, String> {
    get_subscribed_indicators_core(env).await
}

#[tauri::command]
pub async fn start_active_indicators(env: String) -> Result<(), String> {
    start_active_indicators_core(env).await
}

#[tauri::command]
pub async fn stop_active_indicators(env: String) -> Result<(), String> {
    stop_active_indicators_core(env).await
}
