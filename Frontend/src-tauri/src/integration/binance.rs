use reqwest::{Client, Response};

use crate::static_strings::{BACKEND_URL, BINANCE};

pub async fn get_account_integration() -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{BINANCE}"))
        .send()
        .await
        .map_err(|err| err.to_string())
}
