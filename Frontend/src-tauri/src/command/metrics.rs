use crate::{
    core::{get_active_metric_core, select_metrics_core},
    models::entities::critical_metrics::Model,
};

#[tauri::command]
pub async fn select_metrics(env: String) -> Result<Vec<Model>, String> {
    select_metrics_core(env).await
}

#[tauri::command]
pub async fn get_active_metric(env: String) -> Result<Option<Model>, String> {
    get_active_metric_core(env).await
}
