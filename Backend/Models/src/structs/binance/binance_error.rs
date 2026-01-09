use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BinanceError {
    pub code: i64,
    pub msg: String,
}