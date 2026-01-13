use std::collections::{HashMap, HashSet};

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::{
    enums::{LifecycleState, TimestampedState},
    structs::Environments,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheSubscribedIndicators {
    pub models: HashMap<String, HashSet<i32>>,
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
            status: LifecycleState::Off,
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

#[derive(Default)]
pub struct CacheSubscribedIndicatorsEnvironments {
    pub environments: HashMap<Environments, CacheSubscribedIndicators>,
}

impl CacheSubscribedIndicatorsEnvironments {
    pub fn new() -> CacheSubscribedIndicatorsEnvironments {
        CacheSubscribedIndicatorsEnvironments {
            environments: HashMap::from([
                (Environments::DEV, CacheSubscribedIndicators::new()),
                (Environments::PROD, CacheSubscribedIndicators::new()),
            ]),
        }
    }
}

impl CacheSubscribedIndicatorsEnvironments {
    pub fn get_or_create(&mut self, env: Environments) -> &mut CacheSubscribedIndicators {
        self.environments
            .entry(env)
            .or_insert_with(CacheSubscribedIndicators::new)
    }

    pub fn get_mut(&mut self, env: &Environments) -> Option<&mut CacheSubscribedIndicators> {
        self.environments.get_mut(env)
    }

    pub fn get(&self, env: &Environments) -> Option<&CacheSubscribedIndicators> {
        self.environments.get(env)
    }
}
