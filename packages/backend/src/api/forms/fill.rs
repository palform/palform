use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIInternalError, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    api::error::APIError,
    auth::fill_access::APIFillAccessToken,
    captcha::requests::VerifiedCaptcha,
    crypto::submissions::CryptoSubmissionRepr,
    entity_managers::{forms::FormManager, submission::SubmissionManager},
    i18n::request::I18NManager,
    mail::client::PalformMailClient,
    pt,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormsFillPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormsFillRequest {
    data: String,
}

#[api_operation(tag = "Forms", operation_id = "forms.fill")]
pub async fn forms_fill(
    path: Path<FormsFillPath>,
    fill_access_token: APIFillAccessToken,
    data: Json<FormsFillRequest>,
    captcha: Option<VerifiedCaptcha>,
    db: Data<DatabaseConnection>,
    mail_client: Data<PalformMailClient>,
    i18n: I18NManager,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    if captcha.is_none()
        && FormManager::get_captcha_required(&txn, path.form_id)
            .await
            .map_internal_error()?
    {
        return Err(APIError::BadRequest(pt!(i18n, "fill_missing_captcha",)).into());
    }

    let data_repr = CryptoSubmissionRepr::from_pem_string(data.data.clone())
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    let data_bytes = data_repr
        .to_database_bytes()
        .map_err(|e| APIError::report_internal_error("Serialize message to bytes", e))?;

    let submission_id = SubmissionManager::create_submission(
        &txn,
        path.form_id,
        fill_access_token.token_id,
        data_bytes,
    )
    .await
    .map_internal_error()?;

    SubmissionManager::run_submission_notification(
        &txn,
        path.org_id,
        path.form_id,
        submission_id,
        mail_client.as_ref(),
    )
    .await
    .map_err(|e| APIError::report_internal_error("send submission notifications", e))?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
