use std::collections::HashMap;

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::enums::{LifecycleState, TimestampedState};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheSubscribedIndicators {
    pub models: HashMap<String, i32>,
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl CacheSubscribedIndicators {
    pub fn new() -> CacheSubscribedIndicators {
        let now = Local::now().naive_local();

        CacheSubscribedIndicators {
            models: HashMap::new(),
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Running,
        }
    }
}

impl TimestampedState for CacheSubscribedIndicators {
    type State = LifecycleState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.status
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}

impl CacheSubscribedIndicators {
    pub fn get_or_create(&mut self, symbol: String) -> &mut i32 {
        self.models.entry(symbol).or_insert_with(|| 0)
    }

    pub fn get_mut(&mut self, symbol: &str) -> Option<&mut i32> {
        self.models.get_mut(symbol)
    }

    pub fn get(&self, symbol: &str) -> Option<&i32> {
        self.models.get(symbol)
    }
}
