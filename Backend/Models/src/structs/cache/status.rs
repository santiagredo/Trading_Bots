use chrono::{Local, NaiveDateTime};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    entities::status::Model,
    enums::{LifecycleState, TimestampedState},
    structs::Environments,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheStatus {
    pub models: HashMap<i32, Model>,
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl CacheStatus {
    pub fn new() -> CacheStatus {
        let now = Local::now().naive_local();

        CacheStatus {
            models: HashMap::new(),
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Off,
        }
    }
}

impl TimestampedState for CacheStatus {
    type State = LifecycleState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.status
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}

#[derive(Default)]
pub struct CacheStatusEnvironments {
    pub environments: HashMap<Environments, CacheStatus>,
}

impl CacheStatusEnvironments {
    pub fn new() -> CacheStatusEnvironments {
        CacheStatusEnvironments {
            environments: HashMap::from([
                (Environments::DEV, CacheStatus::new()),
                (Environments::PROD, CacheStatus::new()),
            ]),
        }
    }
}

impl CacheStatusEnvironments {
    pub fn get_or_create(&mut self, env: Environments) -> &mut CacheStatus {
        self.environments
            .entry(env)
            .or_insert_with(CacheStatus::new)
    }

    pub fn get_mut(&mut self, env: &Environments) -> Option<&mut CacheStatus> {
        self.environments.get_mut(env)
    }

    pub fn get(&self, env: &Environments) -> Option<&CacheStatus> {
        self.environments.get(env)
    }
}
