use models::structs::Configuration;

use crate::handler::Configurations;
use crate::static_strings::{DEV_DATABASE_URL, PROD_DATABASE_URL};

impl Configurations {
    pub fn select() -> Configuration {
        Configuration {
            dev_database_url: DEV_DATABASE_URL.to_owned(),
            prod_database_url: PROD_DATABASE_URL.to_owned(),
        }
    }
}
