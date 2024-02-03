mod api;
mod config;

use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use config::Config;
// use todo_usecase::interactor::Interactor;

#[derive(Debug, Clone)]
pub struct CustomInteractor;
impl CustomInteractor {
    pub fn use_case(&self) -> String {
        "call use_case()".into()
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::from_env();
    let interactor = Arc::new(CustomInteractor);
    let app = Router::new()
        .route("/", get(root))
        .nest("/", api::create_todo::builder())
        .layer(Extension(interactor));
    let listener = tokio::net::TcpListener::bind(&config.server_host)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    format!("todo app: ver {}", env!("CARGO_PKG_VERSION"))
}
