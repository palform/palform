use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::form::APIForm,
    auth::rbac::requests::APITokenTeamViewerFromForm,
    entity_managers::forms::FormManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormsGetPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Forms", operation_id = "forms.get")]
pub async fn forms_get(
    path: Path<FormsGetPath>,
    _token: APITokenTeamViewerFromForm,
    db: Data<DatabaseConnection>,
) -> Result<Json<APIForm>, APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadOnly),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    let form = FormManager::get_by_id(&txn, path.form_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;

    Ok(Json(form))
}
