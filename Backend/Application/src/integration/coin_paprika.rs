use models::structs::CoinPaprikaTicker;
use reqwest::Client;

use crate::{static_strings::COINPAPRIKA_TICKERS_ENDPOINT, types::CoinPaprika, utils::Integration};

impl CoinPaprika<Integration> {
    pub async fn get_tickers_integration(self) -> Result<Vec<CoinPaprikaTicker>, String> {
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
}
