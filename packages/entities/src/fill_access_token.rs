//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use palform_tsid::{
    resources::{IDFillAccessToken, IDForm},
    tsid::PalformDatabaseID,
};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "fill_access_token")]
pub struct Model {
    pub created_at: DateTime,
    pub expires_at: Option<DateTime>,
    pub nickname: String,
    pub short_link: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: PalformDatabaseID<IDFillAccessToken>,
    pub form_id: PalformDatabaseID<IDForm>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::form::Entity",
        from = "Column::FormId",
        to = "super::form::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Form,
    #[sea_orm(has_many = "super::submission::Entity")]
    Submission,
}

impl Related<super::form::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Form.def()
    }
}

impl Related<super::submission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
