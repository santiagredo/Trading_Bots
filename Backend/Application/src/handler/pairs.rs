use models::{entities::pairs::Model, structs::PairRequest};

#[derive(Debug, Clone)]
pub struct Pairs<R> {
    pub repo: R,
}

impl<R> Pairs<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl Pairs<()> {
    pub fn blank() -> Pairs<()> {
        Self { repo: () }
    }

    pub fn into_request(m: Model) -> PairRequest {
        let pair_request = PairRequest {
            id: Some(m.id),
            base_asset_id: Some(m.base_asset_id),
            quote_asset_id: Some(m.quote_asset_id),
            symbol: Some(m.symbol),
            update_date: Some(m.update_date),
            last_price: Some(m.last_price),
            all_time_high_price: Some(m.all_time_high_price),
            all_time_high_date: Some(m.all_time_high_date),
            percent_from_all_time_high: Some(m.percent_from_all_time_high),
            fifteen_minutes_price_percent_change: Some(m.fifteen_minutes_price_percent_change),
            thirty_minutes_price_percent_change: Some(m.thirty_minutes_price_percent_change),
            hour_price_percent_change: Some(m.hour_price_percent_change),
            six_hours_price_percent_change: Some(m.six_hours_price_percent_change),
            twelve_hours_price_percent_change: Some(m.twelve_hours_price_percent_change),
            day_price_percent_change: Some(m.day_price_percent_change),
            week_price_percent_change: Some(m.week_price_percent_change),
            month_price_percent_change: Some(m.month_price_percent_change),
            year_price_percent_change: Some(m.year_price_percent_change),
            price_filter_min_price: Some(m.price_filter_min_price),
            price_filter_max_price: Some(m.price_filter_max_price),
            price_filter_tick_size: Some(m.price_filter_tick_size),
            lot_size_min_qty: Some(m.lot_size_min_qty),
            lot_size_max_qty: Some(m.lot_size_max_qty),
            lot_size_step_size: Some(m.lot_size_step_size),
            iceberg_parts_limit: Some(m.iceberg_parts_limit),
            market_lot_size_min_qty: Some(m.market_lot_size_min_qty),
            market_lot_size_max_qty: Some(m.market_lot_size_max_qty),
            market_lot_size_step_size: Some(m.market_lot_size_step_size),
            trailing_delta_min_trailing_above_delta: Some(
                m.trailing_delta_min_trailing_above_delta,
            ),
            trailing_delta_max_trailing_above_delta: Some(
                m.trailing_delta_max_trailing_above_delta,
            ),
            trailing_delta_min_trailing_below_delta: Some(
                m.trailing_delta_min_trailing_below_delta,
            ),
            trailing_delta_max_trailing_below_delta: Some(
                m.trailing_delta_max_trailing_below_delta,
            ),
            percent_price_by_side_bid_multiplier_up: Some(
                m.percent_price_by_side_bid_multiplier_up,
            ),
            percent_price_by_side_bid_multiplier_down: Some(
                m.percent_price_by_side_bid_multiplier_down,
            ),
            percent_price_by_side_ask_multiplier_up: Some(
                m.percent_price_by_side_ask_multiplier_up,
            ),
            percent_price_by_side_ask_multiplier_down: Some(
                m.percent_price_by_side_ask_multiplier_down,
            ),
            percent_price_by_side_avg_price_mins: Some(m.percent_price_by_side_avg_price_mins),
            notional_min_notional: Some(m.notional_min_notional),
            notional_apply_min_to_market: Some(m.notional_apply_min_to_market),
            notional_max_notional: Some(m.notional_max_notional),
            notional_apply_max_to_market: Some(m.notional_apply_max_to_market),
            notional_avg_price_mins: Some(m.notional_avg_price_mins),
            max_num_orders: Some(m.max_num_orders),
            max_num_algo_orders: Some(m.max_num_algo_orders),
        };

        pair_request
    }

