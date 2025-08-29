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
