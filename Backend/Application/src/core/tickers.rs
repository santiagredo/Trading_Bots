use crate::handler::Tickers;
use models::structs::Ticker;

impl Tickers {
    pub async fn set_ticker_core(model: Ticker) {
        Tickers::set_ticker_cache(model).await
    }

    pub async fn get_ticker_core(symbol: String) -> Option<Ticker> {
        Tickers::get_ticker_cache(symbol).await
    }
}
