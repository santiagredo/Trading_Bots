use std::collections::HashMap;

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use tokio::task::AbortHandle;

use crate::{
    models::entities::tasks::Model,
    models::enums::{LifecycleState, TaskState, TimestampedState},
    models::structs::Environments,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheTask {
    pub model: Model,
    pub state: TaskState,
    #[serde(skip, default)]
    pub abort_handle: Option<AbortHandle>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheTasks {
    pub models: HashMap<i32, CacheTask>,
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl CacheTasks {
    pub fn new() -> CacheTasks {
        let now = Local::now().naive_local();

        CacheTasks {
            models: HashMap::new(),
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Off,
        }
    }
}

impl TimestampedState for CacheTasks {
    type State = LifecycleState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.status
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}

#[derive(Default)]
pub struct CacheTasksEnvironments {
    pub environments: HashMap<Environments, CacheTasks>,
}

impl CacheTasksEnvironments {
    pub fn new() -> CacheTasksEnvironments {
        CacheTasksEnvironments {
            environments: HashMap::from([
                (Environments::DEV, CacheTasks::new()),
                (Environments::PROD, CacheTasks::new()),
            ]),
        }
    }
}

impl CacheTasksEnvironments {
    pub fn get_or_create(&mut self, env: Environments) -> &mut CacheTasks {
        self.environments.entry(env).or_insert_with(CacheTasks::new)
    }

    pub fn get_mut(&mut self, env: &Environments) -> Option<&mut CacheTasks> {
        self.environments.get_mut(env)
    }

    pub fn get(&self, env: &Environments) -> Option<&CacheTasks> {
        self.environments.get(env)
    }
}
