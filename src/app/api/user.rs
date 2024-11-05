use axum::{extract::Path, Extension, Json};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::pkg::result::response::{Result, Results};
use crate::pkg::result::status::Resp;
use crate::pkg::util::identity::Identity;
use crate::{
    app::service::{self},
    pkg::result::rejection::IRejection,
};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
// Response structure for getting user information
pub struct GetUserResp {
    // User's unique identifier code
    #[schema(example = "A8000")]
    pub user_code: String,

    // User's login name
    #[schema(example = "100000")]
    pub user_no: u64,

    // User's nickname
    #[schema(example = "john")]
    pub nickname: String,

    // User's email address
    #[schema(example = "john@example.com")]
    pub email: String,

    // User's phone number
    #[schema(example = "13800138000")]
    pub phone: String,
}

// User Info
#[utoipa::path(
    get,
    path = "/v1/user",
    tag = "User Module",
    security(
        ("bearer_auth" = []) 
    ),
    responses(
        (status = 200, body = GetUserResp, description="Successful Response")
    ),
    summary = "User Info"
)]
pub async fn info(
    Extension(identity): Extension<Identity>,
    Path(user_code): Path<String>,
) -> Result<Results<GetUserResp>> {
    let user_code = if user_code.is_empty() && !identity.code().is_empty() {
        identity.code()
    } else {
        user_code
    };
    service::user::info(user_code).await
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
// Request structure for updating user information
pub struct UpdateUserReq {
    // User's new nickname
    #[schema(example = "john")]
    pub nickname: String,

    // User's new email address
    #[schema(example = "john@example.com")]
    pub email: String,
}

// User Update
#[utoipa::path(
    put,
    path = "/v1/user",
    tag = "User Module",
    security(
        ("bearer_auth" = []) 
    ),
    request_body = UpdateUserReq,
    summary = "User Update",
    responses(
        (status = 200, body = Resp, description="Successful Response")
    )
)]
pub async fn update(
    Extension(identity): Extension<Identity>,
    WithRejection(Json(req), _): IRejection<Json<UpdateUserReq>>,
) -> Result<Results<()>> {
    service::user::update(req, identity.code()).await
}

// User Delete
#[utoipa::path(
    delete,
    path = "/v1/user",
    tag = "User Module",
    security(
        ("bearer_auth" = []) 
    ),
    summary = "User Delete",
    responses(
        (status = 200, body = Resp, description="Successful Response")
    )
)]
pub async fn delete(Extension(identity): Extension<Identity>) -> Result<Results<()>> {
    service::user::delete(identity.code()).await
}
