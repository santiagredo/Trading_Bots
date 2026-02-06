use models::structs::Ticker;

#[derive(Debug, Default, Clone)]
pub struct Tickers {
    pub model: Ticker,
}

impl Tickers {
    pub async fn set_ticker(model: Ticker) {
        Tickers::set_ticker_core(model).await
    }

    pub async fn get_ticker(symbol: String) -> Option<Ticker> {
        Tickers::get_ticker_core(symbol).await
    }
}
