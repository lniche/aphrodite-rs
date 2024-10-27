use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use pkg::identity::Identity;
use pkg::result::{
    rejection::IRejection,
    response::{ApiErr, ApiOK, Result},
};

use crate::api::service;
#[derive(Debug, Validate, Deserialize, Serialize, ToSchema)]
pub struct ReqLogin {
    /// 登录的用户名
    #[validate(length(min = 1, message = "用户名必填"))]
    #[schema(example = "user123")]
    pub username: String,

    /// 用户的密码
    #[validate(length(min = 1, message = "密码必填"))]
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RespLogin {
    /// 用于验证身份的 JWT token
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub auth_token: String,
}

/// 用户登录接口
#[utoipa::path(
    post,
    path = "/login",
    tag = "认证模块",
    request_body = ReqLogin,
    responses(
        (status = 200, description = "获取用户信息成功", body = RespLogin)
    ),
    description = "用户登录接口，用于用户通过用户名和密码进行身份验证，成功后返回认证 token。"
)]
pub async fn login(
    WithRejection(Json(req), _): IRejection<Json<ReqLogin>>,
) -> Result<ApiOK<RespLogin>> {
    if let Err(e) = req.validate() {
        return Err(ApiErr::ErrParams(Some(e.to_string())));
    }
    service::auth::login(req).await
}

/// 用户退出登录接口
#[utoipa::path(
    post,
    path = "/logout",
    tag = "认证模块",
    security(
        ("bearer_auth" = []) 
    ),
    description = "用户退出登录接口，用于清除用户的认证 token，确保后续操作需要重新认证。"
)]
pub async fn logout(Extension(identity): Extension<Identity>) -> Result<ApiOK<()>> {
    if identity.id() == 0 {
        return Ok(ApiOK(None));
    }
    service::auth::logout(identity).await
}
