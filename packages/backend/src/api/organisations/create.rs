use actix_web::web::{Data, Json};
use apistos::{api_operation, ApiComponent};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;
use validator::Validate;

use crate::{
    actix_util::validated::Validated,
    api::error::{APIError, APIInternalError},
    auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal},
    entity_managers::orgs::OrganisationManager,
};

#[derive(Deserialize, JsonSchema, Validate, ApiComponent)]
pub struct NewOrganisationRequest {
    #[validate(length(min = 1, max = 40, message = "must be between 1 and 20 characters"))]
    pub display_name: String,
}

#[api_operation(tag = "Organisations", operation_id = "orgs.create")]
pub async fn organisations_create(
    data: Validated<Json<NewOrganisationRequest>>,
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: Data<DatabaseConnection>,
    stripe: Data<stripe::Client>,
) -> Result<Json<PalformDatabaseID<IDOrganisation>>, APIError> {
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
        stripe.as_ref(),
    )
    .await
    .map_err(|e| APIError::report_internal_error("bootstrap org", e))?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(Json(org_id))
}
