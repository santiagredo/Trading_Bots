use reqwest::{Client, Error, RequestBuilder, Response};

use crate::{handler::Binance, static_strings::EXCHANGE_INFORMATION_ENDPOINT};

impl<R> Binance<R> {
    pub async fn send_account_request(&self, request: RequestBuilder) -> Result<Response, Error> {
        request.send().await
    }

    pub async fn send_exchange_information_request(&self) -> Result<Response, Error> {
        Client::new()
            .get(EXCHANGE_INFORMATION_ENDPOINT)
            .send()
            .await
    }

    pub async fn send_new_order_request(&self, request: RequestBuilder) -> Result<Response, Error> {
        request.send().await
    }
}
