use crate::{core::get_account_core, models::structs::AccountInformation};

#[tauri::command]
pub async fn get_account(env: String) -> Result<AccountInformation, String> {
    get_account_core(env).await
}
