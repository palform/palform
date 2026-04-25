use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, JsonSchema, FromQueryResult, ApiComponent)]
pub struct APIOrganisation {
    pub id: PalformDatabaseID<IDOrganisation>,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub subdomain: Option<String>,
    pub uses_oidc: bool,
    pub billing_allow_overage: bool,
}
