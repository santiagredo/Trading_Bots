use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::enums::{LifecycleState, TimestampedState};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheEngine {
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl CacheEngine {
    pub fn new() -> CacheEngine {
        let now = Local::now().naive_local();

        CacheEngine {
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Starting,
        }
    }
}

impl TimestampedState for CacheEngine {
    type State = LifecycleState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.status
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}
