use log::warn;
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::api::error::APIError;
use crate::api_entities::form::APIForm;
use crate::auth::rbac::requests::APITokenOrgViewer;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::forms::FormManager;

#[openapi(tag = "Forms", operation_id = "forms.list")]
#[get("/users/me/orgs/<org_id>/forms")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIForm>>, (Status, Json<APIError>)> {
    let forms = FormManager::list_forms_in_my_teams(db.inner(), token.get_user_id(), org_id)
        .await
        .map_err(|e| {
            warn!("List forms in org: {}", e.to_string());
            APIError::Internal
        })?;

    Ok(Json(forms))
}
