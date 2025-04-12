use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

use crate::enums::{OrderStatus, OrderType, Side, TimeInForce};

use super::Fill;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Full {
    pub symbol: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: i64,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    pub orig_quote_order_qty: Decimal,
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub full_type: OrderType,
    pub side: Side,
    pub working_time: i64,
    pub self_trade_prevention_mode: String,
    pub fills: Vec<Fill>,
}
