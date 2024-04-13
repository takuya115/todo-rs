use std::fmt::Debug;

use super::{BaseError, RestError};
use axum::http::StatusCode;
use strum::Display;

#[derive(Debug)]
pub struct BadRequestError {
    error_type: ErrorType,
    title: String,
}

#[derive(Debug, Display)]
enum ErrorType {
    Validation,
}

impl BadRequestError {
    pub fn validation<A: Debug>(title: A) -> Self {
        let inner = |title: String, error_type: ErrorType| Self { title, error_type };
        inner(format!("{:?}", title), ErrorType::Validation)
    }
}

impl BaseError for BadRequestError {
    fn error_type(&self) -> String {
        format!("{}", self.error_type)
    }

    fn title(&self) -> String {
        self.title.clone()
    }
}

impl From<BadRequestError> for RestError {
    fn from(value: BadRequestError) -> Self {
        Self(StatusCode::BAD_REQUEST, value.to_json())
    }
}
