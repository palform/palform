use palform_client_common::errors::error::APIInternalErrorResult;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::api::error::APIError;
use crate::api_entities::org::APIOrganisation;
use crate::auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourceAny};
use crate::entity_managers::orgs::OrganisationManager;

/// List organisation
///
/// List all the organisations that the authenticated user is a member of
#[openapi(tag = "Organisations", operation_id = "orgs.list")]
#[get("/users/me/orgs")]
pub async fn handler(
    token: APIAuthToken<APIAuthTokenSourceAny>,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIOrganisation>>, (Status, Json<APIError>)> {
    let orgs = OrganisationManager::list_orgs_for_user(db.inner(), token.get_user_id())
        .await
        .map_internal_error()?;

    Ok(Json(orgs))
}
