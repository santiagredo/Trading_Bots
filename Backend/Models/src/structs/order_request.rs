use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

use crate::enums::{NewOrderResponseType, OrderType, SelfTradePreventionMode, Side, TimeInForce};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub symbol: String,

    pub side: Side,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_delta: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<NewOrderResponseType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,

    pub timestamp: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_commission_rates: Option<bool>,
}
