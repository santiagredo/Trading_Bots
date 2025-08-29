use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum WebsocketCommand {
    // Reload,
    Shutdown,
}