    pub fn into_model(req: PairRequest) -> Model {
        Model {
            id: req.id.unwrap_or_default(),
            base_asset_id: req.base_asset_id.unwrap_or_default(),
            quote_asset_id: req.quote_asset_id.unwrap_or_default(),
            symbol: req.symbol.unwrap_or_default(),
            update_date: req.update_date.unwrap_or_default(),
            last_price: req.last_price.unwrap_or_default(),
            all_time_high_price: req.all_time_high_price.unwrap_or_default(),
            all_time_high_date: req.all_time_high_date.unwrap_or_default(),
            percent_from_all_time_high: req.percent_from_all_time_high.unwrap_or_default(),

            fifteen_minutes_price_percent_change: req
                .fifteen_minutes_price_percent_change
                .unwrap_or_default(),
            thirty_minutes_price_percent_change: req
                .thirty_minutes_price_percent_change
                .unwrap_or_default(),
            hour_price_percent_change: req.hour_price_percent_change.unwrap_or_default(),
            six_hours_price_percent_change: req.six_hours_price_percent_change.unwrap_or_default(),
            twelve_hours_price_percent_change: req
                .twelve_hours_price_percent_change
                .unwrap_or_default(),
            day_price_percent_change: req.day_price_percent_change.unwrap_or_default(),
            week_price_percent_change: req.week_price_percent_change.unwrap_or_default(),
            month_price_percent_change: req.month_price_percent_change.unwrap_or_default(),
            year_price_percent_change: req.year_price_percent_change.unwrap_or_default(),

            price_filter_min_price: req.price_filter_min_price.unwrap_or_default(),
            price_filter_max_price: req.price_filter_max_price.unwrap_or_default(),
            price_filter_tick_size: req.price_filter_tick_size.unwrap_or_default(),

            lot_size_min_qty: req.lot_size_min_qty.unwrap_or_default(),
            lot_size_max_qty: req.lot_size_max_qty.unwrap_or_default(),
            lot_size_step_size: req.lot_size_step_size.unwrap_or_default(),

            iceberg_parts_limit: req.iceberg_parts_limit.unwrap_or_default(),

            market_lot_size_min_qty: req.market_lot_size_min_qty.unwrap_or_default(),
            market_lot_size_max_qty: req.market_lot_size_max_qty.unwrap_or_default(),
            market_lot_size_step_size: req.market_lot_size_step_size.unwrap_or_default(),

            trailing_delta_min_trailing_above_delta: req
                .trailing_delta_min_trailing_above_delta
                .unwrap_or_default(),
            trailing_delta_max_trailing_above_delta: req
                .trailing_delta_max_trailing_above_delta
                .unwrap_or_default(),
            trailing_delta_min_trailing_below_delta: req
                .trailing_delta_min_trailing_below_delta
                .unwrap_or_default(),
            trailing_delta_max_trailing_below_delta: req
                .trailing_delta_max_trailing_below_delta
                .unwrap_or_default(),

            percent_price_by_side_bid_multiplier_up: req
                .percent_price_by_side_bid_multiplier_up
                .unwrap_or_default(),
            percent_price_by_side_bid_multiplier_down: req
                .percent_price_by_side_bid_multiplier_down
                .unwrap_or_default(),
            percent_price_by_side_ask_multiplier_up: req
                .percent_price_by_side_ask_multiplier_up
                .unwrap_or_default(),
            percent_price_by_side_ask_multiplier_down: req
                .percent_price_by_side_ask_multiplier_down
                .unwrap_or_default(),
            percent_price_by_side_avg_price_mins: req
                .percent_price_by_side_avg_price_mins
                .unwrap_or_default(),

            notional_min_notional: req.notional_min_notional.unwrap_or_default(),
            notional_apply_min_to_market: req.notional_apply_min_to_market.unwrap_or_default(),
            notional_max_notional: req.notional_max_notional.unwrap_or_default(),
            notional_apply_max_to_market: req.notional_apply_max_to_market.unwrap_or_default(),
            notional_avg_price_mins: req.notional_avg_price_mins.unwrap_or_default(),

            max_num_orders: req.max_num_orders.unwrap_or_default(),
            max_num_algo_orders: req.max_num_algo_orders.unwrap_or_default(),
        }
    }
}
