use chrono::NaiveDateTime;
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, JsonSchema, FromQueryResult)]
pub struct APIOrganisation {
    pub id: PalformDatabaseID<IDOrganisation>,
    pub display_name: String,
    pub created_at: NaiveDateTime,
    pub subdomain: Option<String>,
    pub uses_oidc: bool,
    pub billing_allow_overage: bool,
}
