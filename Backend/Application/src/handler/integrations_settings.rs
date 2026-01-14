use std::{collections::HashMap, marker::PhantomData};

use models::{
    entities::{self, integration_settings::Model},
    enums::LifecycleState,
    structs::{CacheIntegrationsSettings, Environments, IntegrationSettingRequest, QueryOptions},
};

use crate::utils::{Core, Response, Types};

#[derive(Debug, Default)]
pub struct IntegrationsSettings<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
    pub model: IntegrationSettingRequest,
}

impl<Phase> IntegrationsSettings<Phase> {
    pub fn next_phase<Next>(self) -> IntegrationsSettings<Next> {
        IntegrationsSettings {
            phase: PhantomData::<Next>,
            environment: self.environment,
            model: self.model,
        }
    }
}

impl IntegrationsSettings {
    pub fn new(model: IntegrationSettingRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model: IntegrationSettingRequest {
                ..Default::default()
            },
        }
    }

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: self.phase,
            environment,
            model: self.model,
        }
    }

    pub async fn select_integrations_settings(
        self,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.next_phase()
            .select_integrations_settings_core(query)
            .await
    }

    pub async fn update_integration_setting(self) -> Result<Model, Response> {
        self.next_phase().update_integration_setting_core().await
    }

    // cache
    pub async fn get_integrations_settings(self) -> Option<CacheIntegrationsSettings> {
        self.next_phase().get_integrations_settings_core().await
    }

    pub async fn get_integration_settings(self) -> Option<HashMap<i32, Model>> {
        self.next_phase().get_integration_settings_core().await
    }

    pub async fn get_integrations_settings_state(self) -> LifecycleState {
        self.next_phase()
            .get_integrations_settings_state_core()
            .await
    }

    pub async fn start_integrations_settings(self) -> Result<(), Response> {
        self.next_phase().start_integrations_settings_core().await
    }

    pub async fn stop_integrations_settings(self) -> Result<(), Response> {
        self.next_phase().stop_integrations_settings_core().await
    }

    pub async fn reset_integrations_settings(self) -> Result<(), Response> {
        self.next_phase().reset_integrations_settings_core().await
    }

    pub async fn resolve_integration_settings(
        self,
    ) -> Result<Vec<entities::integration_settings::Model>, Response> {
        self.next_phase().resolve_integration_settings_core().await
    }

    pub fn resolve_setting_value(
        settings: &[entities::integration_settings::Model],
        nick: &str,
    ) -> Result<String, Response> {
        IntegrationsSettings::<Core>::resolve_setting_value_core(settings, nick)
    }
}
