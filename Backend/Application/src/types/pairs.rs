use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use models::{entities::pairs::Model, structs::PairRequest};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::utils::{Core, Response, Types};

#[derive(Debug, Default)]
pub struct Pairs<Phase = Types> {
    pub phase: PhantomData<Phase>,
    pub model: PairRequest,
}

static ACTIVE_PAIRS: Lazy<Arc<RwLock<Option<HashMap<i32, Model>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Pairs {
    pub fn new(model: PairRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: PairRequest {
                ..Default::default()
            },
        }
    }

    pub fn from_model(mut self, m: Model) -> Self {
        let pair_request = PairRequest {
            id: Some(m.id),
            base_asset_id: Some(m.base_asset_id),
            quote_asset_id: Some(m.quote_asset_id),
            symbol: Some(m.symbol),
            update_date: Some(m.update_date),
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

    async fn set_active_pairs(pairs: Option<Vec<Model>>) -> Option<Vec<Model>> {
        let mut memory_pairs = ACTIVE_PAIRS.write().await;

        let Some(pairs) = pairs else {
            *memory_pairs = None;
            return None;
        };

        let mut active_pairs_map: HashMap<i32, Model> = HashMap::new();

        for pair in pairs.iter() {
            active_pairs_map.insert(pair.id, pair.clone());
        }

        *memory_pairs = Some(active_pairs_map);
        Some(pairs)
    }

    pub async fn get_active_pairs() -> Option<HashMap<i32, Model>> {
        let memory_pairs = ACTIVE_PAIRS.read().await;

        memory_pairs.clone()
    }

    pub async fn get_active_pair(key: &i32) -> Option<Model> {
        let active_pairs = ACTIVE_PAIRS.read().await;

        let Some(pairs_map) = active_pairs.as_ref() else {
            return None;
        };

        pairs_map.get(key).cloned()
    }

    pub async fn start_active_pairs() -> Result<(), Response> {
        if Self::get_active_pairs()
            .await
            .is_none_or(|pairs| pairs.is_empty())
        {
            let pairs = Self::default().select_pairs().await?;
            Self::set_active_pairs(Some(pairs)).await;
        }

        Ok(())
    }

    pub async fn stop_active_pairs() {
        Self::set_active_pairs(None).await;
    }
}

impl<Phase> Pairs<Phase> {
    pub fn next_phase<Next>(self) -> Pairs<Next> {
        Pairs {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl Pairs<Types> {
    pub async fn insert_pair(self) -> Result<Model, Response> {
        self.next_phase::<Core>().insert_pair_core().await
    }

    pub async fn select_pair(self) -> Result<Option<Model>, Response> {
        self.next_phase::<Core>().select_pair_core().await
    }

    pub async fn select_pairs(self) -> Result<Vec<Model>, Response> {
        self.next_phase::<Core>().select_pairs_core().await
    }

    pub async fn update_pair(self) -> Result<Model, Response> {
        self.next_phase::<Core>().update_pair_core().await
    }
}
