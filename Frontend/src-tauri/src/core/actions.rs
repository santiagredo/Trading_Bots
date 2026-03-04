use crate::{
    integration::{
        delete_action_integration, get_active_action_integration, get_active_actions_integration,
        insert_action_integration, select_action_integration, select_actions_integration,
        start_active_actions_integration, stop_active_actions_integration,
        update_action_integration,
    },
    models::{
        entities::actions::Model,
        structs::{ActionRequest, CacheActions},
    },
    utils::handle_response,
};

// db
pub async fn insert_action_core(env: String, action: ActionRequest) -> Result<Model, String> {
    let response = insert_action_integration(env, action).await?;
    handle_response::<Model>(response).await
}

pub async fn select_action_core(
    env: String,
    action: Option<ActionRequest>,
) -> Result<Model, String> {
    let response = select_action_integration(env, action).await?;
    handle_response::<Model>(response).await
}

pub async fn select_actions_core(
    env: String,
    action: Option<ActionRequest>,
) -> Result<Vec<Model>, String> {
    let response = select_actions_integration(env, action).await?;
    handle_response::<Vec<Model>>(response).await
}

pub async fn update_action_core(env: String, action: ActionRequest) -> Result<Model, String> {
    let response = update_action_integration(env, action).await?;
    handle_response::<Model>(response).await
}

pub async fn delete_action_core(env: String, action: ActionRequest) -> Result<u64, String> {
    let response = delete_action_integration(env, action).await?;
    handle_response::<u64>(response).await
}

// cache
pub async fn get_active_action_core(
    env: String,
    action: Option<ActionRequest>,
) -> Result<Model, String> {
    let response = get_active_action_integration(env, action).await?;

    let result = handle_response::<Option<Model>>(response)
        .await?
        .unwrap_or_default();

    Ok(result)
}

pub async fn get_active_actions_core(env: String) -> Result<Option<CacheActions>, String> {
    let response = get_active_actions_integration(env).await?;

    let models = handle_response::<Option<CacheActions>>(response).await?;

    Ok(models)
}

pub async fn start_active_actions_core(env: String) -> Result<(), String> {
    let response = start_active_actions_integration(env).await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to start active actions: {}",
            response.status()
        ))
    }
}

pub async fn stop_active_actions_core(env: String) -> Result<(), String> {
    let response = stop_active_actions_integration(env).await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to stop active actions: {}",
            response.status()
        ))
    }
}
