use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: i64, // corregido
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<serde_json::Value>, // corregido
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: i32,
    pub limit: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: i32,
    pub quote_asset: String,
    pub quote_precision: i32,
    pub quote_asset_precision: i32,
    pub base_commission_precision: i32,
    pub quote_commission_precision: i32,
    pub order_types: Vec<String>,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub oto_allowed: bool,
    pub quote_order_qty_market_allowed: bool,
    pub allow_trailing_stop: bool,
    pub cancel_replace_allowed: bool,
    pub amend_allowed: bool,
    pub peg_instructions_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<Filter>,
    pub permissions: Vec<String>, // corregido
    pub permission_sets: Vec<Vec<String>>,
    pub default_self_trade_prevention_mode: String,
    pub allowed_self_trade_prevention_modes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub filter_type: String,
    pub min_price: Option<String>,
    pub max_price: Option<String>,
    pub tick_size: Option<String>,
    pub min_qty: Option<String>,
    pub max_qty: Option<String>,
    pub step_size: Option<String>,
    pub limit: Option<i32>,
    pub min_trailing_above_delta: Option<i32>,
    pub max_trailing_above_delta: Option<i32>,
    pub min_trailing_below_delta: Option<i32>,
    pub max_trailing_below_delta: Option<i32>,
    pub bid_multiplier_up: Option<String>,
    pub bid_multiplier_down: Option<String>,
    pub ask_multiplier_up: Option<String>,
    pub ask_multiplier_down: Option<String>,
    pub avg_price_mins: Option<i32>,
    pub min_notional: Option<String>,
    pub apply_min_to_market: Option<bool>,
    pub max_notional: Option<String>,
    pub apply_max_to_market: Option<bool>,
    pub max_num_orders: Option<i32>,
    pub max_num_order_lists: Option<i32>,
    pub max_num_algo_orders: Option<i32>,
    pub max_num_order_amends: Option<i32>,
    pub max_position: Option<String>,
}
