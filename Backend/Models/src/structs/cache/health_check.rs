use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::enums::LifecycleState;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheHealthCheck {
    pub startup_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl CacheHealthCheck {
    pub fn new() -> CacheHealthCheck {
        let startup_date = Local::now().naive_local();

        CacheHealthCheck {
            startup_date,
            status: LifecycleState::Starting,
        }
    }
}
