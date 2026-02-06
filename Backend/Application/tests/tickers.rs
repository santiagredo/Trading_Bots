use sea_orm::prelude::Decimal;

use application::handler::Tickers;
use models::structs::Ticker;

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

#[tokio::test]
async fn full_ticker_cache_flow_should_work_correctly() {
    let symbol = "BTCUSDT";

    // Initial state
    let initial = Tickers::get_ticker_cache(symbol.to_string()).await;
    assert!(initial.is_none());

    // Insert
    let first = mock_ticker(symbol, 30_000, 100);
    Tickers::set_ticker_cache(first.clone()).await;

    let cached = Tickers::get_ticker_cache(symbol.to_string()).await.unwrap();
    assert_eq!(cached.last_price, d(30_000));

    // Update same symbol
    let updated = mock_ticker(symbol, 30_500, 200);
    Tickers::set_ticker_cache(updated.clone()).await;

    let cached_after = Tickers::get_ticker_cache(symbol.to_string()).await.unwrap();

    assert_eq!(cached_after.last_price, d(30_500));
    assert_eq!(cached_after.event_time, 200);
}
