use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub asset: String,

    pub free: Decimal,

    pub locked: Decimal,
}
