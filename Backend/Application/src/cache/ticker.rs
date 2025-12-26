use crate::{handler::Tickers, utils::Cache};

use std::{collections::HashMap, sync::Arc};

use models::structs::Ticker;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

static TICKERS: Lazy<Arc<RwLock<HashMap<String, Ticker>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

impl Tickers<Cache> {
    pub async fn set_ticker_cache(model: Ticker) {
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

    pub async fn get_ticker_cache(symbol: String) -> Option<Ticker> {
        let tickers = TICKERS.read().await;
        tickers.get(&symbol).cloned()
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::prelude::Decimal;

    use crate::{handler::Tickers, utils::Cache};
    use models::structs::Ticker;

    // Helpers
    fn d(val: i64) -> Decimal {
        Decimal::new(val, 0)
    }

    fn mock_ticker(symbol: &str, last_price: i64, event_time: u64) -> Ticker {
        Ticker {
            event_type: "24hrTicker".to_string(),
            event_time,
            symbol: symbol.to_string(),
            price_change: d(10),
            price_change_percent: d(2),
            weighted_avg_price: d(100),
            first_trade_before_24h: d(90),
            last_price: d(last_price),
            last_quantity: d(1),
            best_bid_price: d(99),
            best_bid_quantity: d(2),
            best_ask_price: d(101),
            best_ask_quantity: d(2),
            open_price: d(95),
            high_price: d(110),
            low_price: d(90),
            base_asset_volume: d(1000),
            quote_asset_volume: d(100_000),
            stats_open_time: 1,
            stats_close_time: 2,
            first_trade_id: 10,
            last_trade_id: 20,
            total_trades: 100,
        }
    }

    async fn reset_ticker(symbol: &str) {
        let dummy = mock_ticker(symbol, 0, 0);
        Tickers::<Cache>::set_ticker_cache(dummy).await;
    }

    // Scenarios (unit responsibilities)
    async fn scenario_insert_new_ticker() {
        let symbol = "BTCUSDT";
        reset_ticker(symbol).await;

        let ticker = mock_ticker(symbol, 30_000, 100);
        Tickers::<Cache>::set_ticker_cache(ticker.clone()).await;

        let cached = Tickers::<Cache>::get_ticker_cache(symbol.to_string()).await;

        assert_eq!(cached, Some(ticker));
    }

    async fn scenario_update_existing_ticker() {
        let symbol = "ETHUSDT";
        reset_ticker(symbol).await;

        let initial = mock_ticker(symbol, 2_000, 100);
        Tickers::<Cache>::set_ticker_cache(initial).await;

        let updated = mock_ticker(symbol, 2_100, 200);
        Tickers::<Cache>::set_ticker_cache(updated.clone()).await;

        let cached = Tickers::<Cache>::get_ticker_cache(symbol.to_string())
            .await
            .expect("ticker should exist");

        assert_eq!(cached.last_price, d(2_100));
        assert_eq!(cached.event_time, 200);
        assert_eq!(cached.symbol, symbol);
    }

    async fn scenario_get_non_existing_ticker() {
        let cached = Tickers::<Cache>::get_ticker_cache("UNKNOWN".to_string()).await;
        assert!(cached.is_none());
    }

    async fn scenario_symbol_is_not_duplicated() {
        let symbol = "BNBUSDT";
        reset_ticker(symbol).await;

        let first = mock_ticker(symbol, 300, 1);
        Tickers::<Cache>::set_ticker_cache(first).await;

        let second = mock_ticker(symbol, 320, 2);
        Tickers::<Cache>::set_ticker_cache(second.clone()).await;

        let cached = Tickers::<Cache>::get_ticker_cache(symbol.to_string())
            .await
            .unwrap();

        assert_eq!(cached.last_price, d(320));
        assert_eq!(cached.event_time, 2);
    }

    #[tokio::test]
    async fn tickers_cache_unit_responsibilities() {
        scenario_insert_new_ticker().await;
        scenario_update_existing_ticker().await;
        scenario_get_non_existing_ticker().await;
        scenario_symbol_is_not_duplicated().await;
    }
}
