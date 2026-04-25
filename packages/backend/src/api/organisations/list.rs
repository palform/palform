use actix_web::web::{Data, Json};
use apistos::api_operation;
use palform_client_common::errors::error::APIInternalErrorResult;
use sea_orm::DatabaseConnection;

use crate::api::error::APIError;
use crate::api_entities::org::APIOrganisation;
use crate::auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourceAny};
use crate::entity_managers::orgs::OrganisationManager;

/// List organisation
///
/// List all the organisations that the authenticated user is a member of
#[api_operation(tag = "Organisations", operation_id = "orgs.list")]
pub async fn organisations_list(
    token: APIAuthToken<APIAuthTokenSourceAny>,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIOrganisation>>, APIError> {
    let orgs = OrganisationManager::list_orgs_for_user(db.as_ref(), token.get_user_id())
        .await
        .map_internal_error()?;

    Ok(Json(orgs))
}
