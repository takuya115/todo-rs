use axum::{routing::post, Router};

pub fn builder() -> Router {
    Router::new().route("/todo", post(create_todo))
}

async fn create_todo() -> String {
    format!("create-todo")
}
