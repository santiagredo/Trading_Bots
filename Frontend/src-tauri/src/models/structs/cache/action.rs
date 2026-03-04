use chrono::{Local, NaiveDateTime};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    models::entities::actions::Model,
    models::enums::{LifecycleState, TimestampedState},
    models::structs::Environments,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheActions {
    pub models: HashMap<i32, Model>,
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl CacheActions {
    pub fn new() -> CacheActions {
        let now = Local::now().naive_local();

        CacheActions {
            models: HashMap::new(),
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Off,
        }
    }
}

impl TimestampedState for CacheActions {
    type State = LifecycleState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.status
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}

#[derive(Default)]
pub struct CacheActionsEnvironments {
    pub environments: HashMap<Environments, CacheActions>,
}

impl CacheActionsEnvironments {
    pub fn new() -> CacheActionsEnvironments {
        CacheActionsEnvironments {
            environments: HashMap::from([
                (Environments::DEV, CacheActions::new()),
                (Environments::PROD, CacheActions::new()),
            ]),
        }
    }
}

impl CacheActionsEnvironments {
    pub fn get_or_create(&mut self, env: Environments) -> &mut CacheActions {
        self.environments
            .entry(env)
            .or_insert_with(CacheActions::new)
    }

    pub fn get_mut(&mut self, env: &Environments) -> Option<&mut CacheActions> {
        self.environments.get_mut(env)
    }

    pub fn get(&self, env: &Environments) -> Option<&CacheActions> {
        self.environments.get(env)
    }
}
