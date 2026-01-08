use crate::{core::post_user_command_core, models::enums::UserCommands};

#[tauri::command]
pub async fn post_user_command(env: String, command: UserCommands) -> Result<Option<()>, String> {
    post_user_command_core(env, command).await
}
