use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, DbErr, IsolationLevel, TransactionTrait};
use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::{
    actix_util::{from_org_id::FromOrgId, validated::Validated},
    api_entities::billing::entitlement::APIEntitlementRequest,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{rbac::requests::APITokenOrgAdmin, tokens::APIAuthTokenSource},
    entity_managers::{
        billing_entitlement_proxy::BillingEntitlementManager, orgs::OrganisationManager,
    },
};

fn validate_subdomain(value: &str) -> Result<(), ValidationError> {
    for char in value.chars() {
        if !char.is_ascii_lowercase() && !char.is_ascii_digit() && char != '-' {
            return Err(ValidationError::new("invalid subdomain"));
        }
    }

    let first_char = value
        .chars()
        .next()
        .ok_or(ValidationError::new("not enough characters"))?;

    if first_char == '-' || first_char.is_ascii_digit() {
        return Err(ValidationError::new("invalid first character"));
    }

    Ok(())
}

#[derive(Deserialize, JsonSchema, Validate, ApiComponent)]
pub struct CreateSubdomainRequest {
    #[validate(
        custom(
            function = "validate_subdomain",
            message = "Only lowercase letters, numbers, and hyphens (except as the first character) are allowed"
        ),
        length(min = 3, max = 20, message = "Must be between 3 and 20 characters")
    )]
    subdomain: String,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationsCreateSubdomainPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Organisations", operation_id = "orgs.create_subdomain")]
pub async fn organisations_create_subdomain(
    path: Path<OrganisationsCreateSubdomainPath>,
    token: APITokenOrgAdmin,
    request: Validated<Json<CreateSubdomainRequest>>,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    m: FromOrgId<BillingEntitlementManager>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    m.check_entitlement(&txn, APIEntitlementRequest::Subdomain)
        .await?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Create,
            AuditLogTargetResourceEnum::OrganisationSubdomain,
            Some(path.org_id.into_unknown()),
            Some(format!(
                "Registered subdomain {}.palform.app",
                request.subdomain.clone()
            )),
        )
        .await
        .map_internal_error()?;

    if OrganisationManager::get_org_for_subdomain(&txn, request.subdomain.clone())
        .await
        .map_internal_error()?
        .is_some()
    {
        return Err(APIError::BadRequest("That subdomain is already taken".to_string()).into());
    }

    OrganisationManager::set_org_subdomain(&txn, path.org_id, request.subdomain.clone())
        .await
        .map_err(|e| match e {
            DbErr::RecordNotUpdated => APIError::BadRequest("Your organisation already has a subdomain. Currently, it can't be changed or removed".to_string()).into(),
            e => e.to_internal_error(),
        })?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
