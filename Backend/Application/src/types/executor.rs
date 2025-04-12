use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    time::{Duration, Instant},
};

use chrono::DateTime;
use models::{
    entities::{self, assets, ledgers, orders, strategies},
    structs::{FilterType, Quote, Ticker},
};
use sea_orm::prelude::Decimal;
use tokio::time::sleep;

use crate::{integration::binance_websocket_client_task, types::PairAssets, utils::Core};

use super::{Assets, Ledgers, Orders, Solicitor, Strategies, StrategiesPairAssets, Tickers};

pub struct Executor<Phase = Core> {
    phase: PhantomData<Phase>,
}

impl Executor {
    pub async fn run_tasks() {
        tokio::spawn(async {
            loop {
                StrategiesPairAssets::reload_active_strategies_pair_assets().await;
                Orders::reload_stored_orders().await;
                Assets::reload_assets().await;
                sleep(Duration::from_secs(60)).await;
            }
        });

        tokio::spawn(async {
            loop {
                // Executor::update_assets_balance().await;
                Executor::update_pair_assets_statistics().await;
                sleep(Duration::from_secs(300)).await;
            }
        });

        tokio::spawn(async {
            loop {
                Executor::update_exchange_information().await;
                sleep(Duration::from_secs(86400)).await;
            }
        });

        tokio::spawn(async {
            let mut attempt = 0;
            loop {
                let strategies_pair_assets =
                    StrategiesPairAssets::get_active_strategies_pair_assets().await;

                if strategies_pair_assets.is_empty() {
                    attempt += 1;
                    let delay = Duration::from_secs(2_u64.pow(attempt.min(5))); // Max 32 seconds
                    println!(
                        "Strategies Pair Assets empty -- Reloading in {:?}... \n",
                        delay
                    );
                    sleep(delay).await;

                    continue;
                }

                let mut sockets: Vec<String> = Vec::new();

                for strategy_pair_asset in strategies_pair_assets {
                    let socket_pair = format!(
                        "{}@{}",
                        strategy_pair_asset.pair_asset.symbol.to_lowercase(),
                        strategy_pair_asset.strategy.stream_name.to_lowercase()
                    );

                    if !sockets.contains(&socket_pair) {
                        sockets.push(socket_pair);
                    }
                }

                binance_websocket_client_task(sockets).await.unwrap();

                break;
            }
        });

        tokio::spawn(async {
            loop {
                sleep(Duration::from_secs(1)).await;
                Executor::run_strategies().await;
            }
        });
    }

    pub async fn run_strategies() {
        // let start = Instant::now();

        for strategy_pair_asset in StrategiesPairAssets::get_active_strategies_pair_assets().await {
            let ticker = match Tickers::get_tickers(
                strategy_pair_asset.pair_asset.symbol.clone().to_uppercase(),
            )
            .await
            .first()
            {
                Some(val) => val.clone(),
                None => continue,
            };

            let base_asset = Assets::get_asset(strategy_pair_asset.pair_asset.base_asset_id).await;

            let quote_asset =
                Assets::get_asset(strategy_pair_asset.pair_asset.quote_asset_id).await;

            // get orders
            let stored_orders = Orders::get_stored_orders(
                strategy_pair_asset.pair_asset.base_asset_id,
                strategy_pair_asset.pair_asset.quote_asset_id,
            )
            .await;

            // check strategies using prices
            let mut new_orders = Strategies::<Core>::check_price_action_strategies(
                &strategy_pair_asset.strategy,
                &ticker,
                &stored_orders,
                &strategy_pair_asset.pair_asset,
                &quote_asset,
            );

            let last_price = ticker.last_price;

            let open_orders: Vec<orders::Model> = stored_orders
                .iter()
                .filter(|order| order.status_id == 1)
                .cloned()
                .collect();

            new_orders.append(&mut Strategies::<Core>::check_open_orders(
                open_orders,
                last_price,
            ));

            let reload_needed = !new_orders.is_empty();

            Executor::execute_immediate_orders(
                new_orders,
                base_asset,
                quote_asset,
                &strategy_pair_asset.strategy,
            )
            .await;

            if reload_needed {
                Orders::reload_stored_orders().await;
                Assets::reload_assets().await;
            }
        }

        // println!("{:?}", start.elapsed());
    }

