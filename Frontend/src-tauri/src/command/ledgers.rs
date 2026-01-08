use crate::{
    core::{insert_ledger_core, select_ledger_core, select_ledgers_core},
    models::{entities::ledgers::Model, structs::LedgerRequest},
};

// db
#[tauri::command]
pub async fn insert_ledger(env: String, ledger: LedgerRequest) -> Result<Model, String> {
    insert_ledger_core(env, ledger).await
}

#[tauri::command]
pub async fn select_ledger(env: String, query: LedgerRequest) -> Result<Model, String> {
    select_ledger_core(env, query).await
}

#[tauri::command]
pub async fn select_ledgers(env: String, query: LedgerRequest) -> Result<Vec<Model>, String> {
    select_ledgers_core(env, query).await
}
