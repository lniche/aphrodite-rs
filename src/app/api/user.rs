use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::pkg::result::response::{ApiOK, Result};
use crate::pkg::util::identity::Identity;
use crate::{
    app::service::{self, user::RespList},
    pkg::result::rejection::IRejection,
};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
// 响应获取用户信息的结构体
pub struct GetUserResp {
    // 用户的唯一标识代码
    #[schema(example = "S8000")]
    pub user_code: String,

    // 用户的登录名
    #[schema(example = "100000")]
    pub user_no: u64,

    // 用户的昵称
    #[schema(example = "john")]
    pub nickname: String,

    // 用户的电子邮件地址
    #[schema(example = "john@example.com")]
    pub email: String,

    // 用户的电话号码
    #[schema(example = "13800138000")]
    pub phone: String,
}

// 用户信息接口
#[utoipa::path(
    get,
    path = "/v1/user",
    tag = "用户模块",
    security(
        ("bearer_auth" = []) 
    ),
    description = "用户信息接口"
)]
pub async fn info(
    Extension(identity): Extension<Identity>,
    Path(user_code): Path<String>,
) -> Result<ApiOK<GetUserResp>> {
    let user_code = if user_code.is_empty() && !identity.code().is_empty() {
        identity.code()
    } else {
        user_code
    };
    service::user::info(user_code).await
}

pub async fn list(
    Extension(_identity): Extension<Identity>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<ApiOK<RespList>> {
    service::user::list(query).await
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
// 请求更新用户信息的结构体
pub struct UpdateUserReq {
    // 用户的新昵称
    #[schema(example = "john")]
    pub nickname: String,

    // 用户的新电子邮件地址
    #[schema(example = "john@example.com")]
    pub email: String,
}

// 用户更新接口
#[utoipa::path(
    put,
    path = "/v1/user",
    tag = "用户模块",
    security(
        ("bearer_auth" = []) 
    ),
    description = "用户更新接口"
)]
pub async fn update(
    Extension(identity): Extension<Identity>,
    WithRejection(Json(req), _): IRejection<Json<UpdateUserReq>>,
) -> Result<ApiOK<()>> {
    service::user::update(req, identity.code()).await
}

// 用户删除接口
#[utoipa::path(
    delete,
    path = "/v1/user",
    tag = "用户模块",
    security(
        ("bearer_auth" = []) 
    ),
    description = "用户删除接口"
)]
pub async fn delete(Extension(identity): Extension<Identity>) -> Result<ApiOK<()>> {
    service::user::delete(identity.code()).await
}
