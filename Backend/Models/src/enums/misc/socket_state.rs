use std::fmt::Display;

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocketState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    ShuttingDown,
    Closed,
}

impl Display for SocketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SocketState::Disconnected => "disconnected",
            SocketState::Connecting => "connecting",
            SocketState::Connected => "connected",
            SocketState::Reconnecting => "reconnecting",
            SocketState::ShuttingDown => "shutting down",
            SocketState::Closed => "closed",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketStatus {
    pub state: SocketState,
    pub startup_date: NaiveDateTime,
    pub last_connected: Option<NaiveDateTime>,
    pub last_error: Option<String>,
    pub reconnect_attempts: u32,
}

impl Default for SocketStatus {
    fn default() -> Self {
        Self {
            state: SocketState::Disconnected,
            startup_date: Local::now().naive_local(),
            last_connected: None,
            last_error: None,
            reconnect_attempts: 0,
        }
    }
}
