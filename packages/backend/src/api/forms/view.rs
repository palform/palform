use palform_tsid::{resources::{IDForm, IDOrganisation}, tsid::PalformDatabaseID};
use rocket::{get, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api::error::APIError, api_entities::form::APIFormWithQuestions,
    auth::fill_access::APIFillAccessToken, entity_managers::forms::FormManager,
};

#[openapi(tag = "Forms", operation_id = "forms.view")]
#[get("/fill/orgs/<_org_id>/forms/<form_id>")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    _fill_access_token: APIFillAccessToken,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIFormWithQuestions>, (Status, Json<APIError>)> {
    let resp = FormManager::get_with_questions(db.inner(), form_id)
        .await
        .map_err(|e| APIError::report_internal_error("get form", e))?;

    Ok(Json(resp))
}
