use std::{collections::HashMap, marker::PhantomData};

use models::{
    entities::pairs::Model,
    enums::LifecycleState,
    structs::{Environments, PairRequest, QueryOptions},
};

use crate::utils::{Core, Response, Types};

#[derive(Debug, Default)]
pub struct Pairs<Phase = Types> {
    pub phase: PhantomData<Phase>,
    pub environment: Environments,
    pub model: PairRequest,
}

impl<Phase> Pairs<Phase> {
    pub fn next_phase<Next>(self) -> Pairs<Next> {
        Pairs {
            phase: PhantomData::<Next>,
            environment: self.environment,
            model: self.model,
        }
    }
}

impl Pairs {
    pub fn new(model: PairRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model: PairRequest {
                ..Default::default()
            },
        }
    }

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: self.phase,
            environment,
            model: self.model,
        }
    }

    pub fn from_model(mut self, m: Model) -> Self {
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

        self.model = pair_request;
        self
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

    // db
    pub async fn insert_pair(self) -> Result<Model, Response> {
        self.next_phase::<Core>().insert_pair_core().await
    }

    pub async fn select_pair(self) -> Result<Option<Model>, Response> {
        self.next_phase::<Core>().select_pair_core().await
    }

    pub async fn select_pairs(self, query: Option<QueryOptions>) -> Result<Vec<Model>, Response> {
        self.next_phase::<Core>().select_pairs_core(query).await
    }

    pub async fn update_pair(self) -> Result<Model, Response> {
        self.next_phase::<Core>().update_pair_core().await
    }

    // cache
    pub async fn get_pairs(self) -> Option<HashMap<i32, Model>> {
        self.next_phase().get_pairs_core().await
    }

    pub async fn get_pair(self) -> Option<Model> {
        self.next_phase().get_pair_core().await
    }

    pub async fn get_pairs_state(self) -> LifecycleState {
        self.next_phase().get_pairs_state_core().await
    }

    pub async fn upsert_pair(self) -> Result<(), Response> {
        self.next_phase().upsert_pair_core().await
    }

    pub async fn remove_pair(self) -> Result<Option<Model>, Response> {
        self.next_phase().remove_pair_core().await
    }

    pub async fn start_pairs(self) -> Result<(), Response> {
        self.next_phase().start_pairs_core().await
    }

    pub async fn stop_pairs(self) -> Result<(), Response> {
        self.next_phase().stop_pairs_core().await
    }

    pub async fn reset_pairs(self) -> Result<(), Response> {
        self.next_phase().reset_pairs_core().await
    }
}
