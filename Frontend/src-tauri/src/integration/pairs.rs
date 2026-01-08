use reqwest::Response;

use crate::models::structs::PairRequest;

use crate::static_strings::{BACKEND_URL, PAIRS};

pub async fn select_pairs_integration(
    env: String,
    pair: Option<PairRequest>,
) -> Result<Response, String> {
    reqwest::Client::new()
        .get(format!("{BACKEND_URL}{PAIRS}/{env}/all"))
        .query(&pair)
        .send()
        .await
        .map_err(|err| err.to_string())
}
