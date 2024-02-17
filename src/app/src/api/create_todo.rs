use std::{str::FromStr, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use todo_model::Text;
use todo_usecase::{
    error::Error,
    interactor::{create_todo::CreateTodoInput, Interactor},
};

pub fn builder() -> Router {
    Router::new().route("/todo", post(create_todo))
}

#[derive(Debug, Deserialize)]
pub struct CreateToDoBody {
    content: String,
}

async fn create_todo(
    Extension(interactor): Extension<Arc<Interactor>>,
    Json(body): Json<CreateToDoBody>,
) -> Result<(), BadRequestError> {
    let input = to_input(body).map_err(|err| BadRequestError {
        error_type: "invalid input".into(),
        message: format!("{:?}", err),
    })?;
    let result = interactor.create_todo(input).await;
    println!("{:?}", result);
    format!("create-todo");
    Ok(())
}

fn to_input(value: CreateToDoBody) -> Result<CreateTodoInput, Error> {
    Ok(CreateTodoInput {
        content: Text::from_str(&value.content).map_err(Error::invalid_input)?,
    })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BadRequestError {
    #[serde(rename = "type")]
    error_type: String,
    message: String,
}

impl IntoResponse for BadRequestError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(self);
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}
