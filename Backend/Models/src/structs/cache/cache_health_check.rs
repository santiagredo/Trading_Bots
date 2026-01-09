use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheHealthCheck {
    pub startup_date: NaiveDateTime,
}

impl CacheHealthCheck {
    pub fn new() -> CacheHealthCheck {
        let startup_date = Local::now().naive_local();

        CacheHealthCheck { startup_date }
    }
}
