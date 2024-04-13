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
    error::Error,
    interactor::{create_todo::CreateTodoInput, Interactor},
};

use crate::error::{BadRequestError, InternalServerError, RestError};

pub fn builder() -> Router {
    Router::new().route("/todo", post(create_todo))
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

async fn create_todo(
    Extension(interactor): Extension<Arc<Interactor>>,
    Json(body): Json<RequestBody>,
) -> Result<Response> {
    let input = to_input(body).map_err(to_rest_error)?;
    let result = interactor.create_todo(input).await.map_err(to_rest_error)?;
    println!("{:?}", result);
    let response = ResponseBody {
        id: result.id.to_string(),
        task: result.content.to_string(),
    };

    Ok((StatusCode::OK, Json(response)).into_response())
}

fn to_input(value: RequestBody) -> Result<CreateTodoInput, Error> {
    Ok(CreateTodoInput {
        content: TaskBody::from_str(&value.task).map_err(Error::invalid_input)?,
    })
}

fn to_rest_error(err: todo_usecase::error::Error) -> RestError {
    match err {
        todo_usecase::error::Error::InvalidInput(err) => BadRequestError::validation(err).into(),
        _ => InternalServerError::unexpected(err).into(),
    }
}
