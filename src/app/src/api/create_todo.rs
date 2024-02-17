use std::sync::Arc;

use axum::{routing::post, Extension, Router};
use todo_usecase::interactor::{create_todo::CreateTodoInput, Interactor};

pub fn builder() -> Router {
    Router::new().route("/todo", post(create_todo))
}

async fn create_todo(Extension(interactor): Extension<Arc<Interactor>>) -> String {
    let input = CreateTodoInput {
        content: "hogehoge".into(),
    };
    let result = interactor.create_todo(input).await;
    println!("{:?}", result);
    format!("create-todo")
}
