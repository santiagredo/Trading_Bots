use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ack {
    symbol: String,
    order_id: i64,
    order_list_id: i64,
    client_order_id: String,
    transact_time: i64,
}