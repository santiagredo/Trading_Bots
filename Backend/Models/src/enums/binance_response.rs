use serde::Deserialize;

use crate::structs::{BinanceError, Full, MiniTicker, StreamResponse, SubscribeResponse, Ticker};

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)] // Allows different JSON structures in the same enum
pub enum BinanceResponse {
    Error(BinanceError),
    StreamResponse(StreamResponse),
    Ticker(Ticker),
    MiniTicker(MiniTicker),
    SubscribeResponse(SubscribeResponse),
    Full(Full),
}
