use palform_tsid::{resources::{IDForm, IDOrganisation}, tsid::PalformDatabaseID};
use rocket::{get, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::form::APIForm,
    auth::rbac::requests::APITokenTeamViewerFromForm,
    entity_managers::forms::FormManager,
};

#[openapi(tag = "Forms", operation_id = "forms.get")]
#[get("/users/me/orgs/<_org_id>/forms/<form_id>")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    _token: APITokenTeamViewerFromForm,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIForm>, (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadOnly),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    let form = FormManager::get_by_id(&txn, form_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;

    Ok(Json(form))
}
