use crate::{
    integration::get_account_integration, models::structs::AccountInformation,
    utils::handle_response,
};

pub async fn get_account_core(env: String) -> Result<AccountInformation, String> {
    let response = get_account_integration(env).await?;
    handle_response::<AccountInformation>(response).await
}
