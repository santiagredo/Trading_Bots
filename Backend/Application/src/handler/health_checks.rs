use crate::utils::Response;
use std::collections::BTreeMap;

pub struct HealthCheck;

impl HealthCheck {
    pub async fn select_health_check() -> Result<BTreeMap<String, String>, Response> {
        HealthCheck::select_health_check_core().await
    }
}
