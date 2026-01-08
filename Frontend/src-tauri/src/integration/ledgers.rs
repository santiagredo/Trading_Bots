use reqwest::{Client, Response};

use crate::{
    models::structs::LedgerRequest,
    static_strings::{BACKEND_URL, LEDGERS},
};

// db
pub async fn insert_ledger_integration(
    env: String,
    ledger: LedgerRequest,
) -> Result<Response, String> {
    Client::new()
        .post(format!("{BACKEND_URL}{LEDGERS}/{env}"))
        .json(&ledger)
        .send()
        .await
        .map_err(|err| err.to_string())
}

// db
pub async fn select_ledger_integration(
    env: String,
    query: LedgerRequest,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{LEDGERS}/{env}"))
        .query(&query)
        .send()
        .await
        .map_err(|err| err.to_string())
}

// db
pub async fn select_ledgers_integration(
    env: String,
    query: LedgerRequest,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{LEDGERS}/{env}/all"))
        .query(&query)
        .send()
        .await
        .map_err(|err| err.to_string())
}
