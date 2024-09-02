//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use palform_tsid::{
    resources::{IDAdminUser, IDAdminUserSecondAuthenticationFactorSession},
    tsid::PalformDatabaseID,
};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "admin_user_second_authentication_factor_session")]
pub struct Model {
    pub created_at: DateTime,
    pub expires_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
    pub user_id: PalformDatabaseID<IDAdminUser>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::admin_user::Entity",
        from = "Column::UserId",
        to = "super::admin_user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    AdminUser,
}

impl Related<super::admin_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AdminUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
