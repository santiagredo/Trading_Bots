use std::collections::BTreeMap;

use crate::integration::select_health_check_integration;

pub async fn select_health_check_core() -> Result<BTreeMap<String, String>, String> {
    match select_health_check_integration().await {
        Err(err) => Err(err.to_string()),
        Ok(val) => {
            let json = val.json().await.unwrap_or_default();

            Ok(json)
        }
    }
}
