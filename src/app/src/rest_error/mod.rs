mod bad_request;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseError<T> {
    #[serde(rename = "type")]
    error_type: String,
    title: String,
    detail: Option<String>,
    extensions: Option<T>,
}

pub enum RestError {
    BadRequest(String),
    InternalServerError(String),
}

impl IntoResponse for RestError {
    fn into_response(self) -> axum::response::Response {
        let tuple = match self {
            Self::BadRequest(err) => (StatusCode::BAD_REQUEST, Json(err)),
            Self::InternalServerError(err) => (StatusCode::BAD_REQUEST, Json(err)),
        };
        tuple.into_response()
    }
}
