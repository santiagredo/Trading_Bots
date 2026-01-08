use reqwest::Response;

use crate::models::structs::AssetRequest;

use crate::static_strings::{ASSETS, BACKEND_URL};

pub async fn select_assets_integration(
    env: String,
    asset: Option<AssetRequest>,
) -> Result<Response, String> {
    reqwest::Client::new()
        .get(format!("{BACKEND_URL}{ASSETS}/{env}/all"))
        .query(&asset)
        .send()
        .await
        .map_err(|err| err.to_string())
}
