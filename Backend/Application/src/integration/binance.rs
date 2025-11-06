use std::collections::BTreeMap;

use chrono::Utc;
use hmac::{Hmac, Mac};
use models::{
    enums::AccountInformationResponse,
    structs::{AccountInformation, BinanceOrderRequest, ExchangeInformation},
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response,
};
use sha2::Sha256;

use crate::{
    config::get_config,
    handler::Binance,
    static_strings::{ACCOUNT_INFORMATION_ENDPOINT, EXCHANGE_INFORMATION_ENDPOINT, X_MBX_APIKEY},
    utils::Integration,
};

impl Binance<Integration> {
    pub async fn get_account_integration(self) -> Result<AccountInformation, String> {
        let endpoint = ACCOUNT_INFORMATION_ENDPOINT;

        let mut account_request_parameters = BTreeMap::from([
            ("timestamp", Utc::now().timestamp_millis().to_string()),
            ("recvWindow", 5000.to_string()),
            ("omitZeroBalances", true.to_string()),
        ]);

        let query_string = serde_urlencoded::to_string(&account_request_parameters)
            .map_err(|err| err.to_string())?;

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

    pub async fn get_exchange_information_integration(self) -> Result<ExchangeInformation, String> {
        let response = Client::new()
            .get(EXCHANGE_INFORMATION_ENDPOINT)
            .send()
            .await
            .map_err(|err| err.to_string())?;

        response
            .json::<ExchangeInformation>()
            .await
            .map_err(|err| err.to_string())
    }

    pub async fn post_new_order_integration(
        self,
        endpoint: &str,
        order_request: &mut BinanceOrderRequest,
    ) -> Result<Response, String> {
        let query_string =
            serde_urlencoded::to_string(&order_request).map_err(|err| err.to_string())?;

        let mut mac = Hmac::<Sha256>::new_from_slice(&get_config().await.secret_pass.as_bytes())
            .map_err(|e| e.to_string())?;

        mac.update(query_string.as_bytes());

        let result = mac.finalize();
        let signature = hex::encode(result.into_bytes());

        order_request.signature = Some(signature);

        let api_key =
            HeaderValue::from_str(&get_config().await.api_key).map_err(|err| err.to_string())?;

        let mut headers = HeaderMap::new();
        headers.insert(X_MBX_APIKEY, api_key);

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|err| err.to_string())?;

        client
            .post(endpoint)
            .form(&order_request)
            .send()
            .await
            .map_err(|err| err.to_string())
    }
}
