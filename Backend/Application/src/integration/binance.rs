use reqwest::{Client, Error, RequestBuilder, Response};

use crate::{handler::Binance, static_strings::EXCHANGE_INFORMATION_ENDPOINT, utils::Integration};

impl Binance<Integration> {
    pub async fn get_account_integration(self, request: RequestBuilder) -> Result<Response, Error> {
        request.send().await
    }

    pub async fn get_exchange_information_integration(self) -> Result<Response, Error> {
        Client::new()
            .get(EXCHANGE_INFORMATION_ENDPOINT)
            .send()
            .await
    }

    pub async fn post_new_order_integration(
        self,
        request: RequestBuilder,
    ) -> Result<Response, Error> {
        request.send().await
    }
}
