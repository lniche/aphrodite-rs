use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::pkg::util::identity::Identity;
use crate::pkg::result::response::ApiErr;

use crate::app::middleware::auth_check;

pub async fn handle(request: Request, next: Next) -> Response {
    let identity = request.extensions().get::<Identity>();
    match identity {
        None => return ApiErr::ErrAuth(None).into_response(),
        Some(v) => match auth_check(v).await {
            Ok(_) => (),
            Err(e) => return ApiErr::ErrAuth(Some(e.to_string())).into_response(),
        },
    }
    next.run(request).await
}
