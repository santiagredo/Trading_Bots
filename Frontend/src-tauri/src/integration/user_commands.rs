use reqwest::{Client, Response};

use crate::{
    models::enums::UserCommands,
    static_strings::{BACKEND_URL, USER_COMMANDS},
};

pub async fn post_user_command_integration(
    env: String,
    command: UserCommands,
) -> Result<Response, String> {
    let command = command.as_str();

    Client::new()
        .post(format!("{BACKEND_URL}{USER_COMMANDS}/{env}/{command}"))
        .send()
        .await
        .map_err(|err| err.to_string())
}
