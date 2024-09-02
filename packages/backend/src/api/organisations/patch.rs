use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use validator::Validate;

use crate::audit::AuditManager;
use crate::auth::rbac::requests::APITokenOrgAdmin;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::orgs::OrganisationManager;
use crate::rocket_util::from_org_id::FromOrgId;
use crate::rocket_util::validated::Validated;

#[derive(Deserialize, JsonSchema, Validate)]
pub struct PatchOrgRequest {
    #[validate(length(min = 1, max = 40, message = "must be between 1 and 20 characters"))]
    display_name: String,
}

#[openapi(tag = "Organisations", operation_id = "orgs.rename")]
#[post("/users/me/orgs/<org_id>", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    data: Validated<Json<PatchOrgRequest>>,
    token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
) -> Result<(), APIErrorWithStatus> {
    OrganisationManager::rename(db.inner(), org_id, data.display_name.clone())
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            db.inner(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Organisation,
            Some(org_id.into_unknown()),
            Some(format!("Rename to {}", data.display_name.clone())),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
