use crate::{
    integration::select_assets_integration,
    models::{entities::assets::Model, structs::AssetRequest},
    utils::handle_response,
};

pub async fn select_assets_core(env: String, asset: Option<AssetRequest>) -> Result<Vec<Model>, String> {
    let response = select_assets_integration(env, asset).await?;

    handle_response::<Vec<Model>>(response).await
}
