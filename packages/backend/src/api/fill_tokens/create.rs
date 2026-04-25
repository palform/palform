use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use chrono::{Duration, Utc};
use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::resources::{IDForm, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::actix_util::from_org_id::FromOrgId;
use crate::api::error::{APIError, APIInternalError};
use crate::api_entities::billing::entitlement::APIEntitlementRequest;
use crate::api_entities::fill_token::APIFillToken;
use crate::audit::id_chains::IDChainEmpty;
use crate::audit::manager::AuditManager;
use crate::auth::fill_access::FillAccessTokenManager;
use crate::auth::rbac::requests::APITokenTeamEditorFromForm;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::billing_entitlement_proxy::BillingEntitlementManager;
use crate::entity_managers::forms::FormManager;
use crate::entity_managers::orgs::OrganisationManager;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct NewTokenRequest {
    nickname: String,
    expires_in_seconds: Option<u32>,
    short_link: Option<String>,
}

#[derive(Deserialize, ApiComponent, JsonSchema)]
pub struct FillTokensCreatePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Fill Access Tokens", operation_id = "fill_access_tokens.create")]
pub async fn fill_tokens_create(
    path: Path<FillTokensCreatePath>,
    data: Json<NewTokenRequest>,
    token: APITokenTeamEditorFromForm,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    billing: FromOrgId<BillingEntitlementManager>,
) -> Result<Json<APIFillToken>, APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    if !FormManager::verify_form_org(&txn, path.form_id, path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if let Some(short_link) = data.short_link.clone() {
        billing
            .check_entitlement(&txn, APIEntitlementRequest::Subdomain)
            .await?;

        OrganisationManager::get_org_subdomain(&txn, path.org_id)
            .await
            .map_internal_error()?
            .ok_or(APIError::BadRequest(
                "No subdomain set up for organisation; cannot use short links".to_string(),
            ))?;

        if !FillAccessTokenManager::short_link_is_unique(&txn, path.org_id, short_link)
            .await
            .map_internal_error()?
        {
            return Err(APIError::BadRequest("Short link already in use".to_string()).into());
        }
    }

    let expires_at = data
        .expires_in_seconds
        .map(|sec| Utc::now() + Duration::seconds(i64::from(sec)));

    let new_token = FillAccessTokenManager::create(
        &txn,
        path.form_id,
        data.nickname.clone(),
        expires_at,
        data.short_link.clone(),
    )
    .await
    .map_err(|e| e.to_internal_error())?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Form,
            Some(path.form_id.into_unknown()),
            Some(format!("Created Fill Access Token {}", new_token.id)),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(Json(new_token))
}
