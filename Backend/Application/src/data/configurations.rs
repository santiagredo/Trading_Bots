use models::structs::Configuration;

use crate::static_strings::{DEV_DATABASE_URL, PROD_DATABASE_URL};
use crate::{handler::Configurations, utils::Data};

impl Configurations<Data> {
    pub fn select_configuration_data() -> Configuration {
        Configuration {
            dev_database_url: DEV_DATABASE_URL.to_owned(),
            prod_database_url: PROD_DATABASE_URL.to_owned(),
        }
    }
}
