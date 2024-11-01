//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "t_user")]
pub struct Model {
    #[sea_orm(unique)]
    pub user_code: String,
    #[sea_orm(unique)]
    pub user_no: u64,
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub salt: String,
    pub email: String,
    #[sea_orm(index)]
    pub phone: String,
    pub client_ip: String,
    pub login_at: DateTime,
    pub login_token: String,
    #[sea_orm(primary_key)]
    pub id: u64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: DateTime,
    pub created_by: String,
    pub updated_by: String,
    pub version: u8,
    #[sea_orm(column_name = "is_deleted")]
    pub deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
