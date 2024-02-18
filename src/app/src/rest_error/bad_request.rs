use std::fmt::Debug;

use axum::http::StatusCode;
use enum_display::EnumDisplay;

use crate::api::Operation;

use super::{BaseError, RestError};

#[derive(Debug)]
pub struct BadRequestError {
    operation: Operation,
    sub_error: BadRequestType,
    title: String,
}

#[derive(Debug, EnumDisplay)]
enum BadRequestType {
    Validation,
}

impl BadRequestError {
    pub fn validation<A: Debug>(operation: Operation) -> impl FnOnce(A) -> Self {
        |title| Self {
            operation,
            title: format!("{:?}", title),
            sub_error: BadRequestType::Validation,
        }
    }
}

impl BaseError for BadRequestError {
    fn error_type(&self) -> String {
        format!("{}/{}", self.operation, self.sub_error)
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
