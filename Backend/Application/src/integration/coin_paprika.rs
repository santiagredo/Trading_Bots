use reqwest::{Client, Error, Response};

use crate::{
    handler::CoinPaprika, static_strings::COINPAPRIKA_TICKERS_ENDPOINT, utils::Integration,
};

impl CoinPaprika<Integration> {
    pub async fn get_tickers_integration(self) -> Result<Response, Error> {
        Client::new().get(COINPAPRIKA_TICKERS_ENDPOINT).send().await
    }
}
