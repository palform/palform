use chrono::NaiveDateTime;
use palform_tsid::resources::IDAdminUserSecondAuthenticationFactor;
use palform_tsid::tsid::PalformDatabaseID;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, JsonSchema, FromQueryResult)]
pub struct APIAdminUserSecondAuthenticationFactor {
    id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>,
    nickname: String,
    created_at: NaiveDateTime,
}
