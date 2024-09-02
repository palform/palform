use palform_tsid::resources::{IDAdminUser, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(JsonSchema, Serialize, Clone, FromQueryResult)]
pub struct APIOrgMember {
    user_id: PalformDatabaseID<IDAdminUser>,
    user_display_name: String,
    organisation_id: PalformDatabaseID<IDOrganisation>,
    is_admin: bool,
}
