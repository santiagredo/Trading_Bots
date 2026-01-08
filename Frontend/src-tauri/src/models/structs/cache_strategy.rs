use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::entities::strategies::Model;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheStrategy {
    pub is_posting: bool,
    pub model: Model,
    pub last_error_date: Option<NaiveDateTime>,
    pub last_error_message: Option<String>,
}
