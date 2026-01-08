use crate::{
    models::structs::StrategyRequest,
    static_strings::{BACKEND_URL, STRATEGIES_OVERVIEW},
};
use reqwest::{Client, Response};

pub async fn select_strategies_overview_integration(
    env: String,
    query: StrategyRequest,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{STRATEGIES_OVERVIEW}/{env}"))
        .query(&query)
        .send()
        .await
        .map_err(|err| err.to_string())
}
