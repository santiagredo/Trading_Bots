use models::entities;

use crate::{handler::IntegrationsSettings, utils::Logic};

impl IntegrationsSettings<Logic> {
    pub fn update_integration_setting_logic(self) -> Result<Self, String> {
        if self.model.id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid integration setting ID"));
        }

        if self.model.value.as_ref().is_none_or(|val| val.is_empty()) {
            return Err(format!("Invalid Value property"));
        }

        Ok(self)
    }

    pub fn resolve_setting_value_logic(
        settings: &[entities::integration_settings::Model],
        nick: &str,
    ) -> Result<String, String> {
        settings
            .iter()
            .find(|s| s.nick == nick)
            .map(|s| s.value.clone())
            .ok_or(format!("Integration setting '{}' not found", nick))
    }
}
