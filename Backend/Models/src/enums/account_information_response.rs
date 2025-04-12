use serde::Deserialize;

use crate::structs::{AccountInformation, BinanceError};

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum AccountInformationResponse {
    Error(BinanceError),
    AccountInformation(AccountInformation),
}
