use std::marker::PhantomData;

use models::structs::{Configuration, ConfigurationRequest};

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct Configurations<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: ConfigurationRequest,
}

impl<Phase> Configurations<Phase> {
    pub fn next_phase<Next>(self) -> Configurations<Next> {
        Configurations {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl Configurations {
    pub fn new(model: ConfigurationRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: ConfigurationRequest {
                ..Default::default()
            },
        }
    }

    pub async fn insert_configuration(self) -> Result<Configuration, Response> {
        self.next_phase().insert_configuration_core().await
    }

    pub async fn select_configuration(self) -> Configuration {
        self.next_phase().select_configuration_core().await
    }
}
