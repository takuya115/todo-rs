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
    InvalidInput,
}

impl BadRequestError {
    pub fn invalid_input<A: Debug>(title: A) -> Self {
        let inner = |title: String, error_type: ErrorType| Self { title, error_type };
        inner(format!("{:?}", title), ErrorType::InvalidInput)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok_invalid_input() {
        let err = BadRequestError::invalid_input(todo_usecase::Error::invalid_input("test-error"));
        println!("{:?}", err)
    }
}
