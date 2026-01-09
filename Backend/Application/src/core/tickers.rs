use crate::{
    handler::Tickers,
    utils::{Cache, Core},
};
use models::structs::Ticker;

impl Tickers<Core> {
    pub async fn set_ticker_core(model: Ticker) {
        Tickers::<Cache>::set_ticker_cache(model).await
    }

    pub async fn get_ticker_core(symbol: String) -> Option<Ticker> {
        Tickers::<Cache>::get_ticker_cache(symbol).await
    }
}
