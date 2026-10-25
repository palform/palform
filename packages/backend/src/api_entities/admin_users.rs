use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use palform_tsid::resources::IDAdminUser;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(JsonSchema, Serialize, FromQueryResult)]
pub struct APIAdminUser {
    pub id: PalformDatabaseID<IDAdminUser>,
    pub display_name: Option<String>,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(JsonSchema, Serialize, FromQueryResult, ApiComponent)]
pub struct APIServiceAccountAdminUser {
    pub id: PalformDatabaseID<IDAdminUser>,
    pub email: String,
    /// The service account's token, to be used as a bearer credential when making API requests.
    /// Only returned once when the service account is created.
    pub service_account_auth_token: Option<String>,
    pub created_at: DateTime<Utc>,
}
