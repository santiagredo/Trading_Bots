use chrono::{Local, NaiveDateTime};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    entities::integration_settings::Model,
    enums::{LifecycleState, TimestampedState},
    structs::Environments,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheIntegrationSetting {
    pub models: HashMap<i32, Model>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheIntegrationsSettings {
    pub integrations_map: HashMap<i32, CacheIntegrationSetting>,
    pub startup_date: NaiveDateTime,
    pub last_update_date: NaiveDateTime,
    pub status: LifecycleState,
}

impl CacheIntegrationsSettings {
    pub fn new() -> CacheIntegrationsSettings {
        let now = Local::now().naive_local();

        CacheIntegrationsSettings {
            integrations_map: HashMap::new(),
            startup_date: now,
            last_update_date: now,
            status: LifecycleState::Off,
        }
    }
}

impl TimestampedState for CacheIntegrationsSettings {
    type State = LifecycleState;

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.status
    }

    fn last_update_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.last_update_date
    }
}

#[derive(Default)]
pub struct CacheIntegrationsSettingsEnvironments {
    pub environments: HashMap<Environments, CacheIntegrationsSettings>,
}

impl CacheIntegrationsSettingsEnvironments {
    pub fn new() -> CacheIntegrationsSettingsEnvironments {
        CacheIntegrationsSettingsEnvironments {
            environments: HashMap::from([
                (Environments::DEV, CacheIntegrationsSettings::new()),
                (Environments::PROD, CacheIntegrationsSettings::new()),
            ]),
        }
    }
}

impl CacheIntegrationsSettingsEnvironments {
    pub fn get_or_create(&mut self, env: Environments) -> &mut CacheIntegrationsSettings {
        self.environments
            .entry(env)
            .or_insert_with(CacheIntegrationsSettings::new)
    }

    pub fn get_mut(&mut self, env: &Environments) -> Option<&mut CacheIntegrationsSettings> {
        self.environments.get_mut(env)
    }

    pub fn get(&self, env: &Environments) -> Option<&CacheIntegrationsSettings> {
        self.environments.get(env)
    }
}
