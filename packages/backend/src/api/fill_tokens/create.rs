use chrono::{Duration, Utc};
use palform_client_common::errors::error::APIInternalErrorResult;
use palform_tsid::resources::{IDForm, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::api::error::{APIError, APIInternalError};
use crate::api_entities::billing::entitlement::APIEntitlementRequest;
use crate::api_entities::fill_token::APIFillToken;
use crate::auth::fill_access::FillAccessTokenManager;
use crate::auth::rbac::requests::APITokenTeamEditorFromForm;
use crate::entity_managers::billing_entitlement_proxy::BillingEntitlementManager;
use crate::entity_managers::forms::FormManager;
use crate::entity_managers::orgs::OrganisationManager;
use crate::rocket_util::from_org_id::FromOrgId;

#[derive(Deserialize, JsonSchema)]
pub struct NewTokenRequest {
    nickname: String,
    expires_in_seconds: Option<u32>,
    short_link: Option<String>,
}

#[openapi(tag = "Fill Access Tokens", operation_id = "fill_access_tokens.create")]
#[post(
    "/users/me/orgs/<org_id>/forms/<form_id>/fill_access_tokens",
    data = "<data>"
)]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    data: Json<NewTokenRequest>,
    _token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
    billing: FromOrgId<BillingEntitlementManager>,
) -> Result<Json<APIFillToken>, (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    if !FormManager::verify_form_org(&txn, form_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if let Some(short_link) = data.short_link.clone() {
        billing
            .check_entitlement(&txn, APIEntitlementRequest::Subdomain)
            .await?;

        OrganisationManager::get_org_subdomain(&txn, org_id)
            .await
            .map_internal_error()?
            .ok_or(APIError::BadRequest(
                "No subdomain set up for organisation; cannot use short links".to_string(),
            ))?;

        if !FillAccessTokenManager::short_link_is_unique(&txn, org_id, short_link)
            .await
            .map_internal_error()?
        {
            return Err(APIError::BadRequest("Short link already in use".to_string()).into());
        }
    }

    let expires_at = data
        .expires_in_seconds
        .map(|sec| (Utc::now() + Duration::seconds(i64::from(sec))).naive_utc());

    let new_token = FillAccessTokenManager::create(
        &txn,
        form_id,
        data.nickname.clone(),
        expires_at,
        data.short_link.clone(),
    )
    .await
    .map_err(|e| e.to_internal_error())?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(Json(new_token))
}
