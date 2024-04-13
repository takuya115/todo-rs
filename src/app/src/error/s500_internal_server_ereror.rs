use std::fmt::Debug;

use super::{BaseError, RestError};
use axum::http::StatusCode;
use strum::Display;

#[derive(Debug)]
pub struct InternalServerError {
    error_type: ErrorType,
    title: String,
}

#[derive(Debug, Display)]
enum ErrorType {
    Unexpected,
}

impl InternalServerError {
    pub fn unexpected<A: Debug>(title: A) -> Self {
        let inner = |title: String, error_type: ErrorType| Self { title, error_type };
        inner(format!("{:?}", title), ErrorType::Unexpected)
    }
}

impl BaseError for InternalServerError {
    fn error_type(&self) -> String {
        format!("{}", self.error_type)
    }

    fn title(&self) -> String {
        self.title.clone()
    }
}

impl From<InternalServerError> for RestError {
    fn from(value: InternalServerError) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, value.to_json())
    }
}
