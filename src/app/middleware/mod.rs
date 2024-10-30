use crate::app::model::prelude::User;
use crate::pkg::core::db;
use crate::pkg::util::identity::Identity;
use anyhow::anyhow;
use anyhow::Result;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use super::model::user;
pub mod auth;

pub async fn auth_check(identity: &Identity) -> Result<()> {
    if identity.code().is_empty() {
        return Err(anyhow!("未授权，请先登录"));
    }
    let ret = User::find()
        .filter(user::Column::UserCode.eq(identity.code()))
        .one(db::conn())
        .await?;
    match ret {
        None => return Err(anyhow!("授权账号不存在")),
        Some(v) => {
            if v.login_token.is_empty() || !identity.match_token(v.login_token) {
                return Err(anyhow!("授权已失效"));
            }
        }
    }
    Ok(())
}
