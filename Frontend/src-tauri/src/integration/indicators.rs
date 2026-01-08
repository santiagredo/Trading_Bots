use reqwest::{Client, Response};

use crate::models::structs::IndicatorRequest;
use crate::static_strings::{BACKEND_URL, INDICATORS};

// db
pub async fn insert_indicator_integration(
    env: String,
    indicator: IndicatorRequest,
) -> Result<Response, String> {
    Client::new()
        .post(format!("{BACKEND_URL}{INDICATORS}/{env}"))
        .json(&indicator)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn select_indicator_integration(
    env: String,
    indicator: Option<IndicatorRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{INDICATORS}/{env}"))
        .query(&indicator)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn select_indicators_integration(
    env: String,
    indicator: Option<IndicatorRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{INDICATORS}/{env}/all"))
        .query(&indicator)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn update_indicator_integration(
    env: String,
    indicator: IndicatorRequest,
) -> Result<Response, String> {
    Client::new()
        .patch(format!("{BACKEND_URL}{INDICATORS}/{env}"))
        .json(&indicator)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn delete_indicator_integration(
    env: String,
    indicator: IndicatorRequest,
) -> Result<Response, String> {
    Client::new()
        .delete(format!("{BACKEND_URL}{INDICATORS}/{env}"))
        .json(&indicator)
        .send()
        .await
        .map_err(|err| err.to_string())
}

// cache
pub async fn get_active_indicator_integration(
    env: String,
    indicator: Option<IndicatorRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{INDICATORS}/{env}/memory"))
        .query(&indicator)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn get_active_indicators_integration(env: String) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{INDICATORS}/{env}/memory/all"))
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn get_subscribed_indicators_integration(env: String) -> Result<Response, String> {
    Client::new()
        .get(format!(
            "{BACKEND_URL}{INDICATORS}/{env}/memory/subscribed_indicators/all"
        ))
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn start_active_indicators_integration(env: String) -> Result<Response, String> {
    Client::new()
        .post(format!("{BACKEND_URL}{INDICATORS}/{env}/memory/start"))
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn stop_active_indicators_integration(env: String) -> Result<Response, String> {
    Client::new()
        .post(format!("{BACKEND_URL}{INDICATORS}/{env}/memory/stop"))
        .send()
        .await
        .map_err(|err| err.to_string())
}
