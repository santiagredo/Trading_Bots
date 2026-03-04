use crate::core::{
    select_active_tasks_core, select_task_core, select_tasks_core, start_active_tasks_core,
    stop_active_tasks_core, update_task_core,
};
use crate::models::entities::tasks::Model;
use crate::models::structs::{CacheTasks, TaskRequest};

// db
#[tauri::command]
pub async fn select_task(env: String, query: Option<TaskRequest>) -> Result<Vec<Model>, String> {
    select_task_core(env, query).await
}

#[tauri::command]
pub async fn select_tasks(env: String, query: Option<TaskRequest>) -> Result<Vec<Model>, String> {
    select_tasks_core(env, query).await
}

#[tauri::command]
pub async fn update_task(env: String, task: TaskRequest) -> Result<Model, String> {
    update_task_core(env, task).await
}

// cache
#[tauri::command]
pub async fn select_active_tasks(env: String) -> Result<Option<CacheTasks>, String> {
    select_active_tasks_core(env).await
}

#[tauri::command]
pub async fn start_active_tasks(env: String) -> Result<(), String> {
    start_active_tasks_core(env).await
}

#[tauri::command]
pub async fn stop_active_tasks(env: String) -> Result<(), String> {
    stop_active_tasks_core(env).await
}
