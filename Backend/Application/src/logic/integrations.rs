use crate::utils::Utils;
use models::structs::IntegrationRequest;

pub fn validate_insert(req: &IntegrationRequest) -> Result<(), String> {
    let _ = Utils::validate_empty_field(req.name.clone().unwrap_or_default(), "Integration name")?
        .to_uppercase();

    let _ = Utils::validate_empty_field(req.code.clone().unwrap_or_default(), "Integration code")?
        .to_uppercase();

    Ok(())
}

pub fn validate_update(req: &IntegrationRequest) -> Result<(), String> {
    if req.id.is_none_or(|id| id <= 0) {
        return Err(format!("Invalid integration ID"));
    }

    let _ = Utils::validate_empty_field(req.name.clone().unwrap_or_default(), "Integration name")?
        .to_uppercase();

    let _ = Utils::validate_empty_field(req.code.clone().unwrap_or_default(), "Integration code")?
        .to_uppercase();

    Ok(())
}
