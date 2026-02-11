use std::collections::HashMap;

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use tokio::task::AbortHandle;

use crate::{
    entities::strategies::Model,
    enums::{LifecycleState, TimestampedState, TradingState},
    structs::Environments,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheStrategy {
    pub model: Model,
    pub state: TradingState,
    pub last_update_date: NaiveDateTime,
    pub last_error_message: Option<String>,
}

impl TimestampedState for CacheStrategy {
    type State = TradingState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheStrategies {
    pub models: HashMap<i32, CacheStrategy>,
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
    #[serde(skip, default)]
    pub abort_handle: Option<AbortHandle>,
}

impl CacheStrategies {
    pub fn new() -> CacheStrategies {
        let now = Local::now().naive_local();

        CacheStrategies {
            models: HashMap::new(),
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Off,
            abort_handle: None,
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
