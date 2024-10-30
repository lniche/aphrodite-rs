use chrono::Utc;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use time::macros::offset;

use crate::pkg::core::{cache, db};
use crate::pkg::crypto::hash::md5;
use crate::pkg::result::response::{ApiErr, ApiOK, Result};
use crate::pkg::util::{helper, identity::Identity, xtime};

use crate::api::auth::{LoginReq, LoginResp, SendVerifyCodeReq};
use crate::app::model::prelude::User;
use crate::app::model::user;

use rand::Rng;

pub async fn login(req: LoginReq) -> Result<ApiOK<LoginResp>> {
    let redis_key = format!("{}{}", SEND_CODE_KEY, req.code);
    let cache_code: String = match cache::RedisClient::get(&redis_key) {
        Ok(value) => value,
        Err(e) => {
            tracing::error!("Failed to get value from Redis: {:?}", e);
            String::new()
        }
    };
    if cache_code != req.code {
        return Err(ApiErr::ErrSystem(None));
    }

    let count = User::find()
        .filter(user::Column::Phone.eq(req.phone.clone()))
        .count(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find user");
            ApiErr::ErrSystem(None)
        })?;
    if count > 0 {
        
    }else{

    }

    let now = Utc::now().naive_utc();
    let login_token =
        md5(format!("auth.{}.{}.{}", model.user_code, now, helper::nonce(16)).as_bytes());
    let access_token = Identity::new(model.user_code.clone(), login_token.clone())
        .to_auth_token()
        .map_err(|e| {
            tracing::error!(error = ?e, "error identity encrypt");
            ApiErr::ErrSystem(None)
        })?;
    let update_model = user::ActiveModel {
        login_at: Set(now),
        login_token: Set(login_token),
        updated_at: Set(now),
        ..Default::default()
    };
    let ret_update = User::update_many()
        .filter(user::Column::UserCode.eq(model.user_code))
        .set(update_model)
        .exec(db::conn())
        .await;
    if let Err(e) = ret_update {
        tracing::error!(error = ?e, "error update user");
        return Err(ApiErr::ErrSystem(None));
    }

    let resp = LoginResp { access_token };

    Ok(ApiOK(Some(resp)))
}

pub async fn logout(user_code: String) -> Result<ApiOK<()>> {
    let ret = User::update_many()
        .filter(user::Column::Id.eq(user_code))
        .col_expr(user::Column::LoginToken, Expr::value(""))
        .col_expr(
            user::Column::CreatedAt,
            Expr::value(xtime::now(offset!(+8)).unix_timestamp()),
        )
        .exec(db::conn())
        .await;

    if let Err(e) = ret {
        tracing::error!(error = ?e, "error update user");
        return Err(ApiErr::ErrSystem(None));
    }

    Ok(ApiOK(None))
}

pub const SEND_CODE_KEY: &str = "send:code:";

pub async fn send_verify_code(req: SendVerifyCodeReq) -> Result<ApiOK<()>> {
    let code: u32 = rand::thread_rng().gen_range(1000..10000);
    let code_str = code.to_string();
    tracing::debug!("send verify code {} {}", code_str, req.phone);
    let redis_key = format!("{}{}", SEND_CODE_KEY, req.phone);

    match cache::RedisClient::set(&redis_key, &code_str, Some(60)) {
        Ok(_) => Ok(ApiOK(None)),
        Err(e) => {
            tracing::error!("Failed to set value in Redis: {:?}", e);
            Err(ApiErr::ErrSystem(None))
        }
    }
}
