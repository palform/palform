use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use validator::Validate;

use crate::actix_util::from_org_id::FromOrgId;
use crate::actix_util::validated::Validated;
use crate::audit::id_chains::IDChainEmpty;
use crate::audit::manager::AuditManager;
use crate::auth::rbac::requests::APITokenOrgAdmin;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::orgs::OrganisationManager;

#[derive(Deserialize, JsonSchema, Validate, ApiComponent)]
pub struct PatchOrgRequest {
    #[validate(length(min = 1, max = 40, message = "must be between 1 and 20 characters"))]
    display_name: String,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationsPatchPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Organisations", operation_id = "orgs.rename")]
pub async fn organisations_patch(
    path: Path<OrganisationsPatchPath>,
    data: Validated<Json<PatchOrgRequest>>,
    token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
) -> Result<(), APIError> {
    OrganisationManager::rename(db.as_ref(), path.org_id, data.display_name.clone())
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            db.as_ref(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Organisation,
            Some(path.org_id.into_unknown()),
            Some(format!("Rename to {}", data.display_name.clone())),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
