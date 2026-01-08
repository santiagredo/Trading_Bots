use reqwest::{Client, Response};

use crate::models::structs::ActionRequest;
use crate::static_strings::{ACTIONS, BACKEND_URL};

// db
pub async fn insert_action_integration(
    env: String,
    action: ActionRequest,
) -> Result<Response, String> {
    Client::new()
        .post(format!("{BACKEND_URL}{ACTIONS}/{env}"))
        .json(&action)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn select_action_integration(
    env: String,
    action: Option<ActionRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{ACTIONS}/{env}"))
        .query(&action)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn select_actions_integration(
    env: String,
    action: Option<ActionRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{ACTIONS}/{env}/all"))
        .query(&action)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn update_action_integration(
    env: String,
    action: ActionRequest,
) -> Result<Response, String> {
    Client::new()
        .patch(format!("{BACKEND_URL}{ACTIONS}/{env}"))
        .json(&action)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn delete_action_integration(
    env: String,
    action: ActionRequest,
) -> Result<Response, String> {
    Client::new()
        .delete(format!("{BACKEND_URL}{ACTIONS}/{env}"))
        .json(&action)
        .send()
        .await
        .map_err(|err| err.to_string())
}

// cache
pub async fn get_active_action_integration(
    env: String,
    action: Option<ActionRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{ACTIONS}/{env}/memory"))
        .query(&action)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn get_active_actions_integration(env: String) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{ACTIONS}/{env}/memory/all"))
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn start_active_actions_integration(env: String) -> Result<Response, String> {
    Client::new()
        .post(format!("{BACKEND_URL}{ACTIONS}/{env}/memory/start"))
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn stop_active_actions_integration(env: String) -> Result<Response, String> {
    Client::new()
        .post(format!("{BACKEND_URL}{ACTIONS}/{env}/memory/stop"))
        .send()
        .await
        .map_err(|err| err.to_string())
}
