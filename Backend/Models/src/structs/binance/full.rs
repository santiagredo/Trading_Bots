use serde::{Deserialize, Serialize};

use super::Fill;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Full {
    pub symbol: String,
    pub order_id: i64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: i64,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub orig_quote_order_qty: String,
    pub cummulative_quote_qty: String,
    pub status: String,

    #[serde(default)]
    pub time_in_force: Option<String>,

    #[serde(rename = "type")]
    pub full_type: String,

    pub side: String,

    #[serde(default)]
    pub working_time: Option<i64>,

    #[serde(default)]
    pub fills: Option<Vec<Fill>>,

    #[serde(default)]
    pub strategy_id: Option<i64>,

    #[serde(default)]
    pub self_trade_prevention_mode: Option<String>,
}
