use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use rocket::serde::json::Json;
use rocket::{get, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::auth::rbac::requests::APITokenOrgViewer;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::induction::InductionStatusManager;

#[derive(Serialize, JsonSchema)]
pub struct InductionStatus {
    induction_complete: bool,
    key_created: bool,
    can_create_invite: bool,
    invite_created: bool,
    form_created: bool,
}

#[openapi(tag = "Induction", operation_id = "induction.status")]
#[get("/users/me/orgs/<org_id>/induction")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<InductionStatus>, APIErrorWithStatus> {
    let manager = InductionStatusManager::new(token.get_user_id(), org_id, db.inner());

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
