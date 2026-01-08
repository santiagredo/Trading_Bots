use reqwest::{Error, Response};

use crate::static_strings::{BACKEND_URL, HEALTH_CHECK};

pub async fn select_health_check_integration() -> Result<Response, Error> {
    reqwest::get(format!("{BACKEND_URL}{HEALTH_CHECK}")).await
}
