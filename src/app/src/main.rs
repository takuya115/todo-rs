mod api;
mod config;
mod error;
mod gateway;

use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use config::Config;
use gateway::GatewayImpl;
use todo_usecase::interactor::Interactor;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tracing::instrument]
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let config = Config::from_env();
    let gateway = GatewayImpl::build(&config);
    let interactor = Arc::new(Interactor {
        gateway: Box::new(gateway),
    });
    let app = Router::new()
        .route("/", get(root))
        .nest("/", api::create_task::router())
        .layer(Extension(interactor))
        .layer(
            // 入力と応答をログ出力
            TraceLayer::new_for_http()
                .on_request(
                    |req: &axum::http::Request<axum::body::Body>, _span: &tracing::Span| {
                        info!("[app] reqest={:?}", req);
                    },
                )
                .on_response(
                    |res: &axum::http::Response<axum::body::Body>,
                     _latency: std::time::Duration,
                     _span: &tracing::Span| {
                        info!("[app] response={:?}", res);
                    },
                ),
        );
    let listener = tokio::net::TcpListener::bind(&config.server_host)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    format!("todo app: ver {}", env!("CARGO_PKG_VERSION"))
}
