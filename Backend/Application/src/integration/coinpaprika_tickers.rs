use models::structs::CoinPaprikaTicker;
use reqwest::Client;

use crate::static_strings::COINPAPRIKA_TICKERS_ENDPOINT;

pub async fn coinpaprika_get_tickers() -> Result<Vec<CoinPaprikaTicker>, String> {
    let response = Client::new()
        .get(COINPAPRIKA_TICKERS_ENDPOINT)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    response
        .json::<Vec<CoinPaprikaTicker>>()
        .await
        .map_err(|err| err.to_string())
}
