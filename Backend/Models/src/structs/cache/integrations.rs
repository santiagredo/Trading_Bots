use chrono::{Local, NaiveDateTime};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    entities::integrations::Model,
    enums::{LifecycleState, TimestampedState},
    structs::Environments,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheIntegrations {
    pub models: HashMap<i32, Model>,
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl CacheIntegrations {
    pub fn new() -> CacheIntegrations {
        let now = Local::now().naive_local();

        CacheIntegrations {
            models: HashMap::new(),
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Off,
        }
    }
}

impl TimestampedState for CacheIntegrations {
    type State = LifecycleState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.status
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}

#[derive(Default)]
pub struct CacheIntegrationsEnvironments {
    pub environments: HashMap<Environments, CacheIntegrations>,
}

impl CacheIntegrationsEnvironments {
    pub fn new() -> CacheIntegrationsEnvironments {
        CacheIntegrationsEnvironments {
            environments: HashMap::from([
                (Environments::DEV, CacheIntegrations::new()),
                (Environments::PROD, CacheIntegrations::new()),
            ]),
        }
    }
}

impl CacheIntegrationsEnvironments {
    pub fn get_or_create(&mut self, env: Environments) -> &mut CacheIntegrations {
        self.environments
            .entry(env)
            .or_insert_with(CacheIntegrations::new)
    }

    pub fn get_mut(&mut self, env: &Environments) -> Option<&mut CacheIntegrations> {
        self.environments.get_mut(env)
    }

    pub fn get(&self, env: &Environments) -> Option<&CacheIntegrations> {
        self.environments.get(env)
    }
}
