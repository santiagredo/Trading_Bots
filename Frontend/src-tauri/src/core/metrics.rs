use crate::{
    integration::{get_active_metric_integration, select_metrics_integration},
    models::entities::critical_metrics::Model,
    utils::handle_response,
};

// db
pub async fn select_metrics_core(env: String) -> Result<Vec<Model>, String> {
    let response = select_metrics_integration(env).await?;
    handle_response::<Vec<Model>>(response).await
}

// cache
pub async fn get_active_metric_core(env: String) -> Result<Option<Model>, String> {
    let response = get_active_metric_integration(env).await?;
    handle_response::<Option<Model>>(response).await
}
