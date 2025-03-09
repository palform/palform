use palform_tsid::{resources::IDFeedbackItem, tsid::PalformDatabaseID};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "feedback_item")]
pub struct Model {
    pub created_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: PalformDatabaseID<IDFeedbackItem>,
    pub score: i32,
    pub comment: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
