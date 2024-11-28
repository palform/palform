use chrono::NaiveDateTime;
use palform_tsid::resources::IDAdminUserSecondAuthenticationFactor;
use palform_tsid::tsid::PalformDatabaseID;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub enum APIAdminUserSecondAuthenticationFactorMethod {
    TOTP,
    Webauthn,
}

#[derive(Serialize, JsonSchema)]
pub struct APIAdminUserSecondAuthenticationFactor {
    pub id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>,
    pub nickname: String,
    pub created_at: NaiveDateTime,
    pub method: APIAdminUserSecondAuthenticationFactorMethod,
}
