use crate::handler::Senders;
use models::enums::SocketStatus;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct WebsocketStreams;

impl WebsocketStreams {
    pub fn new() -> Self {
        Self
    }

    // Start WebSocket connection
    pub async fn start_websocket(
        self,
        senders: Senders,
        token: &CancellationToken,
    ) -> Result<(), String> {
        self.start_websocket_core(senders, token).await
    }

    /// Get current socket status
    pub async fn get_status(self) -> SocketStatus {
        self.get_status_core().await
    }

    /// Stop WebSocket connection
    pub async fn stop_websocket() -> Result<(), String> {
        WebsocketStreams::stop_websocket_core().await
    }
}
