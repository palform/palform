use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::auth::rbac::requests::APITokenOrgViewer;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::induction::InductionStatusManager;

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct InductionStatus {
    induction_complete: bool,
    key_created: bool,
    can_create_invite: bool,
    invite_created: bool,
    form_created: bool,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct InductionStatusPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Induction", operation_id = "induction.status")]
pub async fn induction_status(
    path: Path<InductionStatusPath>,
    token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<InductionStatus>, APIError> {
    let manager = InductionStatusManager::new(token.get_user_id(), path.org_id, db.as_ref());

    let induction_expired = manager
        .induction_period_expired()
        .await
        .map_internal_error()?;
    if induction_expired {
        return Ok(Json(InductionStatus {
            induction_complete: true,
            key_created: false,
            can_create_invite: false,
            invite_created: false,
            form_created: false,
        }));
    }

    let key_created = manager.has_created_key().await.map_internal_error()?;
    let can_create_invite = manager.can_create_invite().await.map_internal_error()?;
    let invite_created = if !can_create_invite {
        false
    } else {
        manager.has_created_invite().await.map_internal_error()?
    };
    let form_created = manager.has_created_form().await.map_internal_error()?;

    Ok(Json(InductionStatus {
        induction_complete: key_created && (invite_created || !can_create_invite) && form_created,
        key_created,
        can_create_invite,
        invite_created,
        form_created,
    }))
}
