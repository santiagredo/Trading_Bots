use models::structs::Configuration;

use crate::{
    handler::Configurations,
    utils::{Cache, Core, Data, Response},
};

impl Configurations<Core> {
    pub async fn insert_configuration_core(self) -> Result<Configuration, Response> {
        let config = self.next_phase().insert_configuration_data()?;

        Ok(Configurations::<Cache>::set_configuration_cache(config).await)
    }

    pub async fn select_configuration_core(self) -> Configuration {
        if let Some(configuration) = Configurations::<Cache>::get_configuration_cache().await {
            return configuration;
        }

        let configuration = Configurations::<Data>::select_configuration_data();

        Configurations::<Cache>::set_configuration_cache(configuration).await
    }
}
