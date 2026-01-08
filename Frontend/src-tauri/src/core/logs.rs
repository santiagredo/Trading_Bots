use crate::{
    integration::{select_error_logs_integration, select_integration_logs_integration},
    models::entities::{error_log, integration_log},
    utils::handle_response,
};

pub async fn select_error_logs_core(env: String) -> Result<Vec<error_log::Model>, String> {
    let response = select_error_logs_integration(env).await?;
    handle_response::<Vec<error_log::Model>>(response).await
}

pub async fn select_integration_logs_core(
    env: String,
) -> Result<Vec<integration_log::Model>, String> {
    let response = select_integration_logs_integration(env).await?;
    handle_response::<Vec<integration_log::Model>>(response).await
}
