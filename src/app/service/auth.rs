use chrono::Utc;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use time::macros::offset;

use crate::pkg::crypto::hash::md5;
use crate::pkg::util::{identity::Identity,util,xtime};
use crate::pkg::result::response::{ApiErr, ApiOK, Result};
use crate::pkg::core::{cache,db };

use crate::api::auth::{LoginReq, LoginResp, SendVerifyCodeReq};
use crate::app::model::prelude::User;
use crate::app::model::user;

use rand::Rng;

pub async fn login(req: LoginReq) -> Result<ApiOK<LoginResp>> {
    let model = User::find()
        .filter(user::Column::Username.eq(req.username))
        .one(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error find user");
            ApiErr::ErrSystem(None)
        })?
        .ok_or(ApiErr::ErrAuth(Some("账号不存在".to_string())))?;

    let pass = format!("{}{}", req.password, model.salt);
    if md5(pass.as_bytes()) != model.password {
        return Err(ApiErr::ErrAuth(Some("密码错误".to_string())));
    }

    let now = Utc::now().naive_utc();
    let login_token = md5(format!("auth.{}.{}.{}", model.id, now, util::nonce(16)).as_bytes());
    let auth_token = Identity::new(model.id, login_token.clone())
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
        .filter(user::Column::Id.eq(model.id))
        .set(update_model)
        .exec(db::conn())
        .await;
    if let Err(e) = ret_update {
        tracing::error!(error = ?e, "error update user");
        return Err(ApiErr::ErrSystem(None));
    }

    let resp = LoginResp { auth_token };

    Ok(ApiOK(Some(resp)))
}

pub async fn logout(identity: Identity) -> Result<ApiOK<()>> {
    let ret = User::update_many()
        .filter(user::Column::Id.eq(identity.id()))
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
    tracing::debug!( "send verify code {} {}", code_str, req.phone);
    let redis_key = format!("{}{}", SEND_CODE_KEY, req.phone);
    let _ = cache::RedisClient::set(&redis_key,&code_str,Some(60));

    match cache::RedisClient::set(&redis_key, &code_str, Some(60)) {
        Ok(_) => Ok(ApiOK(None)),
        Err(e) => {
            tracing::error!("Failed to set value in Redis: {:?}", e);
            Err(ApiErr::ErrSystem(None)) // 自定义错误处理
        }
    }
}