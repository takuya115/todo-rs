use std::fmt::Debug;

use super::{BaseError, RestError};
use axum::http::StatusCode;
use enum_display::EnumDisplay;

#[derive(Debug)]
pub struct BadRequestError {
    error_type: ErrorType,
    title: String,
}

#[derive(Debug, EnumDisplay)]
enum ErrorType {
    Validation,
}

impl BadRequestError {
    pub fn validation<A: Debug>(title: A) -> Self {
        Self {
            title: format!("{:?}", title),
            error_type: ErrorType::Validation,
        }
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
