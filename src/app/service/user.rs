use std::collections::HashMap;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::Serialize;

use crate::app::model::{prelude::User, user};
use crate::{
    app::api::user::GetUserResp,
    pkg::{
        core::db,
        result::response::{ApiErr, ApiOK, Result},
        util::helper,
    },
};
use chrono::{NaiveDateTime, Utc};
use sea_orm::sea_query::Expr;

pub async fn info(user_code: String) -> Result<ApiOK<GetUserResp>> {
    let model = User::find()
        .filter(user::Column::UserCode.eq(user_code))
        .one(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find user");
            ApiErr::ErrSystem(None)
        })?
        .ok_or(ApiErr::ErrNotFound(Some("账号不存在".to_string())))?;

    let resp = GetUserResp {
        user_code: model.user_code,
        user_no: model.user_no,
        nickname: model.nickname,
        email: model.email,
        phone: model.phone,
    };

    Ok(ApiOK(Some(resp)))
}

#[derive(Debug, Serialize)]
pub struct RespInfo {
    pub id: u64,
    pub username: String,
    login_at: NaiveDateTime,
    created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct RespList {
    pub total: i64,
    pub list: Vec<RespInfo>,
}

pub async fn list(query: HashMap<String, String>) -> Result<ApiOK<RespList>> {
    let mut builder = User::find();
    if let Some(username) = query.get("username") {
        if !username.is_empty() {
            builder = builder.filter(user::Column::Username.eq(username.to_owned()));
        }
    }

    let mut total: i64 = 0;
    let (offset, limit) = helper::query_page(&query);
    // 仅在第一页计算数量
    if offset == 0 {
        total = builder
            .clone()
            .select_only()
            .column_as(user::Column::Id.count(), "count")
            .into_tuple::<i64>()
            .one(db::conn())
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error count user");
                ApiErr::ErrSystem(None)
            })?
            .unwrap_or_default();
    }

    let models = builder
        .order_by(user::Column::Id, Order::Desc)
        .offset(offset)
        .limit(limit)
        .all(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find user");
            ApiErr::ErrSystem(None)
        })?;
    let mut resp = RespList {
        total,
        list: (Vec::with_capacity(models.len())),
    };
    for model in models {
        let info = RespInfo {
            id: model.id,
            username: model.username,
            login_at: model.login_at,
            created_at: model.created_at,
        };
        resp.list.push(info);
    }

    Ok(ApiOK(Some(resp)))
}

pub async fn update(user_code: String) -> Result<ApiOK<()>> {
    let _model = User::find()
        .filter(user::Column::UserCode.eq(user_code.clone()))
        .one(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find user");
            ApiErr::ErrSystem(None)
        })?
        .ok_or(ApiErr::ErrNotFound(Some("账号不存在".to_string())))?;
    let ret = User::update_many()
        .filter(user::Column::UserCode.eq(user_code))
        .col_expr(user::Column::Deleted, Expr::value(true))
        .col_expr(user::Column::DeletedAt, Expr::value(Utc::now().naive_utc()))
        .exec(db::conn())
        .await;

    if let Err(e) = ret {
        tracing::error!(error = ?e, "error update user");
        return Err(ApiErr::ErrSystem(None));
    }
    Ok(ApiOK(None))
}

pub async fn delete(user_code: String) -> Result<ApiOK<()>> {
    let model = User::find()
        .filter(user::Column::UserCode.eq(user_code))
        .one(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find user");
            ApiErr::ErrSystem(None)
        })?
        .ok_or(ApiErr::ErrNotFound(Some("账号不存在".to_string())))?;

    let mut active_model: user::ActiveModel = model.into();
    active_model.deleted = Set(true);
    active_model.deleted_at = Set(Utc::now().naive_utc());

    if let Err(e) = active_model.update(db::conn()).await {
        tracing::error!(error = ?e, "error update user");
        return Err(ApiErr::ErrSystem(None));
    }
    Ok(ApiOK(None))
}
