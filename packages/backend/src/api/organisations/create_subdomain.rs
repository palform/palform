use palform_client_common::errors::error::{
    APIError, APIErrorWithStatus, APIInternalError, APIInternalErrorResult,
};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use sea_orm::{DatabaseConnection, DbErr};
use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::{
    api_entities::billing::entitlement::APIEntitlementRequest,
    audit::AuditManager,
    auth::{rbac::requests::APITokenOrgAdmin, tokens::APIAuthTokenSource},
    entity_managers::{
        billing_entitlement_proxy::BillingEntitlementManager, orgs::OrganisationManager,
    },
    rocket_util::{from_org_id::FromOrgId, validated::Validated},
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

#[derive(Deserialize, JsonSchema, Validate)]
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

#[openapi(tag = "Organisations", operation_id = "orgs.create_subdomain")]
#[post("/user/me/orgs/<org_id>/subdomain", data = "<request>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    token: APITokenOrgAdmin,
    request: Validated<Json<CreateSubdomainRequest>>,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
    m: FromOrgId<BillingEntitlementManager>,
) -> Result<(), APIErrorWithStatus> {
    m.check_entitlement(db.inner(), APIEntitlementRequest::Subdomain)
        .await?;

    audit
        .log_event_with_note(
            db.inner(),
            token.get_user_id(),
            AuditLogVerbEnum::Create,
            AuditLogTargetResourceEnum::OrganisationSubdomain,
            Some(org_id.into_unknown()),
            Some(format!(
                "Registered subdomain {}.palform.app",
                request.subdomain.clone()
            )),
        )
        .await
        .map_internal_error()?;

    if OrganisationManager::get_org_for_subdomain(db.inner(), request.subdomain.clone())
        .await
        .map_internal_error()?
        .is_some()
    {
        return Err(APIError::BadRequest("That subdomain is already taken".to_string()).into());
    }

    OrganisationManager::set_org_subdomain(db.inner(), org_id, request.subdomain.clone())
        .await
        .map_err(|e| match e {
            DbErr::RecordNotUpdated => APIError::BadRequest("Your organisation already has a subdomain. Currently, it can't be changed or removed".to_string()).into(),
            e => e.to_internal_error(),
        })?;

    Ok(())
}
