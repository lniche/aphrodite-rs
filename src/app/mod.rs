pub mod api;
pub mod cmd;
pub mod middleware;
pub mod model;
pub mod router;
pub mod service;
use crate::pkg::core::config;

pub async fn serve() {
    let port = config::global().get_int("app.port").unwrap_or(8000);
    let host = config::global()
        .get_string("app.host")
        .unwrap_or("0.0.0.0".to_string());
    let host_port = format!("{}:{}", host, port);
    let docs_addr = format!("{}/swagger-ui/index.html", host_port);
    tracing::info!("server start host = http://{}", host_port);
    tracing::info!("docs addr = http://{}", docs_addr);

    let listener = tokio::net::TcpListener::bind(host_port).await.unwrap();
    axum::serve(listener, router::init()).await.unwrap();
}
