use serde::{Deserialize, Serialize};

use crate::entities::status;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum Status {
    #[default]
    Open,
    Completed,
    Aborted,
}

impl Status {
    pub fn from_model(model: &status::Model) -> Self {
        match model.name.as_ref() {
            "COMPLETED" => Self::Completed,
            "ABORTED" => Self::Aborted,
            _ => Self::Open,
        }
    }
}
