use std::process::{Command, Stdio};

use crate::{
    models::structs::ConfigurationRequest,
    static_strings::{BACKEND_URL, CONFIGURATIONS, HEALTH_CHECK, SHUTDOWN, ENGINES},
};

use reqwest::Response;
use tauri::{AppHandle, Manager};

pub async fn set_engine_running_integration(app: &AppHandle, run: bool) -> Result<(), String> {
    let running = match reqwest::get(format!("{BACKEND_URL}{HEALTH_CHECK}")).await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    };

    match (run, running) {
        (true, false) => start_engine(app).await,
        (false, true) => stop_engine().await,
        _ => Ok(()), // no-op
    }
}

async fn start_engine(app: &AppHandle) -> Result<(), String> {
    let engine_path = app
        .path()
        .resource_dir()
        .map_err(|err| err.to_string())?
        .join("bin/application");

    Command::new(engine_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| err.to_string())?;

    Ok(())
}

async fn stop_engine() -> Result<(), String> {
    let response = reqwest::Client::new()
        .post(format!("{BACKEND_URL}{ENGINES}{SHUTDOWN}"))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if status.is_success() {
        Ok(())
    } else if status.is_client_error() {
        Err(format!("Client error ({}): {}", status, body))
    } else if status.is_server_error() {
        Err(format!("Server error ({}): {}", status, body))
    } else {
        Err(format!("Unexpected error ({}): {}", status, body))
    }
}

pub async fn select_configuration_integration() -> Result<Response, String> {
    reqwest::Client::new()
        .get(format!("{BACKEND_URL}{CONFIGURATIONS}"))
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn insert_configuration_integration(
    configuration: ConfigurationRequest,
) -> Result<Response, String> {
    reqwest::Client::new()
        .post(format!("{BACKEND_URL}{CONFIGURATIONS}"))
        .json(&configuration)
        .send()
        .await
        .map_err(|err| err.to_string())
}
