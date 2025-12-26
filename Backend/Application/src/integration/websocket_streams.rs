use std::time::Duration;

use chrono::Local;
use futures_util::{SinkExt, StreamExt};
use models::{
    enums::{BinanceResponse, WebsocketCommand},
    structs::BinanceError,
};
use serde_json::json;
use tokio::{task::AbortHandle, time::sleep};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::error_span;

use crate::{
    handler::{Senders, SubscribedIndicators, Tickers, WebsocketStreams},
    static_strings::BINANCE_WEBSOCKET_STREAM_URL,
    utils::Integration,
};

impl WebsocketStreams<Integration> {
    /// Handles incoming messages from Binance WebSocket and internal application commands
    pub fn spawn_socket_loop_integration(self, senders: Senders) -> AbortHandle {
        let socket_handle = tokio::spawn(async move {
            let ws_url = BINANCE_WEBSOCKET_STREAM_URL;
            let mut attempt = 0;
            let timeout_duration = Duration::from_secs(5);
            let mut command_receiver = senders.command_sender.clone().subscribe();

            // outer loop handles the overall socket connection and reconnection
            'outer: loop {
                // tries to connect to websocket
                let (ws_stream, _) = match connect_async(ws_url).await {
                    Ok(v) => {
                        dbg!(format!("Connected to {ws_url}"));

                        attempt = 0;

                        v
                    }
                    Err(err) => {
                        attempt += 1;
                        let now = Local::now().naive_local();

                        dbg!(eprintln!(
                            "Connection failed at {now} : {err} -- Retrying in {} secs",
                            2u64.pow(attempt.min(5))
                        ));

                        sleep(Duration::from_secs(2u64.pow(attempt.min(5)))).await;
                        continue;
                    }
                };

                // splits the websocket stream between write and read
                let (mut write, mut read) = ws_stream.split();

                // gets the subscribed indicators symbols to resubscribe in case the websocket connection drops
                let symbols: Vec<String> = SubscribedIndicators::new(self.environment)
                    .get_active_subscribed_indicators()
                    .await
                    .unwrap_or_default()
                    .keys()
                    .map(|key| key.clone())
                    .collect();

                for symbol in symbols {
                    dbg!(format!("Sending symbol subscription: {symbol}"));

                    let msg = json!({
                        "method": "SUBSCRIBE",
                        "params": [format!("{}@ticker", symbol.to_lowercase())],
                        "id": 1
                    })
                    .to_string();
                    let _ = write.send(Message::Text(msg.into())).await;
                }

                // inner loop handles incoming socket texts and application internal commands
                'inner: loop {
                    let timeout = sleep(timeout_duration);

                    // select is used to execute the first branch, canceling the others, used to break inner loop in case the connection drops
                    tokio::select! {
                        // incoming texts
                        event = read.next() => {
                            match event {
                                Some(Ok(Message::Ping(p))) => {
                                    let _ = write.send(Message::Pong(p)).await;
                                }
                                Some(Ok(Message::Text(txt))) => {
                                    if let Ok(parsed) = serde_json::from_str::<BinanceResponse>(&txt) {
                                        if let BinanceResponse::Ticker(ticker) = Self::process_socket_texts(parsed).await {
                                            let _ = senders.event_sender.send(ticker);
                                        }
                                    }
                                }
                                Some(Ok(Message::Close(close))) => {
                                    dbg!(println!("Socket closed: {close:?}"));
                                    break 'outer;
                                }
                                Some(Err(err)) => {
                                    dbg!(println!("Socket err: {err:?}"));
                                    break 'inner;
                                }
                                None => break,
                                _ => {}
                            }
                        }

                        // internal commands
                        cmd = command_receiver.recv() => {
                            if let Ok(cmd) = cmd {
                                match cmd {
                                    // WebsocketCommand::Reload => {
                                    //     println!("Reloading WebSocket...");
                                    //     let _ = write.send(Message::Close(None)).await;
                                    //     break 'inner;
                                    // }
                                    WebsocketCommand::Shutdown => {
                                        dbg!(println!("Shutting down WebSocket..."));
                                        let _ = write.send(Message::Close(None)).await;
                                        break 'outer;
                                    }
                                }
                            }
                        }

                        // breaks inner loop as no texts have beeen received
                        _ = timeout => {
                            let now = Local::now().naive_local();

                            dbg!(println!("No messages received in {:?}, closing socket at {:?}", timeout_duration, now));
                            break 'inner;
                        }
                    }
                }
            }

            println!("WebSocket task ended");
        });

        socket_handle.abort_handle()
    }

    async fn process_socket_texts(response: BinanceResponse) -> BinanceResponse {
        match response {
            BinanceResponse::Error(err) => {
                error_span!("Error - Binance", error = ?err);
                BinanceResponse::Error(err)
            }
            BinanceResponse::StreamResponse(val) => {
                dbg!(format!("{val:?}"));
                BinanceResponse::StreamResponse(val)
            }
            BinanceResponse::SubscribeResponse(val) => {
                dbg!(format!("{val:?}"));
                BinanceResponse::SubscribeResponse(val)
            }
            BinanceResponse::Ticker(ticker) => {
                // println!(
                //     "tme: {}, sbl: {}, pch: {}, pcp: {}, wap: {}, lst: {}, opn: {}, hgh: {}, low: {}, bav: {}, qav: {}",
                //     ticker.event_time, ticker.symbol, ticker.price_change, ticker.price_change_percent, ticker.weighted_avg_price, ticker.last_price,
                //     ticker.open_price, ticker.high_price, ticker.low_price, ticker.base_asset_volume, ticker.quote_asset_volume
                // );
                // println!(
                //     "Received ticker: {} {} {}",
                //     ticker.symbol, ticker.event_time, ticker.last_price
                // );

                Tickers::set_ticker(ticker.clone()).await;
                BinanceResponse::Ticker(ticker)
            }

            _ => BinanceResponse::Error(BinanceError::default()),
        }
    }
}
