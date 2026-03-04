use reqwest::{Client, Response};

use crate::static_strings::{BACKEND_URL, USER_COMMANDS};

pub async fn post_user_command_integration(
    env: String,
    command: String,
) -> Result<Response, String> {
    let command = command.as_str();

    Client::new()
        .post(format!("{BACKEND_URL}{USER_COMMANDS}/{env}/{command}"))
        .send()
        .await
        .map_err(|err| err.to_string())
}
