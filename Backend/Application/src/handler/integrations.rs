use std::marker::PhantomData;

use models::{
    entities::integrations::Model,
    enums::LifecycleState,
    structs::{CacheIntegrations, Environments, IntegrationRequest},
};

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct Integrations<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
    pub model: IntegrationRequest,
}

impl<Phase> Integrations<Phase> {
    pub fn next_phase<Next>(self) -> Integrations<Next> {
        Integrations {
            phase: PhantomData::<Next>,
            environment: self.environment,
            model: self.model,
        }
    }
}

impl Integrations {
    pub fn new(model: IntegrationRequest) -> Self {
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
            model: IntegrationRequest {
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

    pub async fn select_integrations(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_integrations_core().await
    }

    pub async fn update_integration(self) -> Result<Model, Response> {
        self.next_phase().update_integration_core().await
    }

    // cache
    pub async fn get_integrations(self) -> Option<CacheIntegrations> {
        self.next_phase().get_integrations_core().await
    }

    pub async fn get_integration(self) -> Option<Model> {
        self.next_phase().get_integration_core().await
    }

    pub async fn get_integrations_state(self) -> LifecycleState {
        self.next_phase().get_integrations_state_core().await
    }

    pub async fn start_integrations(self) -> Result<(), Response> {
        self.next_phase().start_integrations_core().await
    }

    pub async fn stop_integrations(self) -> Result<(), Response> {
        self.next_phase().stop_integrations_core().await
    }

    pub async fn reset_integrations(self) -> Result<(), Response> {
        self.next_phase().reset_integrations_core().await
    }
}
