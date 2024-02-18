mod bad_request;
pub use bad_request::BadRequestError;

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
    fn to_json(&self) -> Json<Value> {
        let mut json = self.extends();
        json.insert("type".into(), self.error_type().into());
        json.insert("title".into(), self.title().into());
        Json(json!(json))
    }
}

type ErrorBody = Json<Value>;
pub struct RestError(StatusCode, ErrorBody);

impl IntoResponse for RestError {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}
