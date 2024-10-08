//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use palform_tsid::{
    resources::{IDForm, IDFormTemplateCategory},
    tsid::PalformDatabaseID,
};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "form_template_category_assignment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub form_id: PalformDatabaseID<IDForm>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub category_id: PalformDatabaseID<IDFormTemplateCategory>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::form_template::Entity",
        from = "Column::FormId",
        to = "super::form_template::Column::FormId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    FormTemplate,
    #[sea_orm(
        belongs_to = "super::form_template_category::Entity",
        from = "Column::CategoryId",
        to = "super::form_template_category::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    FormTemplateCategory,
}

impl Related<super::form_template::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FormTemplate.def()
    }
}

impl Related<super::form_template_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FormTemplateCategory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