    async fn execute_immediate_orders(
        orders: Vec<orders::Model>,
        mut base_asset: assets::Model,
        mut quote_asset: assets::Model,
        strategy: &strategies::Model,
    ) {
        let mut failed_orders = Vec::new();

        for mut order in orders.into_iter() {
            if failed_orders.contains(&order.creation_date) {
                continue;
            }

            if strategy.can_trade {
                let symbol = format!("{}{}", base_asset.ticker, quote_asset.ticker);

                if Solicitor::binance_post_new_order(symbol, &mut order)
                    .await
                    .is_err()
                {
                    failed_orders.push(order.creation_date);

                    continue;
                }
            }

            let order = match order.id {
                0 => Orders::<Core>::insert_order(order).await.unwrap(),
                _ => Orders::<Core>::update_order(order).await.unwrap(),
            };

            let base_asset_previous_balance = Decimal::from(base_asset.free);
            let quote_asset_previous_balance = Decimal::from(quote_asset.free);

            let (base_asset_new_balance, quote_asset_new_balance, record_type_id);

            if order.is_sell {
                // Selling base asset → Subtract from base, Add to quote
                base_asset_new_balance = base_asset.free - order.base_asset_amount;
                quote_asset_new_balance = quote_asset.free + order.quote_asset_amount;
                record_type_id = 2;
            } else {
                // Buying base asset → Add to base, Subtract from quote
                base_asset_new_balance = base_asset.free + order.base_asset_amount;
                quote_asset_new_balance = quote_asset.free - order.quote_asset_amount;
                record_type_id = 1
            }

            let ledger = entities::ledgers::Model {
                order_id: Some(order.id),
                base_asset_id: order.base_asset_id,
                base_asset_amount: order.base_asset_amount,
                base_asset_previous_balance,
                base_asset_new_balance,
                quote_asset_id: order.quote_asset_id,
                quote_asset_amount: order.quote_asset_amount,
                quote_asset_previous_balance,
                quote_asset_new_balance,
                record_type_id,
                ..Default::default()
            };

            Ledgers::<Core>::insert_ledger(ledger).await.unwrap();

            base_asset.free = base_asset_new_balance;
            base_asset = Assets::<Core>::update_asset(base_asset).await.unwrap();

            quote_asset.free = quote_asset_new_balance;
            quote_asset = Assets::<Core>::update_asset(quote_asset).await.unwrap();
        }
    }

    pub async fn update_assets_balance() {
        let start = Instant::now();

        let account_information = match Solicitor::binance_get_account_information().await {
            Err(_) => return,
            Ok(val) => val,
        };

        for balance in account_information.balances {
            let mut asset = match Assets::<Core>::select_asset(assets::Model {
                ticker: balance.asset,
                ..Default::default()
            })
            .await
            {
                Err(_) => continue,
                Ok(val) => val,
            };

            let balance_difference = balance.free - asset.free;

            if balance_difference == Decimal::ZERO {
                continue;
            }

            let ledger = ledgers::Model {
                record_type_id: 3,
                base_asset_id: asset.id,
                base_asset_amount: balance_difference,
                base_asset_previous_balance: asset.free,
                base_asset_new_balance: balance.free,
                quote_asset_id: asset.id,
                quote_asset_amount: balance_difference,
                quote_asset_previous_balance: asset.free,
                quote_asset_new_balance: balance.free,
                ..Default::default()
            };

            asset.free = balance.free;

            if let Err(_) = Assets::<Core>::update_asset(asset).await {
                continue;
            }

            if let Err(_) = Ledgers::<Core>::insert_ledger(ledger).await {
                continue;
            }
        }

        // Assets::reload_assets().await;

        println!("Update assets balance duration: {:?} \n", start.elapsed());
    }

