use axum::{
    routing::{get, post},
    Router,
};

use crate::app::api::{auth,user};

pub fn auth() -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .route("/send-code", post(auth::send_verify_code))
}

pub fn user() -> Router {
    Router::new()
        .route("/logout", post(auth::logout))
        .route("/user", get(user::list).post(user::register))
        .route("/user/:user_id", get(user::info))
}
