export interface Ticker {
    e: string; // event_type
    E: number; // event_time
    s: string; // symbol

    p: string; // price_change
    P: string; // price_change_percent
    w: string; // weighted_avg_price
    x: string; // first_trade_before_24h
    c: string; // last_price
    Q: string; // last_quantity

    b: string; // best_bid_price
    B: string; // best_bid_quantity
    a: string; // best_ask_price
    A: string; // best_ask_quantity

    o: string; // open_price
    h: string; // high_price
    l: string; // low_price

    v: string; // base_asset_volume
    q: string; // quote_asset_volume

    O: number; // stats_open_time
    C: number; // stats_close_time
    F: number; // first_trade_id
    L: number; // last_trade_id
    n: number; // total_trades
}
