use std::sync::Arc;

use axum::{routing::post, Extension, Router};
// use todo_usecase::interactor::Interactor;

use crate::CustomInteractor;

pub fn builder() -> Router {
    Router::new().route("/todo", post(create_todo))
}

async fn create_todo(Extension(modules): Extension<Arc<CustomInteractor>>) -> String {
    println!("{}", modules.use_case());
    format!("create-todo")
}
