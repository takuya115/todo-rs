use std::fmt::Display;

use super::{DefaultError, ErrorResponse, ErrorType};

impl ErrorResponse {
    pub fn internal<A: Display>(title: A) -> Self {
        let inner = |title: String| {
            let error = DefaultError {
                title,
                error_type: ErrorType::Unexpected,
            };
            Self::Internal(error)
        };
        inner(format!("{}", title))
    }
}
