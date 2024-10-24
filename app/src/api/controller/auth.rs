use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use validator::Validate;

use pkg::identity::Identity;
use pkg::result::{
    rejection::IRejection,
    response::{ApiErr, ApiOK, Result},
};

use crate::api::service::{
    self,
    auth::{ReqLogin, RespLogin},
};

pub async fn login(
    WithRejection(Json(req), _): IRejection<Json<ReqLogin>>,
) -> Result<ApiOK<RespLogin>> {
    if let Err(e) = req.validate() {
        return Err(ApiErr::ErrParams(Some(e.to_string())));
    }
    service::auth::login(req).await
}

#[utoipa::path(
    post,
    path = "/logout",
    responses(
        (status = 200, description = "Login successful"),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "auth",
)]
pub async fn logout(Extension(identity): Extension<Identity>) -> Result<ApiOK<()>> {
    if identity.id() == 0 {
        return Ok(ApiOK(None));
    }
    service::auth::logout(identity).await
}
