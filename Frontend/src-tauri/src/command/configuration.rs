use tauri::AppHandle;

use crate::{
    core::{insert_configuration_core, select_configuration_core, set_engine_running_core},
    models::structs::{Configuration, ConfigurationRequest},
};

#[tauri::command]
pub async fn set_engine_running(app: AppHandle, run: bool) -> Result<(), String> {
    set_engine_running_core(&app, run).await
}

#[tauri::command]
pub async fn select_configuration() -> Result<Configuration, String> {
    select_configuration_core().await
}

#[tauri::command]
pub async fn insert_configuration(configuration: ConfigurationRequest) -> Result<Configuration, String> {
    insert_configuration_core(configuration).await
}
