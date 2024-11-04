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
        return Err(anyhow!("Unauthorized, please log in first"));
    }
    let ret = User::find()
        .filter(user::Column::UserCode.eq(identity.code()))
        .one(db::conn())
        .await?;
    match ret {
        None => return Err(anyhow!("Authorized account does not exist")),
        Some(v) => {
            if v.login_token.is_empty() || !identity.match_token(v.login_token) {
                return Err(anyhow!("Authorization has expired"));
            }
        }
    }
    Ok(())
}
