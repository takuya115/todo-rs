use super::{BaseError, RestError};

pub struct BadRequestError {
    pub operation: String,
    pub title: String,
}

impl BaseError for BadRequestError {
    fn error_type(&self) -> String {
        format!("{}/BadRequest", self.operation)
    }

    fn title(&self) -> String {
        self.title.clone()
    }
}

impl From<BadRequestError> for RestError {
    fn from(value: BadRequestError) -> Self {
        Self::BadRequest(value.to_json())
    }
}
