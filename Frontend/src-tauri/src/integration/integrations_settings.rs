use reqwest::{Client, Response};

use crate::models::structs::IntegrationSettingRequest;
use crate::static_strings::{BACKEND_URL, INTEGRATIONS_SETTINGS};

/* ==========================================
 * DATABASE
 * ========================================== */

pub async fn select_integration_setting_integration(
    env: String,
    integration_setting: Option<IntegrationSettingRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{INTEGRATIONS_SETTINGS}/{env}"))
        .query(&integration_setting)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn select_integrations_settings_integration(
    env: String,
    integration_setting: Option<IntegrationSettingRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{INTEGRATIONS_SETTINGS}/{env}/all"))
        .query(&integration_setting)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn update_integration_setting_integration(
    env: String,
    integration_setting: IntegrationSettingRequest,
) -> Result<Response, String> {
    Client::new()
        .patch(format!("{BACKEND_URL}{INTEGRATIONS_SETTINGS}/{env}"))
        .json(&integration_setting)
        .send()
        .await
        .map_err(|err| err.to_string())
}

/* ==========================================
 * CACHE (MEMORY)
 * ========================================== */

pub async fn get_integration_setting_integration(
    env: String,
    integration_setting: Option<IntegrationSettingRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{INTEGRATIONS_SETTINGS}/{env}/memory"))
        .query(&integration_setting)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn get_integrations_settings_integration(env: String) -> Result<Response, String> {
    Client::new()
        .get(format!(
            "{BACKEND_URL}{INTEGRATIONS_SETTINGS}/{env}/memory/all"
        ))
        .send()
        .await
        .map_err(|err| err.to_string())
}
