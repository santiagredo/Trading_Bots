use crate::{
    handler::{Assets, Integrations, IntegrationsSettings, Ledgers, Pairs},
    utils::{EntityCache, RepoFactory, Update},
};
use chrono::Local;
use models::{
    entities::{
        assets, ledgers,
        pairs::{self, Model},
    },
    structs::{AssetRequest, Environments, LedgerRequest, PairRequest},
};
use sea_orm::prelude::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Binance<R> {
    pub repo: R,
}

impl<R> Binance<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl Binance<()> {
    pub fn blank() -> Binance<()> {
        Self { repo: () }
    }

    pub async fn update_account_balances(factory: RepoFactory, environment: Environments) {
        let Some(binance_integration) =
            Integrations::blank()
                .get_all(environment)
                .await
                .and_then(|cache| {
                    cache
                        .models
                        .into_iter()
                        .find(|(_, i)| i.code == "BINANCE")
                        .map(|(_, i)| i)
                })
        else {
            return;
        };

        let Some(cache) = IntegrationsSettings::blank().get_all(environment).await else {
            return;
        };

        let Some(binance_map) = cache.integrations_map.get(&binance_integration.id) else {
            return;
        };

        let integration_settings = binance_map.models.values().cloned().collect::<Vec<_>>();

        let Some(api_key) = integration_settings
            .iter()
            .find(|val| val.nick == "api_key")
            .to_owned()
        else {
            return;
        };

        let Some(secret_pass) = integration_settings
            .iter()
            .find(|val| val.nick == "secret_pass")
            .to_owned()
        else {
            return;
        };

        let Ok(account) = Binance::blank()
            .get_account(
                api_key.value.clone(),
                secret_pass.value.clone(),
                factory.clone(),
            )
            .await
        else {
            return;
        };

        let assets = Assets::blank()
            .get_all(environment)
            .await
            .unwrap_or_default();

        let assets_repo = factory.repo::<AssetRequest, assets::Model>();
        let ledgers_repo = factory.repo::<LedgerRequest, ledgers::Model>();

        for balance in account.balances {
            let Some((_, asset)) = assets
                .models
                .iter()
                .find(|(_, val)| val.ticker.to_uppercase() == balance.asset.to_uppercase())
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
                name: Some(asset.name.clone()),
                ticker: Some(asset.ticker.clone()),
                last_update: Some(Local::now().naive_local()),
            };

            let asset_model = Assets::into_model(asset_request.clone());

            if let Err(err) = Assets::blank()
                .upsert(environment, asset_model.id, asset_model)
                .await
            {
                dbg!(err);
                continue;
            };

            let stored_model = match Assets::new(assets_repo.clone()).update(asset_request).await {
                Err(err) => {
                    dbg!("{}", err.message);
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

            if let Err(err) = Ledgers::new(ledgers_repo.clone())
                .insert(ledger_request)
                .await
            {
                dbg!(eprint!("{}", err.message));
            };
        }
    }

    pub async fn update_exchange_information(factory: RepoFactory, environment: Environments) {
        let mut stored_pairs = match Pairs::blank().get_all(environment).await {
            None => return,
            Some(val) => val
                .models
                .into_iter()
                .map(|(_, model)| model)
                .collect::<Vec<Model>>(),
        };

        let mut updates = match Binance::blank()
            .get_exchange_information(factory.clone())
            .await
        {
            Err(err) => {
                dbg!(err);
                return;
            }
            Ok(val) => val,
        };

        let pairs_repo = factory.repo::<PairRequest, pairs::Model>();

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

            let mut pair_request = Pairs::into_request(pair.clone());

            // Prevent update from overwritting existing values
            pair_request.all_time_high_date = None;
            pair_request.all_time_high_price = None;
            pair_request.percent_from_all_time_high = None;
            pair_request.fifteen_minutes_price_percent_change = None;
            pair_request.thirty_minutes_price_percent_change = None;
            pair_request.hour_price_percent_change = None;
            pair_request.six_hours_price_percent_change = None;
            pair_request.twelve_hours_price_percent_change = None;
            pair_request.day_price_percent_change = None;
            pair_request.week_price_percent_change = None;
            pair_request.month_price_percent_change = None;
            pair_request.year_price_percent_change = None;

            let model = match pairs_repo.clone().update(pair_request).await {
                Err(err) => {
                    dbg!(err);
                    continue;
                }
                Ok(val) => val,
            };

            if let Err(err) = Pairs::blank().upsert(environment, model.id, model).await {
                dbg!(err);
            }
        }
    }
}
