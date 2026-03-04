use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommissionRates {
    pub maker: Decimal,

    pub taker: Decimal,

    pub buyer: Decimal,

    pub seller: Decimal,
}
