use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::models::enums::{LifecycleState, TimestampedState};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Runtime {
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl Runtime {
    pub fn new() -> Runtime {
        let now = Local::now().naive_local();

        Runtime {
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Off,
        }
    }
}

impl TimestampedState for Runtime {
    type State = LifecycleState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.status
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheRuntimes {
    pub dev: Runtime,
    pub prod: Runtime,
}

impl CacheRuntimes {
    pub fn new() -> CacheRuntimes {
        CacheRuntimes {
            dev: Runtime::new(),
            prod: Runtime::new(),
        }
    }
}
