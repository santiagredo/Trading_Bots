use reqwest::{Client, Response};

use crate::models::structs::StrategyRequest;
use crate::static_strings::{BACKEND_URL, STRATEGIES};

// db
pub async fn insert_strategy_integration(
    env: String,
    strategy: StrategyRequest,
) -> Result<Response, String> {
    Client::new()
        .post(format!("{BACKEND_URL}{STRATEGIES}/{env}"))
        .json(&strategy)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn select_strategy_integration(
    env: String,
    strategy: Option<StrategyRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{STRATEGIES}/{env}"))
        .query(&strategy)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn select_strategies_integration(
    env: String,
    strategy: Option<StrategyRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{STRATEGIES}/{env}/all"))
        .query(&strategy)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn update_strategy_integration(
    env: String,
    strategy: StrategyRequest,
) -> Result<Response, String> {
    Client::new()
        .patch(format!("{BACKEND_URL}{STRATEGIES}/{env}"))
        .json(&strategy)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn delete_strategy_integration(
    env: String,
    strategy: StrategyRequest,
) -> Result<Response, String> {
    Client::new()
        .delete(format!("{BACKEND_URL}{STRATEGIES}/{env}"))
        .json(&strategy)
        .send()
        .await
        .map_err(|err| err.to_string())
}

// cache
pub async fn get_active_strategy_integration(
    env: String,
    strategy: Option<StrategyRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{STRATEGIES}/{env}/memory"))
        .query(&strategy)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn get_active_strategies_integration(
    env: String,
    strategy: Option<StrategyRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{STRATEGIES}/{env}/memory/all"))
        .query(&strategy)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn start_active_strategies_integration(env: String) -> Result<Response, String> {
    Client::new()
        .post(format!("{BACKEND_URL}{STRATEGIES}/{env}/memory/start"))
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn stop_active_strategies_integration(env: String) -> Result<Response, String> {
    Client::new()
        .post(format!("{BACKEND_URL}{STRATEGIES}/{env}/memory/stop"))
        .send()
        .await
        .map_err(|err| err.to_string())
}