    pub async fn update_pair_assets_statistics() {
        let start = Instant::now();

        let mut stored_pair_assets = match PairAssets::<Core>::select_all_pair_assets().await {
            Ok(pair_assets) if !pair_assets.is_empty() => pair_assets,
            _ => return,
        };

        let coinpaprika_tickers = match Solicitor::coinpaprika_get_tickers().await {
            Ok(tickers) if !tickers.is_empty() => tickers,
            _ => return,
        };

        let updates: Vec<(String, Quote)> = coinpaprika_tickers
            .into_iter()
            .flat_map(|ticker| {
                let ticker_symbol = ticker.symbol.to_uppercase();

                ticker
                    .quotes
                    .into_iter()
                    .map(move |(quote_key, quote_value)| {
                        let mapped_key = if quote_key == "USD" {
                            "USDT".to_string()
                        } else {
                            quote_key.to_uppercase()
                        };

                        let full_symbol = format!("{}{}", ticker_symbol, mapped_key);

                        (full_symbol, quote_value)
                    })
            })
            .collect();

        for pair_asset in stored_pair_assets.iter_mut() {
            if let Some((_, quote)) = updates
                .iter()
                .find(|(full_symbol, _)| full_symbol == &pair_asset.symbol)
            {
                // Update ATH-related fields.
                pair_asset.all_time_high_price =
                    Decimal::from_f64_retain(quote.ath_price).unwrap_or_default();

                pair_asset.all_time_high_date = DateTime::parse_from_rfc3339(&quote.ath_date)
                    .map(|dt| dt.naive_utc())
                    .unwrap_or_default();

                pair_asset.percent_from_all_time_high =
                    Decimal::from_f64_retain(quote.percent_from_price_ath).unwrap_or_default();

                // Update price percent change fields.
                pair_asset.fifteen_minutes_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_15_m).unwrap_or_default();

                pair_asset.thirty_minutes_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_30_m).unwrap_or_default();

                pair_asset.hour_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_1_h).unwrap_or_default();

