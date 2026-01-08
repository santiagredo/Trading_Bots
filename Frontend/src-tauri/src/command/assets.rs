use crate::{
    core::select_assets_core,
    models::{entities::assets::Model, structs::AssetRequest},
};

#[tauri::command]
pub async fn select_assets(env: String, asset: Option<AssetRequest>) -> Result<Vec<Model>, String> {
    select_assets_core(env, asset).await
}
