use serde::Deserialize;

use crate::structs::{BinanceError, Full};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BinanceRestResponse {
    Error(BinanceError),
    Full(Full),
}
