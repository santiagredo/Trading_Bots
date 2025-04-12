use std::marker::PhantomData;

use chrono::{Duration, Local};
use models::{
    entities::{
        assets, orders, pair_assets,
        strategies::{self, Model},
    },
    structs::Ticker,
};
use sea_orm::prelude::Decimal;

use crate::{
    types::Strategies,
    utils::{Data, Logic, Utils},
};

impl Strategies<Logic> {
    pub fn insert_strategy(mut strategy: Model) -> Result<Strategies<Data>, String> {
        strategy.name = Utils::validate_empty_field(strategy.name.clone(), "Strategy name")?;
        strategy.stream_name =
            Utils::validate_empty_field(strategy.stream_name.clone(), "Stream name")?;

        Ok(Strategies {
            phase: PhantomData::<Data>,
            model: strategy,
        })
    }

    pub fn select_strategy(id: i32) -> Result<(), String> {
        if id <= 0 {
            return Err(format!("Invalid strategy ID"));
        }

        Ok(())
    }

    pub fn select_active_strategies() -> Result<(), ()> {
        Ok(())
    }

    pub fn update_strategy(strategy: Model) -> Result<Strategies<Data>, String> {
        Utils::validate_empty_field(strategy.stream_name.clone(), "Stream name")?;

        Ok(Strategies {
            phase: PhantomData::<Data>,
            model: strategy,
        })
    }

    pub fn delete_strategy(strategy: Model) -> Result<Strategies<Data>, String> {
        if strategy.id <= 0 {
            return Err(format!("Invalid strategy ID"));
        }

        Ok(Strategies {
            phase: PhantomData::<Data>,
            model: strategy,
        })
    }

