use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use validator::Validate;

use crate::api::service::{
    self,
    user::{ReqCreate, RespInfo, RespList},
};
use crate::pkg::identity::Identity;
use crate::pkg::result::{
    rejection::IRejection,
    response::{ApiErr, ApiOK, Result},
};

pub async fn register(
    Extension(_identity): Extension<Identity>,
    WithRejection(Json(req), _): IRejection<Json<ReqCreate>>,
) -> Result<ApiOK<()>> {
    if let Err(e) = req.validate() {
        return Err(ApiErr::ErrParams(Some(e.to_string())));
    }
    service::user::create(req).await
}

pub async fn info(
    Extension(_identity): Extension<Identity>,
    Path(user_id): Path<u64>,
) -> Result<ApiOK<RespInfo>> {
    service::user::info(user_id).await
}

pub async fn list(
    Extension(_identity): Extension<Identity>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<ApiOK<RespList>> {
    service::user::list(query).await
}
