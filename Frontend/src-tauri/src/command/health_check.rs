use crate::core::select_health_check_core;
use std::collections::BTreeMap;

#[tauri::command]
pub async fn select_health_check() -> Result<BTreeMap<String, String>, String> {
    select_health_check_core().await
}
