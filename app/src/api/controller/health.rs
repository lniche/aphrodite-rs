use axum::http::StatusCode;

#[utoipa::path(
    get,
    path = "/ping",
    responses(
        (status = 200, description = "Health check passed"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "health",
)]
pub async fn ping() -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}
