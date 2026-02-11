use models::enums::{SocketState, WebsocketCommand};

use crate::handler::{Senders, SubscribedIndicators, WebsocketStreams};

impl WebsocketStreams {
    pub async fn start(self, senders: Senders) -> Result<(), String> {
        if WebsocketStreams::state().await.state != SocketState::Connected {
            // Spawn WebSocket task in Integration
            let handle = self.spawn_socket_loop(senders);

            // Store handle in Integration
            WebsocketStreams::set_handle(handle).await;
        }

        Ok(())
    }

    pub async fn stop(senders: Senders) -> Result<(), String> {
        let subs = SubscribedIndicators::blank().get_all().await;

        if subs.models.is_empty() {
            senders
                .command_sender
                .send(WebsocketCommand::Shutdown)
                .map_err(|err| err.to_string())?;
        }

        Ok(())
    }
}
