use models::enums::SocketStatus;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{
    handler::{Senders, WebsocketStreams},
    utils::{Cache, Core, Integration},
};

impl WebsocketStreams<Core> {
    pub async fn start_websocket_core(
        self,
        senders: Senders,
        token: &CancellationToken,
    ) -> Result<(), String> {
        // Create cancellation token
        let cancellation_token = token.clone();

        // Store token in cache
        WebsocketStreams::<Cache>::set_cancellation_token_cache(cancellation_token.clone()).await;

        // Spawn WebSocket task in Integration
        let handle = self
            .next_phase()
            .spawn_socket_loop_integration(senders, cancellation_token);

        // Store handle in Integration
        WebsocketStreams::<Integration>::set_handle_integration(handle).await;

        Ok(())
    }

    pub async fn get_status_core(self) -> SocketStatus {
        WebsocketStreams::<Integration>::get_status_integration().await
    }

    pub async fn stop_websocket_core() -> Result<(), String> {
        // Get cancellation token from cache
        let token = WebsocketStreams::<Cache>::get_cancellation_token_cache()
            .await
            .ok_or("No active WebSocket connection")?;

        // Signal cancellation
        token.cancel();

        // Take handle from Integration
        let handle = WebsocketStreams::<Integration>::take_handle_integration()
            .await
            .ok_or("No WebSocket handle found")?;

        // Wait for task to finish with timeout
        match tokio::time::timeout(Duration::from_secs(10), handle).await {
            Ok(Ok(())) => {
                info!("WebSocket stopped cleanly");

                // Clean up cache
                WebsocketStreams::<Cache>::remove_cancellation_token_cache().await;

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
