mod api;

use axum::{routing::get, Router};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const SERVER_HOST: &'static str = env!("SERVER_HOST");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .nest("/", api::create_todo::builder());
    let listener = tokio::net::TcpListener::bind(SERVER_HOST).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    format!("todo app: ver {}", VERSION)
}
