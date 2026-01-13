use chrono::{Local, NaiveDateTime};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    ShuttingDown,
    Closed,
}

#[derive(Debug, Clone)]
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
