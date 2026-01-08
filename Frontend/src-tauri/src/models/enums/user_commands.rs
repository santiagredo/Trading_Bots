use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserCommands {
    Start,
    Stop,
    Restart,
}

impl UserCommands {
    pub fn as_str(&self) -> &'static str {
        match self {
            &UserCommands::Start => "start",
            &UserCommands::Stop => "stop",
            &UserCommands::Restart => "restart",
        }
    }
}