    pub fn check_price_action_strategies(
        strategy: &strategies::Model,
        ticker: &Ticker,
        stored_orders: &Vec<orders::Model>,
        pair_asset: &pair_assets::Model,
        quote_asset: &assets::Model,
    ) -> Vec<orders::Model> {
        // Decimal == coefficient / 10 ^ scale
        let mut new_orders: Vec<orders::Model> = Vec::new();

        let mut purchase_amount = Decimal::ZERO;
        let step_qty = pair_asset.lot_size_step_size * ticker.last_price;

        while purchase_amount < pair_asset.notional_min_notional {
            purchase_amount += step_qty;
        }

        if !pair_asset.all_time_high_price.is_zero() {
            // Gets the positive int from ath change to current price
            let mut ath_percentage_change = Decimal::ONE_HUNDRED
                - ((ticker.last_price / pair_asset.all_time_high_price) * Decimal::ONE_HUNDRED);

            // Divides positive int change by ten to get a small multiplier, e.g: 20 ath_percentage_change / 10 = 2
            ath_percentage_change = (ath_percentage_change / Decimal::TEN).floor();

            purchase_amount += step_qty * ath_percentage_change;
        }

        // dbg!(purchase_amount);

        if purchase_amount > quote_asset.free || purchase_amount == Decimal::ZERO {
            return new_orders;
        }

        match strategy.name.as_ref() {
            "LTE85%" => {
                let recent_orders_count = stored_orders
                    .iter()
                    .filter(|order| {
                        order.strategy_id == strategy.id
                            && order.base_asset_id == pair_asset.base_asset_id
                            && order.quote_asset_id == pair_asset.quote_asset_id
                            && order.status_id == 2
                            && order.creation_date
                                >= Local::now()
                                    .naive_local()
                                    // .checked_sub_signed(Duration::minutes(1))
                                    .checked_sub_signed(Duration::hours(8))
                                    .unwrap()
                    })
                    .count();

                if recent_orders_count > 0
                    || pair_asset.percent_from_all_time_high > Decimal::from(-15)
                {
                    // dbg!(pair_asset.percent_from_all_time_high);
                    return new_orders;
                }

                let now = Local::now().naive_local();

                let new_buy_order: orders::Model = orders::Model {
                    status_id: 2,
                    is_sell: false,
                    strategy_id: strategy.id,
                    base_asset_id: pair_asset.base_asset_id,
                    // base_asset_amount: purchase_amount / ticker.last_price,
                    quote_asset_id: pair_asset.quote_asset_id,
                    quote_asset_amount: purchase_amount,
                    price_entry: ticker.last_price,
                    price_target: ticker.last_price,
                    price_abort: Decimal::ZERO,
                    creation_date: now,
                    ..Default::default()
                };

                new_orders.push(new_buy_order);

                // let base_asset_amount = ((purchase_amount / ticker.last_price)
                //     * Decimal::from(100000))
                //     / Decimal::from(100000);

                // let new_sell_order: orders::Model = orders::Model {
                //     status_id: 1,
                //     is_sell: true,
                //     strategy_id: strategy.id,
                //     base_asset_id: pair_asset.base_asset_id,
                //     base_asset_amount,
                //     quote_asset_id: pair_asset.quote_asset_id,
                //     quote_asset_amount: base_asset_amount * ticker.last_price,
                //     price_entry: ticker.last_price,
                //     price_target: Decimal::from(100000),
                //     price_abort: Decimal::ZERO,
                //     creation_date: now,
                //     ..Default::default()
                // };

                // new_orders.push(new_sell_order);
            }
            "WAP" => {
                let (weighted_avg_price, last_price) =
                    (ticker.weighted_avg_price, ticker.last_price);

                let percentil_change =
                    Decimal::from(100) - ((weighted_avg_price / last_price) * Decimal::from(100));

                let recent_orders_count = stored_orders
                    .iter()
                    .filter(|order| {
                        order.strategy_id == strategy.id
                            && order.base_asset_id == pair_asset.base_asset_id
                            && order.quote_asset_id == pair_asset.quote_asset_id
                            && order.status_id == 1
                            && order.creation_date
                                >= Local::now()
                                    .naive_local()
                                    .checked_sub_signed(Duration::minutes(5))
                                    .unwrap()
                    })
                    .count();

                let price_action_valid = percentil_change <= Decimal::new(-5, 0);

                if recent_orders_count == 0 && price_action_valid {
                    let now = Local::now().naive_local();

                    let new_buy_order: orders::Model = orders::Model {
                        status_id: 2,
                        is_sell: false,
                        strategy_id: strategy.id,
                        base_asset_id: pair_asset.base_asset_id,
                        base_asset_amount: purchase_amount / ticker.last_price,
                        quote_asset_id: pair_asset.quote_asset_id,
                        quote_asset_amount: purchase_amount,
                        price_entry: last_price,
                        price_target: last_price,
                        price_abort: Decimal::ZERO,
                        creation_date: now,
                        ..Default::default()
                    };

                    new_orders.push(new_buy_order);

                    let base_asset_amount =
                        ((purchase_amount / last_price) * weighted_avg_price) / weighted_avg_price;

                    let new_sell_order: orders::Model = orders::Model {
                        status_id: 1,
                        is_sell: true,
                        strategy_id: strategy.id,
                        base_asset_id: pair_asset.base_asset_id,
                        base_asset_amount,
                        quote_asset_id: pair_asset.quote_asset_id,
                        quote_asset_amount: base_asset_amount * last_price,
                        price_entry: last_price,
                        price_target: weighted_avg_price,
                        price_abort: Decimal::ZERO,
                        creation_date: now,
                        ..Default::default()
                    };

                    new_orders.push(new_sell_order);
                }
            }
            "HGH" => {
                let (high_price, last_price) = (ticker.high_price, ticker.last_price);

                let percentil_change =
                    Decimal::from(100) - ((high_price / last_price) * Decimal::from(100));

                let recent_orders_count = stored_orders
                    .iter()
                    .filter(|order| {
                        order.strategy_id == strategy.id
                            && order.base_asset_id == pair_asset.base_asset_id
                            && order.quote_asset_id == pair_asset.quote_asset_id
                            // && order.price_target == ticker.high_price
                            && order.status_id == 1
                            && order.creation_date
                                >= Local::now()
                                    .naive_local()
                                    .checked_sub_signed(Duration::minutes(5))
                                    .unwrap()
                    })
                    .count();

                let price_action_valid = percentil_change <= Decimal::new(-5, 0);

                if recent_orders_count == 0 && price_action_valid {
                    let now = Local::now().naive_local();

                    let new_buy_order: orders::Model = orders::Model {
                        status_id: 2,
                        is_sell: false,
                        strategy_id: strategy.id,
                        base_asset_id: pair_asset.base_asset_id,
                        base_asset_amount: purchase_amount / ticker.last_price,
                        quote_asset_id: pair_asset.quote_asset_id,
                        quote_asset_amount: purchase_amount,
                        price_entry: last_price,
                        price_target: last_price,
                        price_abort: Decimal::ZERO,
                        creation_date: now,
                        ..Default::default()
                    };

                    new_orders.push(new_buy_order);

                    let base_asset_amount =
                        ((purchase_amount / last_price) * high_price) / high_price;

                    let new_sell_order: orders::Model = orders::Model {
                        status_id: 1,
                        is_sell: true,
                        strategy_id: strategy.id,
                        base_asset_id: pair_asset.base_asset_id,
                        base_asset_amount,
                        quote_asset_id: pair_asset.quote_asset_id,
                        quote_asset_amount: base_asset_amount * last_price,
                        price_entry: last_price,
                        price_target: high_price,
                        price_abort: Decimal::ZERO,
                        creation_date: now,
                        ..Default::default()
                    };

                    new_orders.push(new_sell_order);
                }
            }
            "OPN" => {
                let (open_price, last_price) = (ticker.open_price, ticker.last_price);

                let percentil_change =
                    ((last_price * Decimal::from(100)) / open_price) - Decimal::from(100);

                let recent_orders_count = stored_orders
                    .iter()
                    .filter(|order| {
                        order.strategy_id == strategy.id
                            && order.base_asset_id == pair_asset.base_asset_id
                            && order.quote_asset_id == pair_asset.quote_asset_id
                            && order.status_id == 1
                            && order.creation_date
                                >= Local::now()
                                    .naive_local()
                                    .checked_sub_signed(Duration::minutes(5))
                                    .unwrap()
                    })
                    .count();

                let price_action_valid = percentil_change <= Decimal::new(-5, 0);

                if recent_orders_count == 0 && price_action_valid {
                    let now = Local::now().naive_local();

                    let new_buy_order: orders::Model = orders::Model {
                        status_id: 2,
                        is_sell: false,
                        strategy_id: strategy.id,
                        base_asset_id: pair_asset.base_asset_id,
                        base_asset_amount: purchase_amount / ticker.last_price,
                        quote_asset_id: pair_asset.quote_asset_id,
                        quote_asset_amount: purchase_amount,
                        price_entry: last_price,
                        price_target: last_price,
                        price_abort: Decimal::ZERO,
                        creation_date: now,
                        ..Default::default()
                    };

                    new_orders.push(new_buy_order);

                    let base_asset_amount =
                        ((purchase_amount / last_price) * open_price) / open_price;

                    let new_sell_order: orders::Model = orders::Model {
                        status_id: 1,
                        is_sell: true,
                        strategy_id: strategy.id,
                        base_asset_id: pair_asset.base_asset_id,
                        base_asset_amount,
                        quote_asset_id: pair_asset.quote_asset_id,
                        quote_asset_amount: base_asset_amount * last_price,
                        price_entry: last_price,
                        price_target: open_price,
                        price_abort: Decimal::ZERO,
                        creation_date: now,
                        // price_abort: open_price * Decimal::new(970, 3),
                        ..Default::default()
                    };

                    new_orders.push(new_sell_order);
                }
            }
            _ => (),
        }

        new_orders
    }

    pub fn check_open_orders(
        open_orders: Vec<orders::Model>,
        last_price: Decimal,
    ) -> Vec<orders::Model> {
        let mut exec_orders: Vec<orders::Model> = Vec::new();

        for mut order in open_orders {
            match order.is_sell {
                false => {
                    if last_price <= order.price_target {
                        order.status_id = 2;
                        order.base_asset_amount = order.quote_asset_amount / order.price_target;
                        exec_orders.push(order);
                        continue;
                    }

                    if last_price >= order.price_abort && order.price_abort != Decimal::ZERO {
                        order.status_id = 3;
                        order.base_asset_amount = order.quote_asset_amount / order.price_target;
                        exec_orders.push(order);
                    }
                }
                true => {
                    if last_price >= order.price_target {
                        order.status_id = 2;
                        order.quote_asset_amount = order.base_asset_amount * last_price;
                        exec_orders.push(order);
                        continue;
                    }

                    if last_price <= order.price_abort && order.price_abort != Decimal::ZERO {
                        order.status_id = 3;
                        order.quote_asset_amount = order.base_asset_amount * last_price;
                        exec_orders.push(order);
                    }
                }
            }
        }

        exec_orders
    }
}
