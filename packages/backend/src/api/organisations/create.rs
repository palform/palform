use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{http::Status, post, serde::json::Json, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;
use validator::Validate;

use crate::{
    api::error::{APIError, APIInternalError},
    auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal},
    entity_managers::orgs::OrganisationManager,
    rocket_util::validated::Validated,
};

#[derive(Deserialize, JsonSchema, Validate)]
pub struct NewOrganisationRequest {
    #[validate(length(min = 1, max = 40, message = "must be between 1 and 20 characters"))]
    pub display_name: String,
}

#[openapi(tag = "Organisations", operation_id = "orgs.create")]
#[post("/users/me/orgs", data = "<data>")]
pub async fn handler(
    data: Validated<Json<NewOrganisationRequest>>,
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: &State<DatabaseConnection>,
    stripe: &State<stripe::Client>,
) -> Result<Json<PalformDatabaseID<IDOrganisation>>, (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    let org_id = OrganisationManager::create(&txn, data.display_name.clone())
        .await
        .map_err(|e| e.to_internal_error())?;

    OrganisationManager::bootstrap_new_org(
        &txn,
        org_id,
        token.get_user_id(),
        #[cfg(feature = "saas")]
        stripe,
    )
    .await
    .map_err(|e| APIError::report_internal_error("bootstrap org", e))?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(Json(org_id))
}
