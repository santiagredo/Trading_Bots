use models::{enums::SocketStatus, structs::Environments};
use std::marker::PhantomData;
use tokio_util::sync::CancellationToken;

use crate::{
    handler::Senders,
    utils::{Core, Types},
};

#[derive(Debug, Clone)]
pub struct WebsocketStreams<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
}

impl<Phase> WebsocketStreams<Phase> {
    fn cast<Next>(self) -> WebsocketStreams<Next> {
        WebsocketStreams {
            phase: PhantomData,
            environment: self.environment,
        }
    }

    pub fn next_phase<Next>(self) -> WebsocketStreams<Next> {
        self.cast()
    }
}

impl WebsocketStreams {
    pub fn new(environment: Environments) -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
        }
    }

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: self.phase,
            environment,
        }
    }
}

impl WebsocketStreams<Types> {
    /// Start WebSocket connection
    pub async fn start_websocket(
        self,
        senders: Senders,
        token: &CancellationToken,
    ) -> Result<(), String> {
        self.next_phase().start_websocket_core(senders, token).await
    }

    /// Get current socket status
    pub async fn get_status(self) -> SocketStatus {
        self.next_phase().get_status_core().await
    }

    /// Stop WebSocket connection
    pub async fn stop_websocket() -> Result<(), String> {
        WebsocketStreams::<Core>::stop_websocket_core().await
    }
}
