use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct IntegrationLogRequest {
    pub id: Option<i32>,
    pub creation_date: Option<NaiveDateTime>,
    pub integration_name: Option<String>,
    pub function_name: Option<String>,
    pub url: Option<String>,
    pub request: Option<String>,
    pub response: Option<String>,
    pub status_code: Option<i32>,
    pub error_message: Option<String>,
    pub execution_time_ms: Option<i32>,
}

impl IntegrationLogRequest {
    pub fn new(
        integration_name: String,
        function_name: String,
        url: String,
    ) -> IntegrationLogRequest {
        IntegrationLogRequest {
            id: None,
            creation_date: None,
            integration_name: Some(integration_name),
            function_name: Some(function_name),
            url: Some(url),
            ..Default::default()
        }
    }

    pub fn with_request(&mut self, request: String) {
        self.request = Some(request)
    }

    pub fn with_response(&mut self, response: String) {
        self.response = Some(response)
    }

    pub fn with_status_code(&mut self, status_code: i32) {
        self.status_code = Some(status_code)
    }

    pub fn with_error_message(&mut self, error_message: String) {
        self.error_message = Some(error_message)
    }

    pub fn with_execution_time_ms(&mut self, execution_time_ms: i32) {
        self.execution_time_ms = Some(execution_time_ms)
    }
}
