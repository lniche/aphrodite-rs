use std::net::SocketAddr;

use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use http::Request;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::pkg::result::{
    rejection::IRejection,
    response::{ApiErr, ApiOK, Result},
};
use crate::pkg::util::identity::Identity;

use crate::app::service;

#[derive(Debug, Validate, Deserialize, Serialize, ToSchema)]
pub struct SendVerifyCodeReq {
    // +区号手机号
    #[validate(length(min = 1, message = "手机号必填"))]
    #[schema(example = "13800138000")]
    pub phone: String,
}

// 发送验证码
#[utoipa::path(
    post,
    path = "/v1/send-code",
    tag = "认证模块",
    request_body = SendVerifyCodeReq,
    description = "发送校验短信"
)]
pub async fn send_verify_code(
    WithRejection(Json(req), _): IRejection<Json<SendVerifyCodeReq>>,
) -> Result<ApiOK<()>> {
    if let Err(e) = req.validate() {
        return Err(ApiErr::ErrParams(Some(e.to_string())));
    }
    service::auth::send_verify_code(req).await
}

#[derive(Debug, Validate, Deserialize, Serialize, ToSchema)]
pub struct LoginReq {
    // 登录的用户名
    #[validate(length(min = 1, message = "用户名必填"))]
    #[schema(example = "13800138000")]
    pub phone: String,

    // 用户的密码
    #[validate(length(min = 1, message = "密码必填"))]
    #[schema(example = "1234")]
    pub code: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginResp {
    // 访问令牌
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
}

// 用户登录接口
#[utoipa::path(
    post,
    path = "/v1/login",
    tag = "认证模块",
    request_body = LoginReq,
    responses(
        (status = 200, description = "获取用户信息成功", body = LoginResp)
    ),
    description = "用户登录接口"
)]
pub async fn login(
    WithRejection(Json(req), _): IRejection<Json<LoginReq>>,
) -> Result<ApiOK<LoginResp>> {
    if let Err(e) = req.validate() {
        return Err(ApiErr::ErrParams(Some(e.to_string())));
    }
    service::auth::login(req, "".to_string()).await
}

// 用户退出登录接口
#[utoipa::path(
    post,
    path = "/v1/logout",
    tag = "认证模块",
    security(
        ("bearer_auth" = []) 
    ),
    description = "用户退出登录接口"
)]
pub async fn logout(Extension(identity): Extension<Identity>) -> Result<ApiOK<()>> {
    service::auth::logout(identity.code()).await
}
