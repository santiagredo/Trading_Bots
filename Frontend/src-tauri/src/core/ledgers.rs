use crate::{
    integration::{
        insert_ledger_integration, select_ledger_integration, select_ledgers_integration,
    },
    models::{entities::ledgers::Model, structs::LedgerRequest},
    utils::handle_response,
};

// db
pub async fn insert_ledger_core(env: String, ledger: LedgerRequest) -> Result<Model, String> {
    let response = insert_ledger_integration(env, ledger).await?;
    handle_response::<Model>(response).await
}

pub async fn select_ledger_core(env: String, query: LedgerRequest) -> Result<Model, String> {
    let response = select_ledger_integration(env, query).await?;
    handle_response::<Model>(response).await
}

pub async fn select_ledgers_core(env: String, query: LedgerRequest) -> Result<Vec<Model>, String> {
    let response = select_ledgers_integration(env, query).await?;
    handle_response::<Vec<Model>>(response).await
}
