use actix_web::{http::StatusCode, HttpResponse};
use sea_orm::{DbErr, SqlErr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Data;

#[derive(Debug, Clone, Copy)]
pub struct Logic;

#[derive(Debug, Clone, Copy)]
pub struct Core;

#[derive(Debug, Clone, Copy)]
pub struct Types;

#[derive(Debug, Clone, Copy)]
pub struct Integration;

#[derive(Debug, Clone, Copy)]
pub struct Cache;

pub struct Utils;

impl Utils {
    pub fn validate_empty_field(field: String, field_name: &str) -> Result<String, String> {
        match field.trim().is_empty() {
            true => Err(format!("{field_name} cannot be empty")),
            false => Ok(field.trim().to_lowercase()),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Conditions {
    pub get_all: bool,
    pub limit: i32,
    pub offset: i32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Response {
    pub code: u16,
    pub message: String,
}

impl Response {
    pub fn bad_request(item: String) -> Response {
        Response {
            code: 400,
            message: format!("Bad request: {item}"),
        }
    }

    pub fn not_found(item: String) -> Response {
        Response {
            code: 404,
            message: format!("{item} not found"),
        }
    }

    pub fn server_error(item: String) -> Response {
        Response {
            code: 500,
            message: format!("Server error: {item}"),
        }
    }
}

pub fn error_response(response: Response) -> HttpResponse {
    HttpResponse::build(
        StatusCode::from_u16(response.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
    )
    .json(Response {
        code: response.code,
        message: response.message,
    })
}

pub fn handle_db_error(err: &DbErr) -> Response {
    if let Some(sql_err) = err.sql_err() {
        match sql_err {
            SqlErr::UniqueConstraintViolation(msg) => Response {
                code: 400,
                message: format!("Unique constraint violation: {msg}"),
            },
            SqlErr::ForeignKeyConstraintViolation(msg) => Response {
                code: 400,
                message: format!("Foreign key constraint violation: {msg}"),
            },
            _ => Response {
                code: 500,
                message: format!("Unexpected SQL error"),
            },
        }
    } else {
        Response {
            code: 500,
            message: format!("Database error"),
        }
    }
}

pub fn handle_user_err(err: String) -> Response {
    Response {
        code: 400,
        message: err,
    }
}

#[macro_export]
macro_rules! log_db_error {
    ($self:expr, $err:expr) => {{
        let error_log_request = ErrorLogRequest {
            file_path: Some(file!().to_string()),
            line_number: Some(line!().to_string()),
            function_name: Some(function_name!().to_string()),
            request: Some(serde_json::to_string(&$self.model).unwrap_or_default()),
            error_type: Some(format!("{:?}", $err)),
            error_details: Some($err.to_string()),
            ..Default::default()
        };

        let environment = $self.environment;

        tokio::spawn(async move {
            let _ = ErrorLogs::new(&environment, error_log_request)
                .insert_log()
                .await;
        });

        Err(crate::utils::handle_db_error(&$err))
    }};
}

pub fn trim_option(value: &mut Option<String>) {
    if let Some(v) = value {
        *v = v.trim().to_owned();
    }
}
