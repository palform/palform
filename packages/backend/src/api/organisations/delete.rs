use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{delete, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    audit::AuditManager, auth::rbac::requests::APITokenOrgAdmin, auth::tokens::APIAuthTokenSource,
    config::Config, entity_managers::orgs::OrganisationManager, mail::client::PalformMailClient,
    rocket_util::from_org_id::FromOrgId,
};

#[openapi(tag = "Organisations", operation_id = "orgs.delete")]
#[delete("/users/me/orgs/<org_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    token: APITokenOrgAdmin,
    audit: FromOrgId<AuditManager>,
    db: &State<DatabaseConnection>,
    config: &State<Config>,
    mail: &State<PalformMailClient>,
) -> Result<(), APIErrorWithStatus> {
    OrganisationManager::send_staff_deletion_request(org_id, config, mail)
        .await
        .map_err(|e| APIError::report_internal_error("send org deletion request", e))?;

    audit
        .log_event_with_note(
            db.inner(),
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::Organisation,
            Some(org_id.into_unknown()),
            Some("Requested deletion of organisation".to_string()),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
