use crate::{
    core::{select_error_logs_core, select_integration_logs_core},
    models::entities::{error_log, integration_log},
};

#[tauri::command]
pub async fn select_error_logs(env: String) -> Result<Vec<error_log::Model>, String> {
    select_error_logs_core(env).await
}

#[tauri::command]
pub async fn select_integration_logs(env: String) -> Result<Vec<integration_log::Model>, String> {
    select_integration_logs_core(env).await
}
