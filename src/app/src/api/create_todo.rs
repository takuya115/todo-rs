use std::{str::FromStr, sync::Arc};

use axum::{routing::post, Extension, Json, Router};
use serde::Deserialize;
use todo_model::Text;
use todo_usecase::{
    error::Error,
    interactor::{create_todo::CreateTodoInput, Interactor},
};

use crate::{
    api::Operation,
    rest_error::{BadRequestError, RestError},
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
) -> Result<(), RestError> {
    let input = to_input(body).map_err(BadRequestError::validation(Operation::CreateTodo))?;
    let result = interactor.create_todo(input).await;
    println!("{:?}", result);
    Ok(())
}

fn to_input(value: CreateToDoBody) -> Result<CreateTodoInput, Error> {
    Ok(CreateTodoInput {
        content: Text::from_str(&value.content).map_err(Error::invalid_input)?,
    })
}
