export interface Pair {
    id: number;
    base_asset_id: number;
    quote_asset_id: number;
    symbol: string;

    update_date: string;

    all_time_high_price: number;
    all_time_high_date: string;
    percent_from_all_time_high: number;

    fifteen_minutes_price_percent_change: number;
    thirty_minutes_price_percent_change: number;
    hour_price_percent_change: number;
    six_hours_price_percent_change: number;
    twelve_hours_price_percent_change: number;
    day_price_percent_change: number;
    week_price_percent_change: number;
    month_price_percent_change: number;
    year_price_percent_change: number;

    price_filter_min_price: number;
    price_filter_max_price: number;
    price_filter_tick_size: number;

    lot_size_min_qty: number;
    lot_size_max_qty: number;
    lot_size_step_size: number;

    iceberg_parts_limit: number;

    market_lot_size_min_qty: number;
    market_lot_size_max_qty: number;
    market_lot_size_step_size: number;

    trailing_delta_min_trailing_above_delta: number;
    trailing_delta_max_trailing_above_delta: number;
    trailing_delta_min_trailing_below_delta: number;
    trailing_delta_max_trailing_below_delta: number;

    percent_price_by_side_bid_multiplier_up: number;
    percent_price_by_side_bid_multiplier_down: number;
    percent_price_by_side_ask_multiplier_up: number;
    percent_price_by_side_ask_multiplier_down: number;
    percent_price_by_side_avg_price_mins: number;

    notional_min_notional: number;
    notional_apply_min_to_market: boolean;
    notional_max_notional: number;
    notional_apply_max_to_market: boolean;
    notional_avg_price_mins: number;

    max_num_orders: number;
    max_num_algo_orders: number;
}
