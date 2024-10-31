use std::net::SocketAddr;

use axum::{extract::Request, middleware::Next, response::Response};
use http::header::AUTHORIZATION;

use crate::pkg::util::identity::Identity;

pub async fn handle(mut request: Request, next: Next) -> Response {
    let token = request.headers().get(AUTHORIZATION);
    let identity = match token {
        None => Identity::empty(),
        Some(v) => match v.to_str() {
            Ok(v) => Identity::from_auth_token(v.to_string()),
            Err(e) => {
                tracing::error!(error = ?e, "error get header(authorization)");
                Identity::empty()
            }
        },
    };
    request.extensions_mut().insert(identity);

    let ip = request
        .extensions()
        .get::<SocketAddr>()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    request.extensions_mut().insert(ip);

    next.run(request).await
}
