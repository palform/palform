//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use super::sea_orm_active_enums::OrganisationMemberRoleEnum;
use palform_tsid::{
    resources::{IDAdminUser, IDTeam},
    tsid::PalformDatabaseID,
};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "team_membership")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub team_id: PalformDatabaseID<IDTeam>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: PalformDatabaseID<IDAdminUser>,
    pub role: OrganisationMemberRoleEnum,
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
        belongs_to = "super::team::Entity",
        from = "Column::TeamId",
        to = "super::team::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Team,
}

impl Related<super::admin_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AdminUser.def()
    }
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
