use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use models::enums::{BinanceResponse, SocketState, SocketStatus, WebsocketCommand};
use once_cell::sync::Lazy;
use serde_json::json;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{error, info, warn};

use crate::handler::WebsocketStreams;
use crate::{
    handler::{Senders, SubscribedIndicators, Tickers},
    static_strings::BINANCE_WEBSOCKET_STREAM_URL,
};

static WEBSOCKET_STATUS: Lazy<Arc<RwLock<SocketStatus>>> =
    Lazy::new(|| Arc::new(RwLock::new(SocketStatus::default())));

static WEBSOCKET_HANDLE: Lazy<Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl WebsocketStreams {
    // === State ===

    pub async fn state() -> SocketStatus {
        WEBSOCKET_STATUS.read().await.clone()
    }

    async fn set_state(state: SocketState) {
        let mut status = WEBSOCKET_STATUS.write().await;
        status.state = state;

        if state == SocketState::Connected {
            status.last_connected = Some(chrono::Local::now().naive_local());
            status.reconnect_attempts = 0;
        }
    }

    async fn set_error(error: String) {
        WEBSOCKET_STATUS.write().await.last_error = Some(error);
    }

    async fn increment_reconnect_attempts() {
        WEBSOCKET_STATUS.write().await.reconnect_attempts += 1;
    }

    // === Handle Management ===

    pub async fn set_handle(handle: tokio::task::JoinHandle<()>) {
        let mut handle_lock = WEBSOCKET_HANDLE.write().await;

        // Abort previous handle if exists
        if let Some(prev_handle) = handle_lock.as_ref() {
            if !prev_handle.is_finished() {
                prev_handle.abort();
            }
        }

        *handle_lock = Some(handle);
    }

    pub async fn abort_handle() {
        dbg!("Triggered abort handle");
        let mut handle_lock = WEBSOCKET_HANDLE.write().await;

        // Abort previous handle if exists
        if let Some(prev_handle) = handle_lock.as_ref() {
            if !prev_handle.is_finished() {
                prev_handle.abort();
            }

            *handle_lock = None;
        }
    }

    // === WebSocket Loop ===

    pub fn spawn_socket_loop(self, senders: Senders) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let ws_url = BINANCE_WEBSOCKET_STREAM_URL;
            let timeout_duration = Duration::from_secs(30);
            let mut command_receiver = senders.command_sender.subscribe();

            'reconnect: loop {
                // Attempt connection
                Self::set_state(SocketState::Connecting).await;

                let ws_stream = match Self::connect_with_retry(ws_url).await {
                    Ok(stream) => stream,
                    Err(err) => {
                        Self::set_state(SocketState::Closed).await;
                        dbg!("Failed to connect: {}", err);
                        break;
                    }
                };

                Self::set_state(SocketState::Connected).await;
                info!("WebSocket connected successfully");

                let (mut write, mut read) = ws_stream.split();

                // Subscribe to tickers
                if let Err(e) = Self::subscribe_to_tickers(&mut write).await {
                    warn!("Failed to subscribe: {}", e);
                    Self::set_error(e).await;
                    continue 'reconnect;
                }

                // Message processing loop
                'messages: loop {
                    tokio::select! {
                        // Incoming messages
                        event = read.next() => {
                            match event {
                                Some(Ok(Message::Ping(p))) => {
                                    if write.send(Message::Pong(p)).await.is_err() {
                                        warn!("Failed to send pong");
                                        break 'messages;
                                    }
                                }
                                Some(Ok(Message::Text(txt))) => {
                                    Self::process_message(&txt, &senders).await;
                                }
                                Some(Ok(Message::Close(close))) => {
                                    info!("Socket closed: {:?}", close);
                                    break 'reconnect;
                                }
                                Some(Err(err)) => {
                                    error!("Socket error: {:?}", err);
                                    Self::set_error(err.to_string()).await;
                                    Self::set_state(SocketState::Reconnecting).await;
                                    break 'messages;
                                }
                                None => {
                                    warn!("Stream ended");
                                    break 'messages;
                                }
                                _ => {}
                            }
                        }

                        cmd = command_receiver.recv() => {
                            match cmd {
                                Ok(WebsocketCommand::Unsubscribe) => {
                                    dbg!("Unsubscribe command received");
                                    let _ = Self::unsubscribe_to_tickers(&mut write).await;
                                }

                                Ok(WebsocketCommand::Shutdown) => {
                                    dbg!("Shutdown command received");
                                    Self::set_state(SocketState::ShuttingDown).await;
                                    let _ = write.send(Message::Close(None)).await;
                                    break 'reconnect;
                                }

                                Err(err) => {
                                    dbg!(err);
                                    break 'reconnect;
                                }
                            }
                        }


                        // Timeout
                        _ = sleep(timeout_duration) => {
                            warn!("No messages in {:?}, reconnecting", timeout_duration);
                            Self::set_state(SocketState::Reconnecting).await;
                            break 'messages;
                        }
                    }
                }

                // Delay before reconnection
                sleep(Duration::from_secs(2)).await;
            }

            Self::set_state(SocketState::Closed).await;
            dbg!("WebSocket task ended");
        })
    }

    // === Helper Methods ===

    async fn connect_with_retry(
        url: &str,
    ) -> Result<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        String,
    > {
        let mut attempt = 0u32;
        let max_delay_secs = 60;

        loop {
            match connect_async(url).await {
                Ok((stream, _)) => {
                    dbg!("Connected to {} on attempt {}", url, attempt + 1);
                    return Ok(stream);
                }
                Err(err) => {
                    Self::increment_reconnect_attempts().await;
                    Self::set_error(err.to_string()).await;

                    let delay = 2u64.pow(attempt).min(max_delay_secs);

                    dbg!(
                        "Connection failed (attempt {}): {} - Retrying in {}s",
                        attempt + 1,
                        err,
                        delay
                    );

                    sleep(Duration::from_secs(delay)).await;

                    attempt = (attempt + 1).min(6);
                }
            }
        }
    }

    async fn subscribe_to_tickers<S>(write: &mut S) -> Result<(), String>
    where
        S: SinkExt<Message> + Unpin,
        S::Error: std::fmt::Display,
    {
        let symbols = SubscribedIndicators::blank()
            .get_all_unique_symbols()
            .await
            .map_err(|e| format!("Failed to get symbols: {}", e))?;

        for symbol in symbols {
            let msg = json!({
                "method": "SUBSCRIBE",
                "params": [format!("{}@ticker", symbol.to_lowercase())],
                "id": 1
            })
            .to_string();

            write
                .send(Message::Text(msg.into()))
                .await
                .map_err(|e| format!("Failed to subscribe to {}: {}", symbol, e))?;

            info!("Subscribed to {}", symbol);
        }

        Ok(())
    }

    async fn unsubscribe_to_tickers<S>(write: &mut S) -> Result<(), String>
    where
        S: SinkExt<Message> + Unpin,
        S::Error: std::fmt::Display,
    {
        let symbols = SubscribedIndicators::blank().remove_entries().await;

        for symbol in symbols {
            let msg = json!({
                "method": "UNSUBSCRIBE",
                "params": [format!("{}@ticker", symbol.to_lowercase())],
                "id": 1
            })
            .to_string();

            write
                .send(Message::Text(msg.into()))
                .await
                .map_err(|e| format!("Failed to unsubscribe to {}: {}", symbol, e))?;

            dbg!("Unsubscribed to {}", symbol);
        }

        Ok(())
    }

    async fn process_message(text: &str, senders: &Senders) {
        match serde_json::from_str::<BinanceResponse>(text) {
            Ok(response) => match response {
                BinanceResponse::Error(err) => {
                    error!("Binance error: {:?}", err);
                }
                BinanceResponse::StreamResponse(val) => {
                    info!("Stream response: {:?}", val);
                }
                BinanceResponse::SubscribeResponse(val) => {
                    info!("Subscribe response: {:?}", val);
                }
                BinanceResponse::Ticker(ticker) => {
                    Tickers::set_ticker(ticker.clone()).await;
                    let _ = senders.event_sender.send(ticker);
                }
                _ => {}
            },
            Err(e) => {
                warn!("Failed to parse message: {} - Text: {}", e, text);
            }
        }
    }
}
