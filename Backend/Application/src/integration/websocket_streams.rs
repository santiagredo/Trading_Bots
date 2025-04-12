use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use models::enums::BinanceResponse;
use serde_json::json;
use tokio::{select, time::sleep};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::error_span;

use crate::types::{MiniTickers, Tickers};

pub async fn binance_websocket_client_task(sockets: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let ws_url = "wss://stream.binance.com:9443/ws";

    let mut attempt = 0;

    loop {
        match connect_async(ws_url).await {
            Err(err) => println!("Socket connection failed: {err} \n"),
            Ok((ws_stream, _response)) => {
                println!("Connected to {ws_url} \n");

                let (mut write, mut read) = ws_stream.split();

                let subscribe_msg = json!({
                    "method": "SUBSCRIBE",
                    "params": sockets
                })
                .to_string();

                if let Err(err) = write.send(Message::Text(subscribe_msg.into())).await {
                    println!("Subscription failed: {err} \n");

                    attempt += 1;
                    let delay = Duration::from_secs(2_u64.pow(attempt.min(5))); // Max 32 seconds
                    println!("Reconnecting in {:?}...\n", delay);
                    sleep(delay).await;

                    continue;
                }

                println!("Subscribe OK \n");

                attempt = 0;

                let timeout_duration = Duration::from_secs(15);

                loop {
                    let timeout = sleep(timeout_duration);

                    select! {
                        // Wait for a message or timeout
                        msg = read.next() => {
                            match msg {
                                Some(Ok(Message::Ping(payload))) => {
                                    println!("Ping received \n");

                                    if let Err(err) = write.send(Message::Pong(payload)).await {
                                        println!("Failed to send Pong: {err} \n");
                                    }
                                }
                                Some(Ok(Message::Text(text))) => {
                                    if let Ok(parsed) = serde_json::from_str::<BinanceResponse>(&text) {
                                        process_socket_texts(parsed).await;
                                    }
                                }
                                Some(Ok(Message::Close(e))) => {
                                    println!("WebSocket closed by server {e:?} \n");
                                    break;
                                }
                                Some(Err(err)) => {
                                    println!("WebSocket error: {err} \n");
                                    break;
                                }
                                None => {
                                    println!("WebSocket unexpectedly closed \n");
                                    break;
                                }
                                _ => {}
                            }
                        }
                        _ = timeout => {
                            println!("No messages received in {:?}, assuming disconnection \n", timeout_duration);
                            break;
                        }
                    }
                }
            }
        }

        attempt += 1;
        let delay = Duration::from_secs(2_u64.pow(attempt.min(5))); // Max 32 seconds
        println!("Reconnecting in {:?}... \n", delay);
        sleep(delay).await;
    }
}

pub async fn process_socket_texts(response: BinanceResponse) {
    match response {
        BinanceResponse::Error(err) => {
            error_span!("Error - Binance", error = ?err);
        }
        BinanceResponse::StreamResponse(val) => println!("{val:?} \n"),
        BinanceResponse::SubscribeResponse(val) => println!("{val:?} \n"),
        BinanceResponse::Ticker(ticker) => {
            println!(
                "tme: {}, sbl: {}, pch: {}, pcp: {}, wap: {}, lst: {}, opn: {}, hgh: {}, low: {}, bav: {}, qav: {} \n",
                ticker.event_time, ticker.symbol, ticker.price_change, ticker.price_change_percent, ticker.weighted_avg_price, ticker.last_price,
                ticker.open_price, ticker.high_price, ticker.low_price, ticker.base_asset_volume, ticker.quote_asset_volume
            );

            Tickers::set_ticker(ticker).await;
        }
        BinanceResponse::MiniTicker(mini_ticker) => {
            println!(
                "MiniTicker: {}, last: {}, open: {}, high: {}, low: {}, base_asset_volume: {}, quote_asset_volume: {} \n",
                mini_ticker.symbol,
                mini_ticker.last_price,
                mini_ticker.open_price,
                mini_ticker.high_price,
                mini_ticker.low_price,
                mini_ticker.base_asset_volume,
                mini_ticker.quote_asset_volume
            );

            MiniTickers::set_mini_ticker(mini_ticker).await;
        }
        _ => ()
    }
}
