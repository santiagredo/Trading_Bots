use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use models::enums::{BinanceResponse, SocketState, SocketStatus, WebsocketCommand};
use once_cell::sync::Lazy;
use serde_json::json;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::handler::WebsocketStreams;
use crate::{
    handler::{Senders, SubscribedIndicators, Tickers},
    static_strings::BINANCE_WEBSOCKET_STREAM_URL,
    utils::Integration,
};

static WEBSOCKET_STATUS: Lazy<Arc<RwLock<SocketStatus>>> =
    Lazy::new(|| Arc::new(RwLock::new(SocketStatus::default())));

static WEBSOCKET_HANDLE: Lazy<Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl WebsocketStreams<Integration> {
    // === State ===

    pub async fn get_status_integration() -> SocketStatus {
        WEBSOCKET_STATUS.read().await.clone()
    }

    async fn update_state(state: SocketState) {
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

    pub async fn set_handle_integration(handle: tokio::task::JoinHandle<()>) {
        let mut handle_lock = WEBSOCKET_HANDLE.write().await;

        // Abort previous handle if exists
        if let Some(prev_handle) = handle_lock.as_ref() {
            if !prev_handle.is_finished() {
                prev_handle.abort();
            }
        }

        *handle_lock = Some(handle);
    }

    pub async fn abort_handle_integration() -> Result<(), String> {
        let handle_lock = WEBSOCKET_HANDLE.write().await;

        // Abort previous handle if exists
        if let Some(prev_handle) = handle_lock.as_ref() {
            if !prev_handle.is_finished() {
                prev_handle.abort();
            }
        }

        Ok(())
    }

    pub async fn take_handle_integration() -> Option<tokio::task::JoinHandle<()>> {
        WEBSOCKET_HANDLE.write().await.take()
    }

    // === WebSocket Loop ===

    pub fn spawn_socket_loop_integration(
        self,
        senders: Senders,
        cancellation_token: CancellationToken,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let ws_url = BINANCE_WEBSOCKET_STREAM_URL;
            let timeout_duration = Duration::from_secs(30);
            let mut command_receiver = senders.command_sender.subscribe();

            'reconnect: loop {
                // Check cancellation
                if cancellation_token.is_cancelled() {
                    dbg!("WebSocket cancelled before connection");
                    Self::update_state(SocketState::Closed).await;
                    break;
                }

                // Attempt connection
                Self::update_state(SocketState::Connecting).await;

                let ws_stream = match Self::connect_with_retry(ws_url, &cancellation_token).await {
                    Ok(stream) => stream,
                    Err(err) => {
                        Self::update_state(SocketState::Closed).await;
                        dbg!("Failed to connect: {}", err);
                        break;
                    }
                };

                Self::update_state(SocketState::Connected).await;
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
                        // Cancellation
                        _ = cancellation_token.cancelled() => {
                            info!("WebSocket shutdown requested");
                            Self::update_state(SocketState::ShuttingDown).await;
                            let _ = write.send(Message::Close(None)).await;
                            break 'reconnect;
                        }

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
                                    Self::update_state(SocketState::Reconnecting).await;
                                    break 'messages;
                                }
                                None => {
                                    warn!("Stream ended");
                                    break 'messages;
                                }
                                _ => {}
                            }
                        }

                        // Internal commands
                        cmd = command_receiver.recv() => {
                            if let Ok(WebsocketCommand::Shutdown) = cmd {
                                info!("Shutdown command received");
                                Self::update_state(SocketState::ShuttingDown).await;
                                let _ = write.send(Message::Close(None)).await;
                                break 'reconnect;
                            }
                        }

                        // Timeout
                        _ = sleep(timeout_duration) => {
                            warn!("No messages in {:?}, reconnecting", timeout_duration);
                            Self::update_state(SocketState::Reconnecting).await;
                            break 'messages;
                        }
                    }
                }

                // Delay before reconnection
                sleep(Duration::from_secs(2)).await;
            }

            Self::update_state(SocketState::Closed).await;
            dbg!("WebSocket task ended");
        })
    }

    // === Helper Methods ===

    async fn connect_with_retry(
        url: &str,
        cancellation_token: &CancellationToken,
    ) -> Result<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        String,
    > {
        let mut attempt = 0u32;
        let max_delay_secs = 60;

        loop {
            if cancellation_token.is_cancelled() {
                return Err("Connection cancelled".to_string());
            }

            match connect_async(url).await {
                Ok((stream, _)) => {
                    info!("Connected to {} on attempt {}", url, attempt + 1);
                    return Ok(stream);
                }
                Err(err) => {
                    Self::increment_reconnect_attempts().await;
                    Self::set_error(err.to_string()).await;

                    let delay = 2u64.pow(attempt).min(max_delay_secs);

                    warn!(
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
        let symbols = SubscribedIndicators::get_all_unique_symbols()
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
