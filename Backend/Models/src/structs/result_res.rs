use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultRes {
    symbol: String,
    order_id: i64,
    order_list_id: i64,
    client_order_id: String,
    transact_time: i64,
    price: String,
    orig_qty: String,
    executed_qty: String,
    orig_quote_order_qty: String,
    cummulative_quote_qty: String,
    status: String,
    time_in_force: String,
    #[serde(rename = "type")]
    result_type: String,
    side: String,
    working_time: i64,
    self_trade_prevention_mode: String,
}