use std::fmt::Display;

use super::{DefaultError, ErrorResponse, ErrorType};

impl ErrorResponse {
    pub fn invalid_input<A: Display>(title: A) -> Self {
        let inner = |title: String| {
            let error = DefaultError {
                title,
                error_type: ErrorType::InvalidInput,
            };
            Self::BadRequest(error)
        };
        inner(format!("{}", title))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok_invalid_input() {
        let err = ErrorResponse::invalid_input("test-error");
        println!("{:?}", err);
    }
}
