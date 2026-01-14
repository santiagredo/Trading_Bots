use serde::{Deserialize, Serialize};

use crate::enums::OrderDirection;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryOptions {
    pub limit: Option<u64>,

    pub offset: Option<u64>,

    pub order_by: Option<String>,

    pub order_direction: Option<OrderDirection>,
}
