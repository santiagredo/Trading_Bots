use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Response {
    pub code: u16,
    pub message: String,
}
