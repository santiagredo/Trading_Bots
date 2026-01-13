use chrono::{Local, NaiveDateTime};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    entities::pairs::Model,
    enums::{LifecycleState, TimestampedState},
    structs::Environments,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CachePairs {
    pub models: HashMap<i32, Model>,
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl CachePairs {
    pub fn new() -> CachePairs {
        let now = Local::now().naive_local();

        CachePairs {
            models: HashMap::new(),
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Off,
        }
    }
}

impl TimestampedState for CachePairs {
    type State = LifecycleState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.status
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}

#[derive(Default)]
pub struct CachePairsEnvironments {
    pub environments: HashMap<Environments, CachePairs>,
}

impl CachePairsEnvironments {
    pub fn new() -> CachePairsEnvironments {
        CachePairsEnvironments {
            environments: HashMap::from([
                (Environments::DEV, CachePairs::new()),
                (Environments::PROD, CachePairs::new()),
            ]),
        }
    }
}

impl CachePairsEnvironments {
    pub fn get_or_create(&mut self, env: Environments) -> &mut CachePairs {
        self.environments.entry(env).or_insert_with(CachePairs::new)
    }

    pub fn get_mut(&mut self, env: &Environments) -> Option<&mut CachePairs> {
        self.environments.get_mut(env)
    }

    pub fn get(&self, env: &Environments) -> Option<&CachePairs> {
        self.environments.get(env)
    }
}
