use crate::{integration::post_user_command_integration, utils::handle_response};

pub async fn post_user_command_core(env: String, command: String) -> Result<Option<()>, String> {
    let response = post_user_command_integration(env, command).await?;
    handle_response::<Option<()>>(response).await
}
