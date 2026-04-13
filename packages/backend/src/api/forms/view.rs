use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api::error::APIError, api_entities::form::APIFormWithQuestions,
    auth::fill_access::APIFillAccessToken, entity_managers::forms::FormManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormsViewPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Forms", operation_id = "forms.view")]
pub async fn forms_view(
    path: Path<FormsViewPath>,
    _fill_access_token: APIFillAccessToken,
    db: Data<DatabaseConnection>,
) -> Result<Json<APIFormWithQuestions>, APIError> {
    let resp = FormManager::get_with_questions(db.as_ref(), path.form_id)
        .await
        .map_err(|e| APIError::report_internal_error("get form", e))?;

    Ok(Json(resp))
}
