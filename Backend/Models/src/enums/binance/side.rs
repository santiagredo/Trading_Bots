use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Side {
    #[default]
    Buy,
    Sell,
}