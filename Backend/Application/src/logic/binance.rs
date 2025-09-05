use chrono::Utc;
use models::{
    entities::orders,
    enums::{BinanceResponse, NewOrderResponseType, OrderType, Side},
    structs::{BinanceOrderRequest, OrderRequest},
};
use sea_orm::prelude::Decimal;

use crate::{environments::Environments, handler::Binance, utils::Logic};

impl Binance<Logic> {
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

    pub async fn post_new_order_logic(
        environment: Environments,
        binance_response: String,
        order: &mut OrderRequest,
    ) -> Result<(), String> {
        // in non production environtments: {} means a successful order
        if environment != Environments::PRO && binance_response == "{}".to_string() {
            return Ok(());
        }

        let binance_response =
            serde_json::from_str(&binance_response).map_err(|err| err.to_string())?;

        match binance_response {
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

                order.base_asset_amount = Some(full.executed_qty);
                order.quote_asset_amount = Some(cummulative_quote_asset_amount);
                order.price_entry = Some(weighted_average_price);

                Ok(())
            }
            _ => Err(format!("Invalid binance response received")),
        }
    }
}
