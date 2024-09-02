use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;
use sequoia_openpgp::packet::key::PublicParts;

use crate::api_entities::billing::entitlement::APIEntitlementRequest;
use crate::api_entities::key::APIUserKeyWithIdentity;
use crate::auth::rbac::requests::APITokenOrgAdmin;
use crate::crypto::keys::{CryptoKeyRepr, KeyConversionError};
use crate::entity_managers::billing_entitlement_proxy::BillingEntitlementManager;
use crate::entity_managers::keys::UserKeyManager;
use crate::rocket_util::from_org_id::FromOrgId;

#[openapi(tag = "Organisation Keys", operation_id = "org.keys.list")]
#[get("/users/me/orgs/<org_id>/keys/all", rank = 1)]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    _token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
    billing: FromOrgId<BillingEntitlementManager>,
) -> Result<Json<Vec<APIUserKeyWithIdentity>>, APIErrorWithStatus> {
    billing
        .check_entitlement(db.inner(), APIEntitlementRequest::CryptoDetails)
        .await?;

    let all_keys = UserKeyManager::list_all_org_keys_with_identities(db.inner(), org_id)
        .await
        .map_internal_error()?;

    let all_keys: Result<Vec<APIUserKeyWithIdentity>, KeyConversionError> = all_keys
        .iter()
        .map(|key| {
            let key_data = CryptoKeyRepr::<PublicParts>::from_database_bytes(&key.public_key)?;
            Ok(APIUserKeyWithIdentity {
                id: key.id,
                key_fingerprint: key_data.fingerprint().to_hex(),
                user_id: key.user_id,
                user_display_name: key.user_display_name.clone(),
                created_at: key.created_at,
                expires_at: key.expires_at,
            })
        })
        .collect();

    let all_keys =
        all_keys.map_err(|e| APIError::report_internal_error("get key fingerprints in org", e))?;

    Ok(Json(all_keys))
}
