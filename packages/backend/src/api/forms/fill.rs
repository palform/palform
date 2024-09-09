use palform_client_common::errors::error::APIInternalErrorResult;
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use rocket::{http::Status, post, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api::error::APIError,
    auth::fill_access::APIFillAccessToken,
    captcha::VerifiedCaptcha,
    crypto::submissions::CryptoSubmissionRepr,
    entity_managers::{
        forms::FormManager,
        orgs::{OrganisationManager, OrganisationSubmissionBehaviour},
        submission::SubmissionManager,
    },
    i18n::request::I18NManager,
    mail::client::PalformMailClient,
    pt,
};

#[openapi(tag = "Forms", operation_id = "forms.fill")]
#[post("/fill/orgs/<org_id>/forms/<form_id>", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    fill_access_token: APIFillAccessToken,
    data: String,
    captcha: Option<VerifiedCaptcha>,
    db: &State<DatabaseConnection>,
    mail_client: &State<PalformMailClient>,
    i18n: I18NManager,
) -> Result<(), (Status, Json<APIError>)> {
    let submission_behaviour = OrganisationManager::get_org_submission_block(db.inner(), org_id)
        .await
        .map_internal_error()?;

    if submission_behaviour == OrganisationSubmissionBehaviour::Block {
        return Err(APIError::BadRequest(pt!(i18n, "fill_response_limit",)).into());
    }

    if captcha.is_none()
        && FormManager::get_captcha_required(db.inner(), form_id)
            .await
            .map_internal_error()?
    {
        return Err(APIError::BadRequest(pt!(i18n, "fill_missing_captcha",)).into());
    }

    let data_repr = CryptoSubmissionRepr::from_pem_string(data.clone())
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    let data_bytes = data_repr
        .to_database_bytes()
        .map_err(|e| APIError::report_internal_error("Serialize message to bytes", e))?;

    let submission_id = SubmissionManager::create_submission(
        db.inner(),
        form_id,
        fill_access_token.token_id,
        data_bytes,
    )
    .await
    .map_internal_error()?;

    SubmissionManager::run_submission_notification(
        db.inner(),
        org_id,
        form_id,
        submission_id,
        data,
        mail_client,
    )
    .await
    .map_err(|e| APIError::report_internal_error("send submission notifications", e))?;

    Ok(())
}
