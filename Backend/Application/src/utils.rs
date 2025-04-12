use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Data;

#[derive(Debug, Clone, Copy)]
pub struct Logic;

#[derive(Debug, Clone, Copy)]
pub struct Core;

pub struct Utils;

impl Utils {
    pub fn validate_empty_field(field: String, field_name: &str) -> Result<String, String> {
        match field.trim().is_empty() {
            true => Err(format!("{field_name} cannot be empty")),
            false => Ok(field.trim().to_lowercase()),
        }
    }
}

#[derive(Debug)]
pub enum OutcomeError<F, E> {
    Failure(F),
    Error(E),
}

pub type Outcome<S, F, E> = Result<S, OutcomeError<F, E>>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Conditions {
    pub get_all: bool,
    pub limit: i32,
    pub offset: i32,
}
