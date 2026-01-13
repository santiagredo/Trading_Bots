use std::collections::HashMap;

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::{
    entities::strategies::Model,
    enums::{LifecycleState, TimestampedState},
    structs::Environments,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheStrategy {
    pub is_posting: bool,
    pub model: Model,
    pub last_error_date: Option<NaiveDateTime>,
    pub last_error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheStrategies {
    pub models: HashMap<i32, CacheStrategy>,
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl CacheStrategies {
    pub fn new() -> CacheStrategies {
        let now = Local::now().naive_local();

        CacheStrategies {
            models: HashMap::new(),
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Off,
        }
    }
}

impl TimestampedState for CacheStrategies {
    type State = LifecycleState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.status
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}

#[derive(Default)]
pub struct CacheStrategiesEnvironments {
    pub environments: HashMap<Environments, CacheStrategies>,
}

impl CacheStrategiesEnvironments {
    pub fn new() -> CacheStrategiesEnvironments {
        CacheStrategiesEnvironments {
            environments: HashMap::from([
                (Environments::DEV, CacheStrategies::new()),
                (Environments::PROD, CacheStrategies::new()),
            ]),
        }
    }
}

impl CacheStrategiesEnvironments {
    pub fn get_or_create(&mut self, env: Environments) -> &mut CacheStrategies {
        self.environments
            .entry(env)
            .or_insert_with(CacheStrategies::new)
    }

    pub fn get_mut(&mut self, env: &Environments) -> Option<&mut CacheStrategies> {
        self.environments.get_mut(env)
    }

    pub fn get(&self, env: &Environments) -> Option<&CacheStrategies> {
        self.environments.get(env)
    }
}
