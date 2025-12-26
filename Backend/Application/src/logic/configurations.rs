use crate::{handler::Configurations, utils::Logic};

impl Configurations<Logic> {
    pub fn insert_configuration_logic(self) -> Result<Self, String> {
        Ok(self)
    }
}
