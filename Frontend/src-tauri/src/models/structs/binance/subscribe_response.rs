use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscribeResponse {
    pub result: Option<serde_json::Value>,
    pub id: Option<i32>,
}
