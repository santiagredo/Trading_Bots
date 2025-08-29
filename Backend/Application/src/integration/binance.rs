use std::collections::BTreeMap;

use chrono::Utc;
use hmac::{Hmac, Mac};
use models::{
    entities::orders,
    enums::{AccountInformationResponse, BinanceResponse, NewOrderResponseType, OrderType, Side},
    structs::{AccountInformation, BinanceOrderRequest, ExchangeInformation},
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use sea_orm::prelude::Decimal;
use sha2::Sha256;

use crate::{
    config::get_config,
    environments::Environments,
    static_strings::{
        ACCOUNT_INFORMATION_ENDPOINT, EXCHANGE_INFORMATION_ENDPOINT, ORDERS_ENDPOINT,
        ORDERS_TEST_ENDPOINT, X_MBX_APIKEY,
    },
    types::Binance,
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
        environment: Environments,
        symbol: String,
        order: &mut orders::Model,
    ) -> Result<(), String> {
        let endpoint = match environment {
            crate::environments::Environments::PRO => ORDERS_ENDPOINT,
            _ => ORDERS_TEST_ENDPOINT,
        };

        let side = Side::Buy;
        let order_type = OrderType::Market;
        let new_client_order_id = Some(order.id.to_string());
        let strategy_id = Some(order.strategy_id as i64);
        let new_order_resp_type = Some(NewOrderResponseType::Full);
        let timestamp = Utc::now().timestamp_millis();
        // let compute_commission_rates = Some(true);

        let mut order_request = BinanceOrderRequest {
            symbol,
            side,
            order_type,
            new_client_order_id,
            strategy_id,
            new_order_resp_type,
            timestamp,
            // compute_commission_rates,
            ..Default::default()
        };

        if order.is_sell {
            order_request.side = Side::Sell;
            order_request.quantity = Some(order.base_asset_amount.round_dp(8));
        } else {
            // dbg!(order.quote_asset_amount.round_dp(8));

            order_request.quote_order_qty = Some(order.quote_asset_amount.round_dp(8));
        }

        // dbg!(order_request.quantity);
        // dbg!(order_request.quote_order_qty);

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

        let response = client
            .post(endpoint)
            .form(&order_request)
            .send()
            .await
            .map_err(|err| err.to_string())?;

        let full_response = response
            .json::<BinanceResponse>()
            .await
            .map_err(|err| err.to_string())?;

        // match response.text().await {
        //     Ok(val) => println!("{val}"),
        //     Err(err) => {
        //         println!("{err:?}");
        //         return Err(err.to_string());
        //     }
        // };

        match full_response {
            BinanceResponse::Error(err) => {
                return Err(format!("code: {} - message: {}", err.code, err.msg))
            }
            BinanceResponse::Full(full) => {
                let weighted_average_price = full
                    .fills
                    .iter()
                    .fold(Decimal::ZERO, |acc, e| acc + (e.price * e.qty))
                    / full.fills.iter().fold(Decimal::ZERO, |acc, e| acc + e.qty);

                let commission = full
                    .fills
                    .iter()
                    .fold(Decimal::ZERO, |acc, e| acc + e.commission);

                let cummulative_quote_asset_amount = full.cummulative_quote_qty + commission;

                order.base_asset_amount = full.executed_qty;
                order.quote_asset_amount = cummulative_quote_asset_amount;
                order.price_entry = weighted_average_price;

                Ok(())
            }
            _ => Ok(()),
        }

        // Err("Testing".to_string())
    }
}
