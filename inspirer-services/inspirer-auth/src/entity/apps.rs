//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use tabled::Tabled;

use crate::auth::application::AppSetting;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Tabled)]
#[sea_orm(table_name = "apps")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    pub domain_uuid: Uuid,
    #[sea_orm(unique)]
    pub name: String,
    pub display_name: String,
    #[tabled(display_with = "crate::helper::base64_encode")]
    pub secret: Vec<u8>,
    pub profile: Json,
    #[tabled(inline)]
    pub setting: AppSetting,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
