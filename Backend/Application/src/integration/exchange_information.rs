use models::structs::ExchangeInformation;
use reqwest::Client;

use crate::static_strings::EXCHANGE_INFORMATION_ENDPOINT;

pub async fn binance_get_exchange_information() -> Result<ExchangeInformation, String> {
    let response = Client::new()
        .get(EXCHANGE_INFORMATION_ENDPOINT)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    response
        .json::<ExchangeInformation>()
        .await
        .map_err(|err| err.to_string())
}
