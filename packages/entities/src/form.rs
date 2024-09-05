//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use palform_tsid::{
    resources::{IDForm, IDFormBranding, IDTeam},
    tsid::PalformDatabaseID,
};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "form")]
pub struct Model {
    pub editor_name: String,
    pub title: Option<String>,
    pub created_at: DateTime,
    pub notification_email: bool,
    pub notification_webhook_url: Option<String>,
    pub auto_delete_submission_after_days: Option<i32>,
    #[sea_orm(column_type = "JsonBinary")]
    pub end_configuration: Json,
    pub enable_captcha: bool,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: PalformDatabaseID<IDForm>,
    pub branding_id: Option<PalformDatabaseID<IDFormBranding>>,
    pub team_id: PalformDatabaseID<IDTeam>,
    pub one_question_per_page: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::deleted_submission::Entity")]
    DeletedSubmission,
    #[sea_orm(has_many = "super::fill_access_token::Entity")]
    FillAccessToken,
    #[sea_orm(
        belongs_to = "super::form_branding::Entity",
        from = "Column::BrandingId",
        to = "super::form_branding::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    FormBranding,
    #[sea_orm(has_many = "super::question_group::Entity")]
    QuestionGroup,
    #[sea_orm(has_many = "super::submission::Entity")]
    Submission,
    #[sea_orm(
        belongs_to = "super::team::Entity",
        from = "Column::TeamId",
        to = "super::team::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Team,
}

impl Related<super::deleted_submission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DeletedSubmission.def()
    }
}

impl Related<super::fill_access_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FillAccessToken.def()
    }
}

impl Related<super::form_branding::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FormBranding.def()
    }
}

impl Related<super::question_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QuestionGroup.def()
    }
}

impl Related<super::submission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submission.def()
    }
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
