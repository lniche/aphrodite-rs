use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::pkg::result::{
    rejection::IRejection,
    response::{Errors, Result, Results},
    status::Resp,
};
use crate::pkg::util::identity::Identity;

use crate::app::service;

#[derive(Debug, Validate, Deserialize, Serialize, ToSchema)]
pub struct SendVerifyCodeReq {
    // + country code phone number
    #[validate(length(min = 1, message = "Phone number is required"))]
    #[schema(example = "13800138000")]
    pub phone: String,
}

// Send Verification Code
#[utoipa::path(
    post,
    path = "/v1/send-code",
    tag = "Auth Module",
    request_body = SendVerifyCodeReq,
    summary = "Send Verification Code",
    responses(
        (status = 200, body = Resp, description="Successful Response")
    )
)]
pub async fn send_verify_code(
    WithRejection(Json(req), _): IRejection<Json<SendVerifyCodeReq>>,
) -> Result<Results<()>> {
    if let Err(e) = req.validate() {
        return Err(Errors::ErrBadRequest(Some(e.to_string())));
    }
    service::auth::send_verify_code(req).await
}

#[derive(Debug, Validate, Deserialize, Serialize, ToSchema)]
pub struct LoginReq {
    // Username for login
    #[validate(length(min = 1, message = "Username is required"))]
    #[schema(example = "13800138000")]
    pub phone: String,

    // User's password
    #[validate(length(min = 1, message = "Password is required"))]
    #[schema(example = "1234")]
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginResp {
    // Access token
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
}

// User Registration/Login
#[utoipa::path(
    post,
    path = "/v1/login",
    tag = "Auth Module",
    request_body = LoginReq,
    responses(
        (status = 200, body = LoginResp, description="Successful Response")
    ),
    summary = "User Registration/Login"
)]
pub async fn login(
    Extension(ip): Extension<String>,
    WithRejection(Json(req), _): IRejection<Json<LoginReq>>,
) -> Result<Results<LoginResp>> {
    if let Err(e) = req.validate() {
        return Err(Errors::ErrBadRequest(Some(e.to_string())));
    }
    service::auth::login(req, ip).await
}

// User Logout
#[utoipa::path(
    post,
    path = "/v1/logout",
    tag = "Auth Module",
    security(
        ("bearer_auth" = []) 
    ),
    summary = "User Logout",
    responses(
        (status = 200, body = Resp, description="Successful Response")
    )
)]
pub async fn logout(Extension(identity): Extension<Identity>) -> Result<Results<()>> {
    service::auth::logout(identity.code()).await
}
