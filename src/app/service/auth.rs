use chrono::Utc;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use time::macros::offset;

use crate::pkg::core::{cache, db};
use crate::pkg::crypto::hash::md5;
use crate::pkg::result::response::{Errors, Result, Results};
use crate::pkg::util::snowflake::SnowflakeGen;
use crate::pkg::util::{helper, identity::Identity, xtime};

use crate::api::auth::{LoginReq, LoginResp, SendVerifyCodeReq};
use crate::app::model::prelude::User;
use crate::app::model::user;

use rand::Rng;

pub async fn login(req: LoginReq, ip: String) -> Result<Results<LoginResp>> {
    let redis_key = format!("{}{}", SEND_CODE_KEY, req.phone);
    let cache_code: String = match cache::RedisClient::get(&redis_key) {
        Ok(value) => value,
        Err(e) => {
            tracing::error!("Failed to get value from Redis: {:?}", e);
            String::new()
        }
    };
    if cache_code != req.code {
        tracing::error!(
            "Verification code is incorrect: cache_code = {:?}, req.code = {:?}",
            cache_code,
            req.code
        );
        return Err(Errors::ErrInternalServerError(Some(
            "Verification code is incorrect".to_string(),
        )));
    }
    let user_option = User::find()
        .filter(user::Column::Phone.eq(req.phone.clone()))
        .one(db::conn())
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to retrieve user information");
            Errors::ErrInternalServerError(None)
        })?;

    let now = Utc::now().naive_utc();
    let login_token: String;
    let user_code: String;

    if let Some(user) = user_option {
        user_code = user.user_code.clone();
        login_token = md5(format!("auth.{}.{}.{}", user_code, now, helper::nonce(16)).as_bytes());
        let update_model = user::ActiveModel {
            client_ip: Set(ip),
            login_at: Set(now),
            login_token: Set(login_token.clone()),
            ..Default::default()
        };
        User::update_many()
            .filter(user::Column::UserCode.eq(user.user_code.clone()))
            .set(update_model)
            .exec(db::conn())
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Failed to update user");
                Errors::ErrInternalServerError(None)
            })?;
    } else {
        let mut generator = SnowflakeGen::new(1);
        user_code = generator.next().to_string();
        login_token = md5(format!("auth.{}.{}.{}", user_code, now, helper::nonce(16)).as_bytes());
        let user_no = cache::RedisClient::next_no().map_err(|e| {
            tracing::error!(error = ?e, "Failed to generate user_no");
            Errors::ErrInternalServerError(None)
        })?;
        let phone_suffix = &req.phone[req.phone.len().saturating_sub(4)..];
        let nickname_value = format!("SUGAR_{}", phone_suffix);
        let new_user = user::ActiveModel {
            phone: Set(req.phone.clone()),
            nickname: Set(nickname_value),
            user_code: Set(user_code.clone()),
            user_no: Set(user_no),
            client_ip: Set(ip),
            login_token: Set(login_token.clone()),
            login_at: Set(now),
            created_at: Set(now),
            created_by: Set("777".to_string()),
            ..Default::default()
        };
        User::insert(new_user).exec(db::conn()).await.map_err(|e| {
            tracing::error!(error = ?e, "Failed to create user");
            Errors::ErrInternalServerError(None)
        })?;
    }

    let access_token = Identity::new(req.phone.clone(), login_token.clone())
        .to_auth_token()
        .map_err(|e| {
            tracing::error!(error = ?e, "Failed to encrypt identity");
            Errors::ErrInternalServerError(None)
        })?;

    let resp = LoginResp { access_token };
    Ok(Results(Some(resp)))
}

pub async fn logout(user_code: String) -> Result<Results<()>> {
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
        tracing::error!(error = ?e, "Error updating user");
        return Err(Errors::ErrInternalServerError(None));
    }

    Ok(Results(None))
}

pub const SEND_CODE_KEY: &str = "send:code:";

pub async fn send_verify_code(req: SendVerifyCodeReq) -> Result<Results<()>> {
    let code: u32 = rand::thread_rng().gen_range(1000..10000);
    let code_str = code.to_string();
    tracing::debug!("Send verify code {} {}", code_str, req.phone);
    let redis_key = format!("{}{}", SEND_CODE_KEY, req.phone);

    match cache::RedisClient::set(&redis_key, &code_str, Some(60)) {
        Ok(_) => Ok(Results(None)),
        Err(e) => {
            tracing::error!("Failed to set value in Redis: {:?}", e);
            Err(Errors::ErrInternalServerError(None))
        }
    }
}
