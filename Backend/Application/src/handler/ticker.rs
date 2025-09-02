use std::marker::PhantomData;

use models::structs::Ticker;

use crate::utils::Core;

#[derive(Debug, Default, Clone)]
pub struct Tickers<Phase = Core> {
    pub phase: PhantomData<Phase>,
    pub model: Ticker,
}

impl Tickers {
    pub async fn set_ticker(model: Ticker) {
        Tickers::<Core>::set_ticker_core(model).await
    }

    pub async fn get_ticker(symbol: String) -> Option<Ticker> {
        Tickers::<Core>::get_ticker_core(symbol).await
    }
}
