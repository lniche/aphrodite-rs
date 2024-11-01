use axum::{
    routing::{get, post, put},
    Router,
};

use crate::app::api::{auth, user};

pub fn open() -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .route("/send-code", post(auth::send_verify_code))
}

pub fn auth() -> Router {
    Router::new()
        .route("/logout", post(auth::logout))
        .route("/user", put(user::update).delete(user::delete))
        .route("/user/:user_id", get(user::info))
}
