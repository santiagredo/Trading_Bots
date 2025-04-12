use std::{marker::PhantomData, sync::Arc};

use models::structs::MiniTicker;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::utils::Core;

#[derive(Debug, Default, Clone)]
pub struct MiniTickers<Phase = Core> {
    pub phase: PhantomData<Phase>,
    pub model: MiniTicker,
}

static MINI_TICKERS: Lazy<Arc<RwLock<Vec<MiniTicker>>>> =
    Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

impl MiniTickers {
    pub async fn set_mini_ticker(model: MiniTicker) {
        let mut mini_tickers = MINI_TICKERS.write().await;

        if let Some(existing) = mini_tickers
            .iter_mut()
            .find(|item| item.symbol == model.symbol && item.event_type == model.event_type)
        {
            existing.event_time = model.event_time;
            existing.last_price = model.last_price;
            existing.open_price = model.open_price;
            existing.high_price = model.high_price;
            existing.low_price = model.low_price;
            existing.base_asset_volume = model.base_asset_volume;
            existing.quote_asset_volume = model.quote_asset_volume;
        } else {
            mini_tickers.push(model);
        }
    }

    pub async fn get_mini_tickers(symbol: String) -> Vec<MiniTicker> {
        let mini_tickers = MINI_TICKERS.read().await;

        let filtered_mini_tickers = mini_tickers
            .iter()
            .filter(|ticker| ticker.symbol == symbol)
            .cloned()
            .collect();

        // println!("Filtered mini tickers: {filtered_mini_tickers:?}");

        filtered_mini_tickers
    }
}
