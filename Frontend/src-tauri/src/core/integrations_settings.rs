use std::collections::HashMap;

use crate::{
    integration::{
        get_integration_setting_integration, get_integrations_settings_integration,
        select_integration_setting_integration, select_integrations_settings_integration,
        update_integration_setting_integration,
    },
    models::{entities::integration_settings::Model, structs::IntegrationSettingRequest},
    utils::handle_response,
};

/* ==========================================
 * DATABASE
 * ========================================== */

pub async fn select_integration_setting_core(
    env: String,
    integration_setting: Option<IntegrationSettingRequest>,
) -> Result<Model, String> {
    let response = select_integration_setting_integration(env, integration_setting).await?;

    handle_response::<Model>(response).await
}

pub async fn select_integrations_settings_core(
    env: String,
    integration_setting: Option<IntegrationSettingRequest>,
) -> Result<Vec<Model>, String> {
    let response = select_integrations_settings_integration(env, integration_setting).await?;

    handle_response::<Vec<Model>>(response).await
}

pub async fn update_integration_setting_core(
    env: String,
    integration_setting: IntegrationSettingRequest,
) -> Result<Model, String> {
    let response = update_integration_setting_integration(env, integration_setting).await?;

    handle_response::<Model>(response).await
}

/* ==========================================
 * CACHE (MEMORY)
 * ========================================== */

pub async fn get_integration_setting_core(
    env: String,
    integration_setting: Option<IntegrationSettingRequest>,
) -> Result<Model, String> {
    let response = get_integration_setting_integration(env, integration_setting).await?;

    let result = handle_response::<Option<Model>>(response)
        .await?
        .unwrap_or_default();

    Ok(result)
}

pub async fn get_integrations_settings_core(env: String) -> Result<Vec<Model>, String> {
    let response = get_integrations_settings_integration(env).await?;

    let models = handle_response::<Option<HashMap<i32, Model>>>(response)
        .await?
        .unwrap_or_default()
        .into_iter()
        .map(|(_, val)| val)
        .collect();

    Ok(models)
}
