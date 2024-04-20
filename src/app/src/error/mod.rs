mod s400_bad_request;
mod s500_internal_server_error;

use serde::Serialize;
use strum::Display;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

#[derive(Debug, Display, Serialize)]
enum ErrorType {
    Unexpected,
    InvalidInput,
}

#[derive(Debug, Serialize)]
pub struct DefaultError {
    #[serde(rename = "type")]
    error_type: ErrorType,
    title: String,
}

impl DefaultError {
    /// json化
    fn to_json(&self) -> Json<Value> {
        Json(json!(self))
    }
}

#[derive(Debug)]
pub enum ErrorResponse {
    BadRequest(DefaultError),
    Internal(DefaultError),
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::BadRequest(err) => (StatusCode::BAD_REQUEST, err.to_json()),
            Self::Internal(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_json()),
        }
        .into_response()
    }
}
