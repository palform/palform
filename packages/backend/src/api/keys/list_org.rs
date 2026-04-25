use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use sequoia_openpgp::packet::key::PublicParts;
use serde::Deserialize;

use crate::actix_util::from_org_id::FromOrgId;
use crate::api_entities::billing::entitlement::APIEntitlementRequest;
use crate::api_entities::key::APIUserKeyWithIdentity;
use crate::auth::rbac::requests::APITokenOrgAdmin;
use crate::crypto::keys::{CryptoKeyRepr, KeyConversionError};
use crate::entity_managers::billing_entitlement_proxy::BillingEntitlementManager;
use crate::entity_managers::keys::UserKeyManager;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct KeysListOrgPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Organisation Keys", operation_id = "org.keys.list")]
pub async fn keys_list_org(
    path: Path<KeysListOrgPath>,
    _token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    billing: FromOrgId<BillingEntitlementManager>,
) -> Result<Json<Vec<APIUserKeyWithIdentity>>, APIError> {
    billing
        .check_entitlement(db.as_ref(), APIEntitlementRequest::CryptoDetails)
        .await?;

    let all_keys = UserKeyManager::list_all_org_keys_with_identities(db.as_ref(), path.org_id)
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
                user_email: key.user_email.clone(),
                created_at: key.created_at,
                expires_at: key.expires_at,
            })
        })
        .collect();

    let all_keys =
        all_keys.map_err(|e| APIError::report_internal_error("get key fingerprints in org", e))?;

    Ok(Json(all_keys))
}
