use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use log::warn;
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::api::error::APIError;
use crate::api_entities::form::APIForm;
use crate::auth::rbac::requests::APITokenOrgViewer;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::forms::FormManager;

#[derive(Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct FormsListPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Forms", operation_id = "forms.list")]
pub async fn forms_list(
    path: Path<FormsListPath>,
    token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIForm>>, APIError> {
    let forms = FormManager::list_forms_in_my_teams(db.as_ref(), token.get_user_id(), path.org_id)
        .await
        .map_err(|e| {
            warn!("List forms in org: {}", e.to_string());
            APIError::Internal
        })?;

    Ok(Json(forms))
}
