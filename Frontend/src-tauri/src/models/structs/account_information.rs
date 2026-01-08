use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

use super::{Balance, CommissionRates};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub maker_commission: Decimal,

    pub taker_commission: Decimal,

    pub buyer_commission: Decimal,

    pub seller_commission: Decimal,

    pub commission_rates: CommissionRates,

    pub can_trade: bool,

    pub can_withdraw: bool,

    pub can_deposit: bool,

    pub brokered: bool,

    pub require_self_trade_prevention: bool,

    pub prevent_sor: bool,

    pub update_time: u64,

    pub account_type: String,

    pub balances: Vec<Balance>,

    pub permissions: Vec<String>,

    pub uid: u64,
}
