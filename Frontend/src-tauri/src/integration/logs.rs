use crate::static_strings::{BACKEND_URL, ERROR_LOG, INTEGRATION_LOG};
use reqwest::{Client, Response};

pub async fn select_error_logs_integration(env: String) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{ERROR_LOG}/{env}/all"))
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn select_integration_logs_integration(env: String) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{INTEGRATION_LOG}/{env}/all"))
        .send()
        .await
        .map_err(|err| err.to_string())
}
