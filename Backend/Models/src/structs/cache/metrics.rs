use chrono::{Local, NaiveDateTime};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    entities::critical_metrics::Model,
    enums::{LifecycleState, TimestampedState},
    structs::Environments,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheMetrics {
    pub model: Model,
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl CacheMetrics {
    pub fn new() -> CacheMetrics {
        let now = Local::now().naive_local();

        CacheMetrics {
            model: Model::default(),
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Off,
        }
    }
}

impl TimestampedState for CacheMetrics {
    type State = LifecycleState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.status
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}

#[derive(Default)]
pub struct CacheMetricsEnvironments {
    pub environments: HashMap<Environments, CacheMetrics>,
}

impl CacheMetricsEnvironments {
    pub fn new() -> CacheMetricsEnvironments {
        CacheMetricsEnvironments {
            environments: HashMap::from([
                (Environments::DEV, CacheMetrics::new()),
                (Environments::PROD, CacheMetrics::new()),
            ]),
        }
    }
}

impl CacheMetricsEnvironments {
    pub fn get_or_create(&mut self, env: Environments) -> &mut CacheMetrics {
        self.environments
            .entry(env)
            .or_insert_with(CacheMetrics::new)
    }

    pub fn get_mut(&mut self, env: &Environments) -> Option<&mut CacheMetrics> {
        self.environments.get_mut(env)
    }

    pub fn get(&self, env: &Environments) -> Option<&CacheMetrics> {
        self.environments.get(env)
    }
}