                pair_asset.six_hours_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_6_h).unwrap_or_default();

                pair_asset.twelve_hours_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_12_h).unwrap_or_default();

                pair_asset.day_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_24_h).unwrap_or_default();

                pair_asset.week_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_7_d).unwrap_or_default();

                pair_asset.month_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_30_d).unwrap_or_default();

                pair_asset.year_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_1_y).unwrap_or_default();

                let _ = PairAssets::<Core>::update_pair_asset(pair_asset.clone()).await;
            }
        }

        println!(
            "Update pair assets statistics duration: {:?} \n",
            start.elapsed()
        );
    }

    pub async fn update_exchange_information() {
        let start = Instant::now();

        let mut stored_pair_assets = match PairAssets::<Core>::select_all_pair_assets().await {
            Ok(pair_assets) if !pair_assets.is_empty() => pair_assets,
            _ => return,
        };

        let mut updates = match Solicitor::binance_get_exchange_information().await {
            None => return,
            Some(val) => val,
        };

        for pair_asset in stored_pair_assets.iter_mut() {
            if let Some(exchange_pair_asset) = updates
                .symbols
                .iter_mut()
                .find(|asset| asset.symbol == pair_asset.symbol)
            {
                for filter in &exchange_pair_asset.filters {
                    match filter.filter_type {
                        FilterType::PriceFilter => {
                            if let Some(min_price) = filter.min_price {
                                pair_asset.price_filter_min_price = min_price;
                            }
                            if let Some(max_price) = filter.max_price {
                                pair_asset.price_filter_max_price = max_price;
                            }
                            if let Some(tick_size) = filter.tick_size {
                                pair_asset.price_filter_tick_size = tick_size;
                            }
                        }
                        FilterType::LotSize => {
                            if let Some(min_qty) = filter.min_qty {
                                pair_asset.lot_size_min_qty = min_qty;
                            }
                            if let Some(max_qty) = filter.max_qty {
                                pair_asset.lot_size_max_qty = max_qty;
                            }
                            if let Some(step_size) = filter.step_size {
                                pair_asset.lot_size_step_size = step_size;
                            }
                        }
                        FilterType::MarketLotSize => {
                            if let Some(min_qty) = filter.min_qty {
                                pair_asset.market_lot_size_min_qty = min_qty;
                            }
                            if let Some(max_qty) = filter.max_qty {
                                pair_asset.market_lot_size_max_qty = max_qty;
                            }
                            if let Some(step_size) = filter.step_size {
                                pair_asset.market_lot_size_step_size = step_size;
                            }
                        }
                        FilterType::TrailingDelta => {
                            if let Some(min_trailing_above_delta) = filter.min_trailing_above_delta
                            {
                                pair_asset.trailing_delta_min_trailing_above_delta =
                                    min_trailing_above_delta;
                            }
                            if let Some(max_trailing_above_delta) = filter.max_trailing_above_delta
                            {
                                pair_asset.trailing_delta_max_trailing_above_delta =
                                    max_trailing_above_delta;
                            }
                            if let Some(min_trailing_below_delta) = filter.min_trailing_below_delta
                            {
                                pair_asset.trailing_delta_min_trailing_below_delta =
                                    min_trailing_below_delta;
                            }
                            if let Some(max_trailing_below_delta) = filter.max_trailing_below_delta
                            {
                                pair_asset.trailing_delta_max_trailing_below_delta =
                                    max_trailing_below_delta;
                            }
                        }
                        FilterType::PercentPriceBySide => {
                            if let Some(bid_multiplier_up) = filter.bid_multiplier_up {
                                pair_asset.percent_price_by_side_bid_multiplier_up =
                                    bid_multiplier_up;
                            }
                            if let Some(bid_multiplier_down) = filter.bid_multiplier_down {
                                pair_asset.percent_price_by_side_bid_multiplier_down =
                                    bid_multiplier_down;
                            }
                            if let Some(ask_multiplier_up) = filter.ask_multiplier_up {
                                pair_asset.percent_price_by_side_ask_multiplier_up =
                                    ask_multiplier_up;
                            }
                            if let Some(ask_multiplier_down) = filter.ask_multiplier_down {
                                pair_asset.percent_price_by_side_ask_multiplier_down =
                                    ask_multiplier_down;
                            }
                            if let Some(avg_price_mins) = filter.avg_price_mins {
                                pair_asset.percent_price_by_side_avg_price_mins = avg_price_mins;
                            }
                        }
                        FilterType::Notional => {
                            if let Some(min_notional) = filter.min_notional {
                                pair_asset.notional_min_notional = min_notional;
                            }
                            if let Some(apply_min_to_market) = filter.apply_min_to_market {
                                pair_asset.notional_apply_min_to_market = apply_min_to_market;
                            }
                            if let Some(max_notional) = filter.max_notional {
                                pair_asset.notional_max_notional = max_notional;
                            }
                            if let Some(apply_max_to_market) = filter.apply_max_to_market {
                                pair_asset.notional_apply_max_to_market = apply_max_to_market;
                            }
                            if let Some(avg_price_mins) = filter.avg_price_mins {
                                pair_asset.notional_avg_price_mins = avg_price_mins;
                            }
                        }
                        FilterType::IcebergParts => {
                            if let Some(limit) = filter.limit {
                                pair_asset.iceberg_parts_limit = limit as i32;
                            }
                        }
                        FilterType::MaxNumOrders => {
                            if let Some(max_num_orders) = filter.max_num_orders {
                                pair_asset.max_num_orders = max_num_orders as i32;
                            }
                        }
                        FilterType::MaxNumAlgoOrders => {
                            if let Some(max_num_algo_orders) = filter.max_num_algo_orders {
                                pair_asset.max_num_algo_orders = max_num_algo_orders as i32;
                            }
                        }
                        _ => {}
                    }
                }
            }

            let _ = PairAssets::<Core>::update_pair_asset(pair_asset.clone()).await;
        }

        println!(
            "Update exchange information duration: {:?} \n",
            start.elapsed()
        );
    }

    pub async fn run_backtest(tickers: Vec<Ticker>) -> String {
        let start = Instant::now();
        println!("Running backtest \n");

        let mut result_string = String::new();
        let mut grouped: HashMap<i32, (Decimal, usize, usize, usize, Decimal, Decimal)> =
            HashMap::new();
        let mut last_ledgers: Vec<(ledgers::Model, i32)> = Vec::new();

        for strategy_pair_asset in StrategiesPairAssets::get_active_strategies_pair_assets().await {
            println!("Strategy: {} \n", strategy_pair_asset.strategy.id);

            let mut open_orders: Vec<orders::Model> = Vec::new();
            let mut all_orders: Vec<orders::Model> = Vec::new();
            let mut ledgers = Vec::new();

            let mut base_asset = assets::Model {
                free: Decimal::ZERO,
                ..Default::default()
            };

            let mut quote_asset = assets::Model {
                free: Decimal::from(1000),
                ..Default::default()
            };

            let mut order_id = 0;

            for ticker in tickers.iter() {
                // check strategies using prices
                let new_orders = Strategies::<Core>::check_price_action_strategies(
                    &strategy_pair_asset.strategy,
                    &ticker,
                    &open_orders,
                    &strategy_pair_asset.pair_asset,
                    &quote_asset,
                );

                let mut limit_orders: Vec<orders::Model> = Vec::new();
                let mut market_orders: Vec<orders::Model> = Vec::new();

                new_orders.into_iter().for_each(|mut order| {
                    order_id += 1;
                    order.id = order_id;

                    if order.status_id == 1 {
                        open_orders.push(order.clone());
                        limit_orders.push(order);
                    } else {
                        all_orders.push(order.clone());
                        market_orders.push(order);
                    }
                });

                let last_price = ticker.last_price;

                market_orders.append(&mut Strategies::<Core>::check_open_orders(
                    open_orders.clone(),
                    last_price,
                ));

                if !market_orders.is_empty() {
                    let order_ids: HashSet<i32> =
                        market_orders.iter().map(|order| order.id).collect();

                    open_orders.retain(|order| {
                        if order_ids.contains(&order.id) {
                            let updated_order = market_orders
                                .iter()
                                .find(|updated_order| updated_order.id == order.id)
                                .cloned()
                                .unwrap_or_default();

                            all_orders.push(updated_order);
                            false
                        } else {
                            true
                        }
                    });
                }

                Executor::execute_immediate_orders_backtest(
                    market_orders,
                    &mut ledgers,
                    &mut base_asset,
                    &mut quote_asset,
                )
                .await;
            }

            for order in all_orders {
                let entry = grouped.entry(order.strategy_id).or_insert((
                    Decimal::ZERO,
                    0,
                    0,
                    0,
                    Decimal::ZERO,
                    Decimal::ZERO,
                ));

                entry.0 += if order.is_sell {
                    order.quote_asset_amount
                } else {
                    -order.quote_asset_amount
                };

                entry.1 += 1;

                if order.status_id == 3 {
                    entry.3 += 1;
                }
            }

            for order in open_orders {
                let entry = grouped.entry(order.strategy_id).or_insert((
                    Decimal::ZERO,
                    0,
                    0,
                    0,
                    Decimal::ZERO,
                    Decimal::ZERO,
                ));

                entry.0 += if order.is_sell {
                    order.quote_asset_amount
                } else {
                    -order.quote_asset_amount
                };

                entry.2 += 1;
            }

            let entry = grouped.entry(strategy_pair_asset.strategy.id).or_insert((
                Decimal::ZERO,
                0,
                0,
                0,
                Decimal::ZERO,
                Decimal::ZERO,
            ));

            entry.4 = quote_asset.free;
            entry.5 = base_asset.free;

            if let Some(last_ledger) = ledgers.iter().last() {
                last_ledgers.push((last_ledger.clone(), strategy_pair_asset.strategy.id));
            }
        }

        for (strategy_id, (sum, proccesed, open, aborted, quote_free, base_free)) in grouped {
            result_string.push_str(&format!(
                "Strategy {}: Sum = {}, Processed orders count = {}, Open orders count = {}, Aborted orders count = {}, Quote asset free: {}, Base asset free: {} \n",
                strategy_id, sum, proccesed, open, aborted, quote_free, base_free
            ));

            if let Some(last_ledger) = last_ledgers.iter().find(|ledger| ledger.1 == strategy_id) {
                result_string.push_str(&format!("Last ledger: {:?} \n", last_ledger.0));
            }
        }

        let duration = start.elapsed().as_secs();
        println!("Processed in {duration:?} secs \n");

        result_string
    }

    async fn execute_immediate_orders_backtest(
        orders: Vec<orders::Model>,
        ledgers: &mut Vec<ledgers::Model>,
        base_asset: &mut assets::Model,
        quote_asset: &mut assets::Model,
    ) {
        for order in orders.into_iter() {
            let base_asset_previous_balance = base_asset.free;
            let quote_asset_previous_balance = quote_asset.free;

            let (base_asset_new_balance, quote_asset_new_balance, record_type_id);

            if order.is_sell {
                // Selling base asset → Subtract from base, Add to quote
                base_asset_new_balance = base_asset.free - order.base_asset_amount;
                quote_asset_new_balance = quote_asset.free + order.quote_asset_amount;
                record_type_id = 2;
            } else {
                // Buying base asset → Add to base, Subtract from quote
                base_asset_new_balance = base_asset.free + order.base_asset_amount;
                quote_asset_new_balance = quote_asset.free - order.quote_asset_amount;
                record_type_id = 1
            }

            let ledger = entities::ledgers::Model {
                order_id: Some(order.id),
                base_asset_id: order.base_asset_id,
                base_asset_amount: order.base_asset_amount,
                base_asset_previous_balance,
                base_asset_new_balance,
                quote_asset_id: order.quote_asset_id,
                quote_asset_amount: order.quote_asset_amount,
                quote_asset_previous_balance,
                quote_asset_new_balance,
                record_type_id,
                ..Default::default()
            };

            ledgers.push(ledger);

            base_asset.free = base_asset_new_balance;

            quote_asset.free = quote_asset_new_balance;
        }
    }
}
