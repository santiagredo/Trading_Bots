use crate::{
    core::{
        delete_action_core, get_active_action_core, get_active_actions_core, insert_action_core,
        select_action_core, select_actions_core, start_active_actions_core,
        stop_active_actions_core, update_action_core,
    },
    models::{
        entities::actions::Model,
        structs::{ActionRequest, CacheActions},
    },
};

// db
#[tauri::command]
pub async fn insert_action(env: String, action: ActionRequest) -> Result<Model, String> {
    insert_action_core(env, action).await
}

#[tauri::command]
pub async fn select_action(env: String, action: Option<ActionRequest>) -> Result<Model, String> {
    select_action_core(env, action).await
}

#[tauri::command]
pub async fn select_actions(
    env: String,
    action: Option<ActionRequest>,
) -> Result<Vec<Model>, String> {
    select_actions_core(env, action).await
}

#[tauri::command]
pub async fn update_action(env: String, action: ActionRequest) -> Result<Model, String> {
    update_action_core(env, action).await
}

#[tauri::command]
pub async fn delete_action(env: String, action: ActionRequest) -> Result<u64, String> {
    delete_action_core(env, action).await
}

// cache
#[tauri::command]
pub async fn get_active_action(
    env: String,
    action: Option<ActionRequest>,
) -> Result<Model, String> {
    get_active_action_core(env, action).await
}

#[tauri::command]
pub async fn get_active_actions(env: String) -> Result<Option<CacheActions>, String> {
    get_active_actions_core(env).await
}

#[tauri::command]
pub async fn start_active_actions(env: String) -> Result<(), String> {
    start_active_actions_core(env).await
}

#[tauri::command]
pub async fn stop_active_actions(env: String) -> Result<(), String> {
    stop_active_actions_core(env).await
}
