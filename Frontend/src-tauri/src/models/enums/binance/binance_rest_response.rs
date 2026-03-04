use serde::Deserialize;

use crate::models::structs::{BinanceError, Full};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BinanceRestResponse {
    Error(BinanceError),
    Full(Full),
}
