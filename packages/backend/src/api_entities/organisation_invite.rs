use chrono::NaiveDateTime;
use palform_entities::organisation_invite;
use palform_tsid::resources::{IDOrganisation, IDOrganisationInvite};
use palform_tsid::tsid::PalformDatabaseID;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, JsonSchema, FromQueryResult)]
pub struct APIOrganisationInvite {
    pub id: PalformDatabaseID<IDOrganisationInvite>,
    pub single_use: bool,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

impl From<organisation_invite::Model> for APIOrganisationInvite {
    fn from(value: organisation_invite::Model) -> Self {
        Self {
            id: value.id,
            single_use: value.single_use,
            created_at: value.created_at,
            expires_at: value.expires_at,
        }
    }
}

#[derive(Serialize, JsonSchema, FromQueryResult)]
pub struct APIOrganisationInvitePreview {
    pub org_id: PalformDatabaseID<IDOrganisation>,
    pub org_display_name: String,
    pub expires_at: NaiveDateTime,
}
