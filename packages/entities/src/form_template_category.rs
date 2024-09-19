//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use palform_tsid::{resources::IDFormTemplateCategory, tsid::PalformDatabaseID};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "form_template_category")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: PalformDatabaseID<IDFormTemplateCategory>,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::form_template_category_assignment::Entity")]
    FormTemplateCategoryAssignment,
}

impl Related<super::form_template_category_assignment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FormTemplateCategoryAssignment.def()
    }
}

impl Related<super::form_template::Entity> for Entity {
    fn to() -> RelationDef {
        super::form_template_category_assignment::Relation::FormTemplate.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::form_template_category_assignment::Relation::FormTemplateCategory
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
