//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use palform_tsid::{
    resources::{IDForm, IDQuestionGroup},
    tsid::PalformDatabaseID,
};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "question_group")]
pub struct Model {
    pub title: Option<String>,
    pub description: Option<String>,
    pub position: i32,
    #[sea_orm(column_type = "JsonBinary")]
    pub step_strategy: Json,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: PalformDatabaseID<IDQuestionGroup>,
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
    #[sea_orm(has_many = "super::question::Entity")]
    Question,
}

impl Related<super::form::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Form.def()
    }
}

impl Related<super::question::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Question.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
