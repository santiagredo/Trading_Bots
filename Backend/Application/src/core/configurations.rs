use models::structs::Configuration;

use crate::handler::Configurations;

impl Configurations {
    pub async fn select_configuration(self) -> Configuration {
        if let Some(configuration) = Configurations::get_configuration().await {
            return configuration;
        }

        let configuration = Configurations::select();

        Configurations::set_configuration(configuration).await
    }
}
