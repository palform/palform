use apistos::ApiComponent;
use palform_tsid::{
    resources::{IDFillAccessToken, IDForm, IDFormTemplateCategory, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, JsonSchema, FromQueryResult, ApiComponent)]
pub struct APIFormTemplate {
    pub id: PalformDatabaseID<IDForm>,
    pub name: String,
    pub description: String,
    pub views: i64,
    pub clones: i64,
    pub author_name: String,
    pub preview_token: PalformDatabaseID<IDFillAccessToken>,
    pub organisation_id: PalformDatabaseID<IDOrganisation>,
}

#[derive(Serialize, JsonSchema, FromQueryResult, ApiComponent)]
pub struct APIFormTemplateCategory {
    pub id: PalformDatabaseID<IDFormTemplateCategory>,
    pub name: String,
    pub template_count: i64,
}
