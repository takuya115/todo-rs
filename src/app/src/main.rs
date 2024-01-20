mod api;
mod config;

use axum::{routing::get, Router};
use config::Config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::from_env();
    let app = Router::new()
        .route("/", get(root))
        .nest("/", api::create_todo::builder());
    let listener = tokio::net::TcpListener::bind(&config.server_host)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    format!("todo app: ver {}", env!("CARGO_PKG_VERSION"))
}
