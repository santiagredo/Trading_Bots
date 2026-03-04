use crate::{
    core::{
        get_integration_setting_core, get_integrations_settings_core,
        select_integration_setting_core, select_integrations_settings_core,
        update_integration_setting_core,
    },
    models::{entities::integration_settings::Model, structs::IntegrationSettingRequest},
};

/* ==========================================
 * DATABASE
 * ========================================== */

#[tauri::command]
pub async fn select_integration_setting(
    env: String,
    integration_setting: Option<IntegrationSettingRequest>,
) -> Result<Model, String> {
    select_integration_setting_core(env, integration_setting).await
}

#[tauri::command]
pub async fn select_integrations_settings(
    env: String,
    integration_setting: Option<IntegrationSettingRequest>,
) -> Result<Vec<Model>, String> {
    select_integrations_settings_core(env, integration_setting).await
}

#[tauri::command]
pub async fn update_integration_setting(
    env: String,
    integration_setting: IntegrationSettingRequest,
) -> Result<Model, String> {
    update_integration_setting_core(env, integration_setting).await
}

/* ==========================================
 * CACHE (MEMORY)
 * ========================================== */

#[tauri::command]
pub async fn get_integration_setting(
    env: String,
    integration_setting: Option<IntegrationSettingRequest>,
) -> Result<Model, String> {
    get_integration_setting_core(env, integration_setting).await
}

#[tauri::command]
pub async fn get_integrations_settings(env: String) -> Result<Vec<Model>, String> {
    get_integrations_settings_core(env).await
}
