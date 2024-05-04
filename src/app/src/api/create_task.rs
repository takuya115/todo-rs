use std::{str::FromStr, sync::Arc};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response, Result},
    routing::post,
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use todo_model::task::TaskBody;
use todo_usecase::{
    interactor::{create_task::CreateTaskInput, Interactor},
    Error,
};
use tracing::info;

use crate::error::ErrorResponse;

pub fn router() -> Router {
    Router::new().route("/todo", post(create_task))
}

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    task: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseBody {
    id: String,
    task: String,
}

async fn create_task(
    Extension(interactor): Extension<Arc<Interactor>>,
    Json(body): Json<RequestBody>,
) -> Result<Response> {
    info!("[create_task] accept");
    let input = to_input(body).map_err(to_rest_error)?;
    let result = interactor.create_task(input).await.map_err(to_rest_error)?;
    let response = ResponseBody {
        id: result.id.to_string(),
        task: result.content.to_string(),
    };

    Ok((StatusCode::OK, Json(response)).into_response())
}

fn to_input(value: RequestBody) -> Result<CreateTaskInput, Error> {
    Ok(CreateTaskInput {
        task: TaskBody::from_str(&value.task).map_err(Error::invalid_input)?,
    })
}

fn to_rest_error(err: todo_usecase::Error) -> ErrorResponse {
    match err {
        todo_usecase::Error::InvalidInput(..) => ErrorResponse::invalid_input(err),
        _ => ErrorResponse::internal(err),
    }
}
