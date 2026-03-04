use crate::{
    integration::{
        select_active_tasks_integration, select_task_integration, select_tasks_integration,
        start_active_tasks_integration, stop_active_tasks_integration,
        update_task_integration_integration,
    },
    models::{
        entities::tasks::Model,
        structs::{CacheTasks, TaskRequest},
    },
    utils::handle_response,
};

// db
pub async fn select_task_core(
    env: String,
    query: Option<TaskRequest>,
) -> Result<Vec<Model>, String> {
    let response = select_task_integration(env, query).await?;
    handle_response::<Vec<Model>>(response).await
}

pub async fn select_tasks_core(
    env: String,
    query: Option<TaskRequest>,
) -> Result<Vec<Model>, String> {
    let response = select_tasks_integration(env, query).await?;
    handle_response::<Vec<Model>>(response).await
}

pub async fn update_task_core(env: String, task: TaskRequest) -> Result<Model, String> {
    let response = update_task_integration_integration(env, task).await?;
    handle_response::<Model>(response).await
}

// cache
pub async fn select_active_tasks_core(env: String) -> Result<Option<CacheTasks>, String> {
    let response = select_active_tasks_integration(env).await?;
    handle_response::<Option<CacheTasks>>(response).await
}

pub async fn start_active_tasks_core(env: String) -> Result<(), String> {
    start_active_tasks_integration(env).await?;
    Ok(())
}

pub async fn stop_active_tasks_core(env: String) -> Result<(), String> {
    stop_active_tasks_integration(env).await?;
    Ok(())
}
