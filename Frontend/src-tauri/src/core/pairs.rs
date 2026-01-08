use crate::{
    integration::select_pairs_integration,
    models::{entities::pairs::Model, structs::PairRequest},
    utils::handle_response,
};

pub async fn select_pairs_core(
    env: String,
    pair: Option<PairRequest>,
) -> Result<Vec<Model>, String> {
    let response = select_pairs_integration(env, pair).await?;

    handle_response::<Vec<Model>>(response).await
}
