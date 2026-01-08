use tauri::AppHandle;

use crate::{
    integration::{
        insert_configuration_integration, select_configuration_integration,
        set_engine_running_integration,
    },
    models::structs::{Configuration, ConfigurationRequest},
    utils::handle_response,
};

pub async fn set_engine_running_core(app: &AppHandle, run: bool) -> Result<(), String> {
    set_engine_running_integration(app, run).await
}

pub async fn select_configuration_core() -> Result<Configuration, String> {
    let response = select_configuration_integration().await?;

    handle_response::<Configuration>(response).await
}

pub async fn insert_configuration_core(
    configuration: ConfigurationRequest,
) -> Result<Configuration, String> {
    let response = insert_configuration_integration(configuration).await?;

    handle_response::<Configuration>(response).await
}
