pub mod api;
pub mod cmd;
pub mod middleware;
pub mod model;
pub mod router;
pub mod service;
use crate::pkg::core::config;

pub async fn serve() {
    let addr = config::global().get_int("app.port").unwrap_or(8000);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", addr))
        .await
        .unwrap();

    tracing::info!("listening on {}", addr);

    axum::serve(listener, router::init()).await.unwrap();
}
