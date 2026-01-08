use reqwest::{Client, Response};

use crate::static_strings::{BACKEND_URL, METRICS};

// db
pub async fn select_metrics_integration(env: String) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{METRICS}/{env}"))
        .send()
        .await
        .map_err(|err| err.to_string())
}

// cache
pub async fn get_active_metric_integration(env: String) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{METRICS}/{env}/memory"))
        .send()
        .await
        .map_err(|err| err.to_string())
}
