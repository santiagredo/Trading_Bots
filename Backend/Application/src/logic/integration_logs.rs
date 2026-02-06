use models::structs::IntegrationLogRequest;
use reqwest::{Error, Response};

use crate::utils;

pub async fn map_request_result(
    result: Result<Response, Error>,
    integration_log: &mut IntegrationLogRequest,
) -> Result<String, utils::Response> {
    match result {
        Err(err) => {
            let status_code = err.status().map_or(500, |s| s.as_u16());
            integration_log.with_status_code(status_code.try_into().unwrap_or(-1));

            let error_message = err.to_string();
            integration_log.with_error_message(error_message.clone());

            let response = utils::Response {
                code: status_code,
                message: error_message,
            };

            return Err(response);
        }
        Ok(response) => {
            let body = response.text().await.unwrap_or_default();
            integration_log.with_response(body.clone());

            return Ok(body);
        }
    }
}

pub fn map_serde_result<T>(
    result: Result<T, serde_json::Error>,
    integration_log: &mut IntegrationLogRequest,
) -> Result<T, utils::Response> {
    match result {
        Err(err) => {
            let status_code = 500;
            integration_log.with_status_code(status_code.try_into().unwrap_or(-1));

            let error_message = err.to_string();
            integration_log.with_error_message(error_message.clone());

            Err(utils::Response {
                code: status_code,
                message: error_message,
            })
        }
        Ok(response) => Ok(response),
    }
}
