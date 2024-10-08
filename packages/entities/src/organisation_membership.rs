//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use palform_tsid::{
    resources::{IDAdminUser, IDOrganisation},
    tsid::PalformDatabaseID,
};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "organisation_membership")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub organisation_id: PalformDatabaseID<IDOrganisation>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: PalformDatabaseID<IDAdminUser>,
    pub is_admin: bool,
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
    #[sea_orm(
        belongs_to = "super::organisation::Entity",
        from = "Column::OrganisationId",
        to = "super::organisation::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Organisation,
}

impl Related<super::admin_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AdminUser.def()
    }
}

impl Related<super::organisation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organisation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
