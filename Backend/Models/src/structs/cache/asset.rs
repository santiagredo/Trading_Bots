use chrono::{Local, NaiveDateTime};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    entities::assets::Model,
    enums::{LifecycleState, TimestampedState},
    structs::Environments,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheAssets {
    pub models: HashMap<i32, Model>,
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl CacheAssets {
    pub fn new() -> CacheAssets {
        let now = Local::now().naive_local();

        CacheAssets {
            models: HashMap::new(),
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Off,
        }
    }
}

impl TimestampedState for CacheAssets {
    type State = LifecycleState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.status
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}

#[derive(Default)]
pub struct CacheAssetsEnvironments {
    pub environments: HashMap<Environments, CacheAssets>,
}

impl CacheAssetsEnvironments {
    pub fn new() -> CacheAssetsEnvironments {
        CacheAssetsEnvironments {
            environments: HashMap::from([
                (Environments::DEV, CacheAssets::new()),
                (Environments::PROD, CacheAssets::new()),
            ]),
        }
    }
}

impl CacheAssetsEnvironments {
    pub fn get_or_create(&mut self, env: Environments) -> &mut CacheAssets {
        self.environments
            .entry(env)
            .or_insert_with(CacheAssets::new)
    }

    pub fn get_mut(&mut self, env: &Environments) -> Option<&mut CacheAssets> {
        self.environments.get_mut(env)
    }

    pub fn get(&self, env: &Environments) -> Option<&CacheAssets> {
        self.environments.get(env)
    }
}
