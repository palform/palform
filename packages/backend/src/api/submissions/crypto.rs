use std::collections::HashSet;

use palform_crypto::policy::recipient_cert_policy;
use palform_tsid::resources::{IDForm, IDOrganisation, IDSubmission};
use palform_tsid::tsid::PalformDatabaseID;
use rocket::{get, http::Status, serde::json::Json, State};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{okapi::schemars, openapi};
use sea_orm::DatabaseConnection;
use sequoia_openpgp::packet::key::PublicParts;
use sequoia_openpgp::{Fingerprint, KeyHandle};
use serde::Serialize;

use crate::api_entities::billing::entitlement::APIEntitlementRequest;
use crate::api_entities::key::{APIUserKeyWithIdentity, UserKeyWithIdentity};
use crate::auth::rbac::requests::APITokenTeamViewerFromForm;
use crate::crypto::keys::{CryptoKeyRepr, KeyConversionError};
use crate::entity_managers::billing_entitlement_proxy::BillingEntitlementManager;
use crate::rocket_util::from_org_id::FromOrgId;
use crate::{
    api::error::{APIError, APIInternalError},
    crypto::submissions::CryptoSubmissionRepr,
    entity_managers::{forms::FormManager, keys::UserKeyManager, submission::SubmissionManager},
};

#[derive(Serialize, JsonSchema)]
pub enum DecrpytingKey {
    Known(APIUserKeyWithIdentity),
    Unknown(String),
}

#[derive(Serialize, JsonSchema)]
pub struct SubmissionCryptoDetailsResponse {
    decrypting_keys: Vec<DecrpytingKey>,
}

fn filter_certs_containing_key_handles(
    required_key_ids: Vec<KeyHandle>,
    keys: Vec<UserKeyWithIdentity>,
) -> Result<Vec<DecrpytingKey>, KeyConversionError> {
    let mut matched_certs = Vec::<APIUserKeyWithIdentity>::new();
    let policy = recipient_cert_policy();
    let mut seen_key_fingerprints = HashSet::<Fingerprint>::new();
    for key in keys {
        let repr = CryptoKeyRepr::<PublicParts>::from_database_bytes(&key.public_key)?;
        let enc_key = repr.enc_key(&policy)?;
        let matches = required_key_ids.contains(&KeyHandle::KeyID(enc_key.key().keyid()));
        if matches {
            // add ALL subkeys of the matched cert to the list of read keys, otherwise it would show
            // seemingly unidentified keys in the list which were really just part of the matched
            // cert
            for subkey_id in repr.all_subkey_fingerprints(&policy)? {
                seen_key_fingerprints.insert(subkey_id);
            }

            matched_certs.push(APIUserKeyWithIdentity {
                id: key.id,
                user_id: key.user_id,
                user_display_name: key.user_display_name,
                user_email: key.user_email,
                created_at: key.created_at,
                expires_at: key.expires_at,
                key_fingerprint: repr.fingerprint().to_hex(),
            })
        }
    }

    let mut decrypting_key_vec: Vec<DecrpytingKey> = matched_certs
        .iter()
        .map(|c| DecrpytingKey::Known(c.clone()))
        .collect();

    for required_key in required_key_ids {
        let mut seen = false;
        for seen_key_fingerprint in &seen_key_fingerprints {
            println!("{}, {}", required_key, seen_key_fingerprint);
            if required_key.aliases(KeyHandle::from(seen_key_fingerprint)) {
                seen = true;
                break;
            }
        }

        if seen {
            continue;
        }

        decrypting_key_vec.push(DecrpytingKey::Unknown(required_key.to_hex()));
    }

    Ok(decrypting_key_vec)
}

#[openapi(tag = "Submissions", operation_id = "submissions.crypto")]
#[get("/users/me/orgs/<org_id>/forms/<form_id>/submissions/<submission_id>/crypto")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    submission_id: PalformDatabaseID<IDSubmission>,
    _token: APITokenTeamViewerFromForm,
    db: &State<DatabaseConnection>,
    billing: FromOrgId<BillingEntitlementManager>,
) -> Result<Json<SubmissionCryptoDetailsResponse>, (Status, Json<APIError>)> {
    billing
        .check_entitlement(db.inner(), APIEntitlementRequest::CryptoDetails)
        .await?;

    if !SubmissionManager::verify_submission_form(db.inner(), submission_id, form_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if !FormManager::verify_form_org(db.inner(), form_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    let submission = SubmissionManager::get_by_id(db.inner(), submission_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;

    let repr = CryptoSubmissionRepr::from_database_bytes(&submission.encrypted_data)
        .map_err(|e| APIError::report_internal_error("decode submission message bytes", e))?;
    let key_handles = repr.get_decrypting_key_handles();

    let keys_with_identities =
        UserKeyManager::list_all_org_keys_with_identities(db.inner(), org_id)
            .await
            .map_err(|e| APIError::report_internal_error("list org keys", e))?;

    let keys_with_identities =
        filter_certs_containing_key_handles(key_handles, keys_with_identities)
            .map_err(|e| APIError::report_internal_error("filtering certs with key ids", e))?;

    Ok(Json(SubmissionCryptoDetailsResponse {
        decrypting_keys: keys_with_identities,
    }))
}
