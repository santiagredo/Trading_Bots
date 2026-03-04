use crate::core::post_user_command_core;

#[tauri::command]
pub async fn post_user_command(env: String, command: String) -> Result<Option<()>, String> {
    post_user_command_core(env, command).await
}
