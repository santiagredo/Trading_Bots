use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct StreamResponse {
    pub result: Option<serde_json::Value>,
    pub id: u64,
}
