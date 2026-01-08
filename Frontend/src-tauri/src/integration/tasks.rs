use reqwest::{Client, Response};

use crate::models::structs::TaskRequest;
use crate::static_strings::{BACKEND_URL, TASKS};

//
pub async fn select_task_integration(
    env: String,
    task: Option<TaskRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{TASKS}/{env}"))
        .query(&task)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn select_tasks_integration(
    env: String,
    task: Option<TaskRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{TASKS}/{env}/all"))
        .query(&task)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn update_task_integration_integration(
    env: String,
    task: TaskRequest,
) -> Result<Response, String> {
    Client::new()
        .patch(format!("{BACKEND_URL}{TASKS}/{env}"))
        .json(&task)
        .send()
        .await
        .map_err(|err| err.to_string())
}

// cache
pub async fn select_active_tasks_integration(env: String) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{TASKS}/{env}/memory/all"))
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn start_active_tasks_integration(env: String) -> Result<Response, String> {
    Client::new()
        .post(format!("{BACKEND_URL}{TASKS}/{env}/memory/start"))
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn stop_active_tasks_integration(env: String) -> Result<Response, String> {
    Client::new()
        .post(format!("{BACKEND_URL}{TASKS}/{env}/memory/stop"))
        .send()
        .await
        .map_err(|err| err.to_string())
}
