use std::marker::PhantomData;

use models::{
    entities::orders,
    structs::{AccountInformation, AssetRequest, ExchangeInformation, LedgerRequest},
};
use sea_orm::prelude::Decimal;
use std::str::FromStr;

use crate::{
    types::{Assets, Ledgers, Pairs},
    utils::Types,
};

pub struct Binance<Phase = Types> {
    phase: PhantomData<Phase>,
}

impl Binance {
    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
        }
    }

    pub async fn update_account_balances() {
        let account = Self::default().get_account().await.unwrap_or_default();

        let assets = Assets::default().select_assets().await.unwrap_or_default();

        for balance in account.balances {
            let Some(asset) = assets
                .iter()
                .find(|val| val.ticker.to_uppercase() == balance.asset.to_uppercase())
            else {
                continue;
            };

            if balance.free == asset.free && balance.locked == asset.locked {
                continue;
            }

            let asset_request = AssetRequest {
                id: Some(asset.id),
                free: Some(balance.free),
                locked: Some(balance.locked),
                ..Default::default()
            };

            let stored_model = match Assets::new(asset_request).update_asset().await {
                Err(err) => {
                    eprint!("{}", err.message);
                    continue;
                }
                Ok(val) => val,
            };

            let ledger_request = LedgerRequest {
                id: None,
                order_id: None,
                record_type_id: Some(3),
                creation_date: None,
                asset_id: Some(stored_model.id),
                free_amount: Some(balance.free),
                free_previous_balance: Some(asset.free),
                free_new_balance: Some(balance.free),
                locked_amount: Some(balance.locked),
                locked_previous_balance: Some(asset.locked),
                locked_new_balance: Some(balance.locked),
            };

            if let Err(err) = Ledgers::new(ledger_request).insert_ledger().await {
                eprint!("{}", err.message);
            };
        }
    }

    pub async fn update_exchange_information() {
        // let start = Instant::now();

        let mut stored_pairs = match Pairs::default().select_pairs().await {
            Ok(pairs) if !pairs.is_empty() => pairs,
            _ => return,
        };

        let mut updates = match Self::default().get_exchange_information().await {
            None => return,
            Some(val) => val,
        };

        for pair in stored_pairs.iter_mut() {
            if let Some(exchange_pair) = updates
                .symbols
                .iter_mut()
                .find(|asset| asset.symbol.to_uppercase() == pair.symbol.to_uppercase())
            {
                for filter in &exchange_pair.filters {
                    match filter.filter_type.as_str() {
                        "PRICE_FILTER" => {
                            if let Some(min_price) = &filter.min_price {
                                pair.price_filter_min_price =
                                    Decimal::from_str(min_price.as_str()).unwrap_or_default();
                            }
                            if let Some(max_price) = &filter.max_price {
                                pair.price_filter_max_price =
                                    Decimal::from_str(max_price.as_str()).unwrap_or_default();
                            }
                            if let Some(tick_size) = &filter.tick_size {
                                pair.price_filter_tick_size =
                                    Decimal::from_str(tick_size.as_str()).unwrap_or_default();
                            }
                        }
                        "LOT_SIZE" => {
                            if let Some(min_qty) = &filter.min_qty {
                                pair.lot_size_min_qty =
                                    Decimal::from_str(min_qty.as_str()).unwrap_or_default();
                            }
                            if let Some(max_qty) = &filter.max_qty {
                                pair.lot_size_max_qty =
                                    Decimal::from_str(max_qty.as_str()).unwrap_or_default();
                            }
                            if let Some(step_size) = &filter.step_size {
                                pair.lot_size_step_size =
                                    Decimal::from_str(step_size.as_str()).unwrap_or_default();
                            }
                        }
                        "MARKET_LOT_SIZE" => {
                            if let Some(min_qty) = &filter.min_qty {
                                pair.market_lot_size_min_qty =
                                    Decimal::from_str(min_qty.as_str()).unwrap_or_default();
                            }
                            if let Some(max_qty) = &filter.max_qty {
                                pair.market_lot_size_max_qty =
                                    Decimal::from_str(max_qty.as_str()).unwrap_or_default();
                            }
                            if let Some(step_size) = &filter.step_size {
                                pair.market_lot_size_step_size =
                                    Decimal::from_str(step_size.as_str()).unwrap_or_default();
                            }
                        }
                        "TRAILING_DELTA" => {
                            if let Some(min_trailing_above_delta) = filter.min_trailing_above_delta
                            {
                                pair.trailing_delta_min_trailing_above_delta =
                                    min_trailing_above_delta;
                            }
                            if let Some(max_trailing_above_delta) = filter.max_trailing_above_delta
                            {
                                pair.trailing_delta_max_trailing_above_delta =
                                    max_trailing_above_delta;
                            }
                            if let Some(min_trailing_below_delta) = filter.min_trailing_below_delta
                            {
                                pair.trailing_delta_min_trailing_below_delta =
                                    min_trailing_below_delta;
                            }
                            if let Some(max_trailing_below_delta) = filter.max_trailing_below_delta
                            {
                                pair.trailing_delta_max_trailing_below_delta =
                                    max_trailing_below_delta;
                            }
                        }
                        "PERCENT_PRICE_BY_SIDE" => {
                            if let Some(bid_multiplier_up) = &filter.bid_multiplier_up {
                                pair.percent_price_by_side_bid_multiplier_up =
                                    Decimal::from_str(bid_multiplier_up.as_str())
                                        .unwrap_or_default();
                            }
                            if let Some(bid_multiplier_down) = &filter.bid_multiplier_down {
                                pair.percent_price_by_side_bid_multiplier_down =
                                    Decimal::from_str(bid_multiplier_down.as_str())
                                        .unwrap_or_default();
                            }
                            if let Some(ask_multiplier_up) = &filter.ask_multiplier_up {
                                pair.percent_price_by_side_ask_multiplier_up =
                                    Decimal::from_str(ask_multiplier_up.as_str())
                                        .unwrap_or_default();
                            }
                            if let Some(ask_multiplier_down) = &filter.ask_multiplier_down {
                                pair.percent_price_by_side_ask_multiplier_down =
                                    Decimal::from_str(ask_multiplier_down.as_str())
                                        .unwrap_or_default();
                            }
                            if let Some(avg_price_mins) = filter.avg_price_mins {
                                pair.percent_price_by_side_avg_price_mins = avg_price_mins;
                            }
                        }
                        "NOTIONAL" => {
                            if let Some(min_notional) = &filter.min_notional {
                                pair.notional_min_notional =
                                    Decimal::from_str(min_notional.as_str()).unwrap_or_default();
                            }
                            if let Some(apply_min_to_market) = filter.apply_min_to_market {
                                pair.notional_apply_min_to_market = apply_min_to_market;
                            }
                            if let Some(max_notional) = &filter.max_notional {
                                pair.notional_max_notional =
                                    Decimal::from_str(max_notional.as_str()).unwrap_or_default();
                            }
                            if let Some(apply_max_to_market) = filter.apply_max_to_market {
                                pair.notional_apply_max_to_market = apply_max_to_market;
                            }
                            if let Some(avg_price_mins) = filter.avg_price_mins {
                                pair.notional_avg_price_mins = avg_price_mins;
                            }
                        }
                        "ICEBERG_PARTS" => {
                            if let Some(limit) = filter.limit {
                                pair.iceberg_parts_limit = limit as i32;
                            }
                        }
                        "MAX_NUM_ORDERS" => {
                            if let Some(max_num_orders) = filter.max_num_orders {
                                pair.max_num_orders = max_num_orders as i32;
                            }
                        }
                        // "MAX_NUM_ORDER_LISTS" => {
                        //     if let Some(max_num_order_lists) = filter.max_num_order_lists {
                        //         pair.max_num_order_lists = max_num_order_lists as i32;
                        //     }
                        // }
                        "MAX_NUM_ALGO_ORDERS" => {
                            if let Some(max_num_algo_orders) = filter.max_num_algo_orders {
                                pair.max_num_algo_orders = max_num_algo_orders as i32;
                            }
                        }
                        // "MAX_NUM_ORDER_AMENDS" => {
                        //     if let Some(max_num_order_amends) = filter.max_num_order_amends {
                        //         pair.max_num_order_amends = max_num_order_amends as i32;
                        //     }
                        // }
                        // "MAX_POSITION" => {
                        //     if let Some(max_position) = &filter.max_position {
                        //         pair.max_position =
                        //             Decimal::from_str(max_position.as_str()).unwrap_or_default();
                        //     }
                        // }
                        _ => (),
                    }
                }
            }

            let _ = Pairs::default()
                .from_model(pair.clone())
                .update_pair()
                .await;
        }

        // dbg!(format!(
        //     "Update exchange information duration: {:?}",
        //     start.elapsed()
        // ));
    }
}

impl<Phase> Binance<Phase> {
    pub fn next_phase<Next>(self) -> Binance<Next> {
        Binance {
            phase: PhantomData::<Next>,
        }
    }
}

impl Binance<Types> {
    pub async fn get_account(self) -> Option<AccountInformation> {
        self.next_phase().get_account_core().await
    }

    pub async fn get_exchange_information(self) -> Option<ExchangeInformation> {
        self.next_phase().get_exchange_information_core().await
    }

    pub async fn post_new_order(self, symbol: String, order: &mut orders::Model) -> Result<(), ()> {
        self.next_phase().post_new_order_core(symbol, order).await
    }
}
