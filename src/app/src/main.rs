mod api;
mod config;
mod gateway;
mod rest_error;

use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use config::Config;
use gateway::GatewayImpl;
use todo_usecase::interactor::Interactor;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::from_env();
    let gateway = GatewayImpl::build(&config);
    let interactor = Arc::new(Interactor {
        gateway: Box::new(gateway),
    });
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
