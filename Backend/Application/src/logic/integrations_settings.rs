use models::{entities, structs::IntegrationSettingRequest};

pub fn validate_update(req: &IntegrationSettingRequest) -> Result<(), String> {
    if req.id.is_none_or(|id| id <= 0) {
        return Err(format!("Invalid integration setting ID"));
    }

    if req.value.as_ref().is_none_or(|val| val.is_empty()) {
        return Err(format!("Invalid Value property"));
    }

    Ok(())
}

pub fn resolve_setting_value(
    settings: &[entities::integration_settings::Model],
    nick: &str,
) -> Result<String, String> {
    settings
        .iter()
        .find(|s| s.nick == nick)
        .map(|s| s.value.clone())
        .ok_or(format!("Integration setting '{}' not found", nick))
}
