use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    pub price: String,
    pub qty: String,
    pub commission: String,
    pub commission_asset: String,
    pub trade_id: i64,
}
