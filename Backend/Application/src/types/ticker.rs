use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use models::structs::Ticker;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::utils::Core;

#[derive(Debug, Default, Clone)]
pub struct Tickers<Phase = Core> {
    pub phase: PhantomData<Phase>,
    pub model: Ticker,
}

static TICKERS: Lazy<Arc<RwLock<HashMap<String, Ticker>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

impl Tickers {
    pub async fn set_ticker(model: Ticker) {
        let mut tickers = TICKERS.write().await;

        if let Some(existing) = tickers.get_mut(&model.symbol) {
            existing.event_time = model.event_time;
            existing.price_change = model.price_change;
            existing.price_change_percent = model.price_change_percent;
            existing.weighted_avg_price = model.weighted_avg_price;
            existing.first_trade_before_24h = model.first_trade_before_24h;
            existing.last_price = model.last_price;
            existing.last_quantity = model.last_quantity;
            existing.best_bid_price = model.best_bid_price;
            existing.best_bid_quantity = model.best_bid_quantity;
            existing.best_ask_price = model.best_ask_price;
            existing.best_ask_quantity = model.best_ask_quantity;
            existing.open_price = model.open_price;
            existing.high_price = model.high_price;
            existing.low_price = model.low_price;
            existing.base_asset_volume = model.base_asset_volume;
            existing.quote_asset_volume = model.quote_asset_volume;
            existing.stats_open_time = model.stats_open_time;
            existing.stats_close_time = model.stats_close_time;
            existing.first_trade_id = model.first_trade_id;
            existing.last_trade_id = model.last_trade_id;
            existing.total_trades = model.total_trades;
        } else {
            tickers.insert(model.symbol.clone(), model);
        }
    }

    pub async fn get_ticker(symbol: String) -> Option<Ticker> {
        let tickers = TICKERS.read().await;
        tickers.get(&symbol).cloned()
    }
}
