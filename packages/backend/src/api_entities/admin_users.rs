use chrono::NaiveDateTime;
use palform_tsid::resources::IDAdminUser;
use palform_tsid::tsid::PalformDatabaseID;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(JsonSchema, Serialize, FromQueryResult)]
pub struct APIAdminUser {
    pub id: PalformDatabaseID<IDAdminUser>,
    pub display_name: Option<String>,
    pub email: String,
    pub created_at: NaiveDateTime,
}
