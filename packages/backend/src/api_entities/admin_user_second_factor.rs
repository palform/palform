use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use palform_tsid::resources::IDAdminUserSecondAuthenticationFactor;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub enum APIAdminUserSecondAuthenticationFactorMethod {
    TOTP,
    Webauthn,
}

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct APIAdminUserSecondAuthenticationFactor {
    pub id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>,
    pub nickname: String,
    pub created_at: DateTime<Utc>,
    pub method: APIAdminUserSecondAuthenticationFactorMethod,
}
