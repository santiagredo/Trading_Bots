use std::{collections::BTreeMap, str::FromStr};

use chrono::Utc;
use hmac::{Hmac, Mac};
use models::{
    entities::orders,
    enums::{BinanceRestResponse, NewOrderResponseType, OrderType, Side},
    structs::{BinanceOrderRequest, Environments, OrderRequest},
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, RequestBuilder,
};
use sea_orm::prelude::Decimal;
use sha2::Sha256;

use crate::{handler::Binance, utils::Logic};

impl Binance<Logic> {
    pub fn get_account_logic(
        endpoint: &str,
        secret_pass: &str,
        api_key: &str,
        x_mbx_apikey: &'static str,
    ) -> Result<RequestBuilder, String> {
        let mut account_request_parameters = BTreeMap::from([
            ("timestamp", Utc::now().timestamp_millis().to_string()),
            ("recvWindow", 5000.to_string()),
            ("omitZeroBalances", true.to_string()),
        ]);

        let query_string = serde_urlencoded::to_string(&account_request_parameters)
            .map_err(|err| err.to_string())?;

        let mut mac =
            Hmac::<Sha256>::new_from_slice(secret_pass.as_bytes()).map_err(|e| e.to_string())?;

        mac.update(query_string.as_bytes());

        let result = mac.finalize();
        let signature = hex::encode(result.into_bytes());

        account_request_parameters.insert("signature", signature);

        let api_key = HeaderValue::from_str(api_key).map_err(|err| err.to_string())?;

        let mut headers = HeaderMap::new();
        headers.insert(x_mbx_apikey, api_key);

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|err| err.to_string())?;

        Ok(client.get(endpoint).query(&account_request_parameters))
    }

    pub fn build_binance_order_request_logic(
        symbol: String,
        order: orders::Model,
    ) -> BinanceOrderRequest {
        let side = match order.is_sell {
            false => Side::Buy,
            true => Side::Sell,
        };
        let order_type = OrderType::Market;
        let new_client_order_id = Some(order.id.to_string());
        let strategy_id = Some(order.strategy_id as i64);
        let new_order_resp_type = Some(NewOrderResponseType::Full);
        let timestamp = Utc::now().timestamp_millis();
        // let compute_commission_rates = Some(true);

        BinanceOrderRequest {
            symbol,
            side,
            order_type,
            new_client_order_id,
            strategy_id,
            new_order_resp_type,
            timestamp,
            quantity: Some(order.base_asset_amount),
            // compute_commission_rates,
            ..Default::default()
        }
    }

    pub fn post_new_order_logic(
        endpoint: &str,
        secret_pass: &str,
        api_key: &str,
        x_mbx_apikey: &'static str,
        order_request: &mut BinanceOrderRequest,
    ) -> Result<RequestBuilder, String> {
        let query_string =
            serde_urlencoded::to_string(&order_request).map_err(|err| err.to_string())?;

        let mut mac =
            Hmac::<Sha256>::new_from_slice(secret_pass.as_bytes()).map_err(|e| e.to_string())?;

        mac.update(query_string.as_bytes());

        let result = mac.finalize();
        let signature = hex::encode(result.into_bytes());

        order_request.signature = Some(signature);

        let api_key = HeaderValue::from_str(api_key).map_err(|err| err.to_string())?;

        let mut headers = HeaderMap::new();
        headers.insert(x_mbx_apikey, api_key);

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|err| err.to_string())?;

        Ok(client.post(endpoint).form(&order_request))
    }

    pub async fn map_new_order_logic(
        environment: Environments,
        binance_response: String,
        order: &mut OrderRequest,
    ) -> Result<(), String> {
        // in non production environtments: {} means a successful order
        if environment != Environments::PROD && binance_response == "{}".to_string() {
            return Ok(());
        }

        let inner =
            serde_json::from_str::<String>(&binance_response).unwrap_or(binance_response.clone());

        let binance_response: BinanceRestResponse =
            serde_json::from_str(&inner).map_err(|e| e.to_string())?;

        match binance_response {
            BinanceRestResponse::Error(err) => {
                Err(format!("code: {} - message: {}", err.code, err.msg))
            }

            BinanceRestResponse::Full(full) => {
                let fills = full.fills.unwrap_or_default();

                let weighted_average_price = {
                    let total_qty = fills.iter().fold(Decimal::ZERO, |acc, f| {
                        acc + Decimal::from_str(&f.qty).unwrap_or_default()
                    });
                    if total_qty.is_zero() {
                        Decimal::ZERO
                    } else {
                        fills.iter().fold(Decimal::ZERO, |acc, f| {
                            acc + (Decimal::from_str(&f.price).unwrap_or_default()
                                * Decimal::from_str(&f.qty).unwrap_or_default())
                        }) / total_qty
                    }
                };

                let commission = fills.iter().fold(Decimal::ZERO, |acc, f| {
                    acc + Decimal::from_str(&f.commission).unwrap_or_default()
                });

                let cummulative_quote_asset_amount =
                    Decimal::from_str(&full.cummulative_quote_qty).unwrap_or_default() + commission;

                order.base_asset_amount =
                    Some(Decimal::from_str(&full.executed_qty).unwrap_or_default());
                order.quote_asset_amount = Some(cummulative_quote_asset_amount);
                order.price_entry = Some(weighted_average_price);

                Ok(())
            }
        }
    }
}
