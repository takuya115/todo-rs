mod bad_request;

use std::collections::HashMap;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};

pub trait BaseError: Into<RestError> {
    /// エラータイプ
    fn error_type(&self) -> String;
    /// タイトル
    fn title(&self) -> String;
    /// 追加要素
    fn extends(&self) -> HashMap<String, Value> {
        HashMap::new()
    }
    /// json化
    fn to_json(&self) -> Value {
        let mut json = self.extends();
        json.insert("type".into(), self.error_type().into());
        json.insert("title".into(), self.title().into());
        json!(json)
    }
}

pub enum RestError {
    BadRequest(Value),
    InternalServerError(Value),
}

impl RestError {
    fn to_json(&self) -> Json<Value> {
        match self {
            Self::BadRequest(v) => Json(v.to_owned()),
            Self::InternalServerError(v) => Json(v.to_owned()),
        }
    }
}

impl IntoResponse for RestError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, self.to_json()).into_response()
    }
}
