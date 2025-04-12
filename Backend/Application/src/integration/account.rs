use std::collections::BTreeMap;

use chrono::Utc;
use hmac::{Hmac, Mac};
use models::{enums::AccountInformationResponse, structs::AccountInformation};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use sha2::Sha256;

use crate::{
    config::get_config,
    static_strings::{ACCOUNT_INFORMATION_ENDPOINT, X_MBX_APIKEY},
};

pub async fn binance_get_account_information() -> Result<AccountInformation, String> {
    let endpoint = ACCOUNT_INFORMATION_ENDPOINT;

    let mut account_request_parameters = BTreeMap::from([
        ("timestamp", Utc::now().timestamp_millis().to_string()),
        ("recvWindow", 5000.to_string()),
        ("omitZeroBalances", true.to_string()),
    ]);

    let query_string =
        serde_urlencoded::to_string(&account_request_parameters).map_err(|err| err.to_string())?;

    let mut mac = Hmac::<Sha256>::new_from_slice(&get_config().await.secret_pass.as_bytes())
        .map_err(|e| e.to_string())?;

    mac.update(query_string.as_bytes());

    let result = mac.finalize();
    let signature = hex::encode(result.into_bytes());

    account_request_parameters.insert("signature", signature);

    let api_key =
        HeaderValue::from_str(&get_config().await.api_key).map_err(|err| err.to_string())?;

    let mut headers = HeaderMap::new();
    headers.insert(X_MBX_APIKEY, api_key);

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|err| err.to_string())?;

    let response = client
        .get(endpoint)
        .query(&account_request_parameters)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    let account_response = response
        .json::<AccountInformationResponse>()
        .await
        .map_err(|err| err.to_string())?;

    match account_response {
        AccountInformationResponse::Error(err) => {
            return Err(format!("code: {} - message: {}", err.code, err.msg))
        }
        AccountInformationResponse::AccountInformation(val) => Ok(val),
    }
}
