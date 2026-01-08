use crate::{
    core::select_pairs_core,
    models::{entities::pairs::Model, structs::PairRequest},
};

#[tauri::command]
pub async fn select_pairs(env: String, pair: Option<PairRequest>) -> Result<Vec<Model>, String> {
    select_pairs_core(env, pair).await
}
