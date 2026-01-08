use reqwest::Response;
use serde::de::DeserializeOwned;

pub async fn handle_response<T>(response: Response) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let status = response.status();
    let url = response.url().clone();

    let body = response
        .text()
        .await
        .map_err(|err| format!("Request to text serialization error: {err}"))?;

    if status.is_client_error() {
        return Err(format!("Client error ({status}): {body} - {url}"));
    }

    if status.is_server_error() {
        return Err(format!("Server error ({status}): {body} - {url}"));
    }

    if body.trim().is_empty() {
        return serde_json::from_str("null")
            .map_err(|err| format!("Empty body deserialization error: {err}"));
    }

    serde_json::from_str::<T>(&body)
        .map_err(|err| format!("Text to entity deserialization error: {err}"))
}
