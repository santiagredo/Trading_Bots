// use models::entities;
// use sea_orm::prelude::Decimal;

// use crate::{
//     types::{reload_open_orders, Assets, Executor, Ledgers, Orders, Pairs, Strategies},
//     utils::Core,
// };

// impl Executor<Core> {
//     pub async fn check_open_orders_strategies(
//         open_orders: Vec<entities::orders::Model>,
//         s: String,
//         c: Decimal,
//         o: Decimal,
//         h: Decimal,
//         l: Decimal,
//     ) {
//         let pair = Pairs::<Core>::select_pair(entities::pairs::Model {
//             pair_ticker: s.to_ascii_uppercase(),
//             ..Default::default()
//         })
//         .await
//         .unwrap();

//         let assets = Assets::<Core>::select_assets().await.unwrap();

//         let mut base_asset = assets
//             .iter()
//             .find(|item| item.id == pair.base_asset_id)
//             .unwrap()
//             .to_owned();

//         let mut quote_asset = assets
//             .iter()
//             .find(|item| item.id == pair.quote_asset_id)
//             .unwrap()
//             .to_owned();

//         let open_orders = open_orders
//             .into_iter()
//             .filter(|item| {
//                 item.base_asset_id == pair.base_asset_id
//                     && item.quote_asset_id == pair.quote_asset_id
//             })
//             .collect();

//         let mut reload_needed = false;

//         let mut exec_orders =
//             Strategies::<Core>::check_open_orders_strategies(open_orders, c, o, h, l);

//         if !exec_orders.is_empty() {
//             reload_needed = true;
//         }

//         for order in exec_orders.iter_mut() {
//             // A trade in Binance should take place here
//             // Assume that the trade in Binance is successful

//             order.status_id = 2;
//             let order = Orders::<Core>::update_order(order.clone()).await.unwrap();

//             let base_asset_previous_balance = Decimal::from(base_asset.balance);
//             let quote_asset_previous_balance = Decimal::from(quote_asset.balance);

//             let (base_asset_new_balance, quote_asset_new_balance, record_type_id);

//             if order.is_sell {
//                 // Selling base asset → Subtract from base, Add to quote
//                 base_asset_new_balance = base_asset.balance - order.base_asset_amount;
//                 quote_asset_new_balance = quote_asset.balance + order.quote_asset_amount;
//                 record_type_id = 2;
//             } else {
//                 // Buying base asset → Add to base, Subtract from quote
//                 base_asset_new_balance = base_asset.balance + order.base_asset_amount;
//                 quote_asset_new_balance = quote_asset.balance - order.quote_asset_amount;
//                 record_type_id = 1
//             }

//             let ledger = entities::ledgers::Model {
//                 order_id: order.id,
//                 base_asset_id: order.base_asset_id,
//                 base_asset_amount: order.base_asset_amount,
//                 base_asset_previous_balance,
//                 base_asset_new_balance,
//                 quote_asset_id: order.quote_asset_id,
//                 quote_asset_amount: order.quote_asset_amount,
//                 quote_asset_previous_balance,
//                 quote_asset_new_balance,
//                 record_type_id,
//                 ..Default::default()
//             };

//             Ledgers::<Core>::insert_ledger(ledger).await.unwrap();

//             base_asset.balance = base_asset_new_balance;
//             base_asset = Assets::<Core>::update_asset(base_asset).await.unwrap();

//             quote_asset.balance = quote_asset_new_balance;
//             quote_asset = Assets::<Core>::update_asset(quote_asset).await.unwrap();
//         }

//         let mut new_orders =
//             Strategies::<Core>::check_completed_orders_strategies(exec_orders, c, o, h, l);

//         if !new_orders.is_empty() {
//             reload_needed = true;
//         }

//         for order in new_orders.iter_mut() {
//             // A trade in Binance should take place here
//             // Assume that the trade in Binance is successful
//             order.price_entry = c;
//             Orders::<Core>::insert_order(order.clone()).await.unwrap();
//         }

//         if reload_needed {
//             reload_open_orders().await;
//         }
//     }
// }
