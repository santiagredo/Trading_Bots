use std::{collections::BTreeMap, str::FromStr};

use chrono::Utc;
use hmac::{Hmac, Mac};
use models::{
    entities::orders,
    enums::{BinanceRestResponse, NewOrderResponseType, OrderType, Side},
    structs::{BinanceOrderRequest, OrderRequest},
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, RequestBuilder,
};
use sea_orm::prelude::Decimal;
use sha2::Sha256;

use crate::utils::{handle_user_err, Response};

pub fn build_account_request(
    endpoint: &str,
    secret_pass: &str,
    api_key: &str,
    x_mbx_apikey: &'static str,
) -> Result<RequestBuilder, String> {
    if secret_pass.is_empty() {
        return Err(format!("Binance secret pass can't be empty"));
    }

    if secret_pass.is_empty() {
        return Err(format!("Binance api key can't be empty"));
    }

    let mut account_request_parameters = BTreeMap::from([
        ("timestamp", Utc::now().timestamp_millis().to_string()),
        ("recvWindow", 5000.to_string()),
        ("omitZeroBalances", true.to_string()),
    ]);

    let query_string =
        serde_urlencoded::to_string(&account_request_parameters).map_err(|err| err.to_string())?;

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

pub fn build_order(
    symbol: String,
    order: orders::Model,
    api_key: &str,
    secret_pass: &str,
) -> Result<BinanceOrderRequest, Response> {
    if secret_pass.is_empty() {
        return Err(handle_user_err(format!(
            "Binance secret pass can't be empty"
        )));
    }

    if api_key.is_empty() {
        return Err(handle_user_err(format!("Binance api key can't be empty")));
    }

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

    let order = BinanceOrderRequest {
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
    };

    Ok(order)
}

pub fn build_order_request(
    endpoint: &str,
    secret_pass: &str,
    api_key: &str,
    x_mbx_apikey: &'static str,
    order_request: &mut BinanceOrderRequest,
) -> Result<RequestBuilder, Response> {
    let query_string = serde_urlencoded::to_string(&order_request)
        .map_err(|err| Response::server_error(err.to_string()))?;

    let mut mac = Hmac::<Sha256>::new_from_slice(secret_pass.as_bytes())
        .map_err(|err| Response::server_error(err.to_string()))?;

    mac.update(query_string.as_bytes());

    let result = mac.finalize();
    let signature = hex::encode(result.into_bytes());

    order_request.signature = Some(signature);

    let api_key =
        HeaderValue::from_str(api_key).map_err(|err| Response::server_error(err.to_string()))?;

    let mut headers = HeaderMap::new();
    headers.insert(x_mbx_apikey, api_key);

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|err| Response::server_error(err.to_string()))?;

    Ok(client.post(endpoint).form(&order_request))
}

pub fn map_new_order(
    binance_response: BinanceRestResponse,
    order: &mut OrderRequest,
) -> Result<(), Response> {
    match binance_response {
        BinanceRestResponse::Error(err) => Err(Response {
            code: 400,
            message: format!("code: {} - message: {}", err.code, err.msg),
        }),

        BinanceRestResponse::Full(full) => {
            let executed_qty = Decimal::from_str(&full.executed_qty).unwrap_or_default();

            let cummulative_quote_qty =
                Decimal::from_str(&full.cummulative_quote_qty).unwrap_or_default();

            let commission = full
                .fills
                .unwrap_or_default()
                .into_iter()
                .fold(Decimal::ZERO, |acc, f| {
                    acc + Decimal::from_str(&f.commission).unwrap_or_default()
                });

            let avg_price = if executed_qty.is_zero() {
                Decimal::ZERO
            } else {
                cummulative_quote_qty / executed_qty
            };

            order.base_asset_amount = Some(executed_qty);
            order.quote_asset_amount = Some(cummulative_quote_qty + commission);
            order.price_entry = Some(avg_price);

            Ok(())
        }
    }
}
