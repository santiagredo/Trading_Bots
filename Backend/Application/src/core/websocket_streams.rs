use models::enums::SocketStatus;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::handler::{Senders, WebsocketStreams};

impl WebsocketStreams {
    pub async fn start_websocket_core(
        self,
        senders: Senders,
        token: &CancellationToken,
    ) -> Result<(), String> {
        // Create cancellation token
        let cancellation_token = token.clone();

        // Store token in cache
        WebsocketStreams::set_cancellation_token_cache(cancellation_token.clone()).await;

        // Spawn WebSocket task in Integration
        let handle = self
            .spawn_socket_loop_integration(senders, cancellation_token);

        // Store handle in Integration
        WebsocketStreams::set_handle_integration(handle).await;

        Ok(())
    }

    pub async fn get_status_core(self) -> SocketStatus {
        WebsocketStreams::get_status_integration().await
    }

    pub async fn stop_websocket_core() -> Result<(), String> {
        // Get cancellation token from cache
        let token = WebsocketStreams::get_cancellation_token_cache()
            .await
            .ok_or("No active WebSocket connection")?;

        // Signal cancellation
        token.cancel();

        // Take handle from Integration
        let handle = WebsocketStreams::take_handle_integration()
            .await
            .ok_or("No WebSocket handle found")?;

        // Wait for task to finish with timeout
        match tokio::time::timeout(Duration::from_secs(10), handle).await {
            Ok(Ok(())) => {
                info!("WebSocket stopped cleanly");

                // Clean up cache
                WebsocketStreams::remove_cancellation_token_cache().await;

                Ok(())
            }
            Ok(Err(e)) => {
                error!("WebSocket task panicked: {:?}", e);
                Err(format!("Task panicked: {:?}", e))
            }
            Err(_) => {
                error!("WebSocket didn't stop within 10s");
                Err("Shutdown timeout".to_string())
            }
        }
    }
}
