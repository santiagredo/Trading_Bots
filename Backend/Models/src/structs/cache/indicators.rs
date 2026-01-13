use chrono::{Local, NaiveDateTime};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    entities::indicators::Model,
    enums::{LifecycleState, TimestampedState},
    structs::Environments,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheIndicators {
    pub models: HashMap<i32, Model>,
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl CacheIndicators {
    pub fn new() -> CacheIndicators {
        let now = Local::now().naive_local();

        CacheIndicators {
            models: HashMap::new(),
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Off,
        }
    }
}

impl TimestampedState for CacheIndicators {
    type State = LifecycleState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.status
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}

#[derive(Default)]
pub struct CacheIndicatorsEnvironments {
    pub environments: HashMap<Environments, CacheIndicators>,
}

impl CacheIndicatorsEnvironments {
    pub fn new() -> CacheIndicatorsEnvironments {
        CacheIndicatorsEnvironments {
            environments: HashMap::from([
                (Environments::DEV, CacheIndicators::new()),
                (Environments::PROD, CacheIndicators::new()),
            ]),
        }
    }
}

impl CacheIndicatorsEnvironments {
    pub fn get_or_create(&mut self, env: Environments) -> &mut CacheIndicators {
        self.environments
            .entry(env)
            .or_insert_with(CacheIndicators::new)
    }

    pub fn get_mut(&mut self, env: &Environments) -> Option<&mut CacheIndicators> {
        self.environments.get_mut(env)
    }

    pub fn get(&self, env: &Environments) -> Option<&CacheIndicators> {
        self.environments.get(env)
    }
}
