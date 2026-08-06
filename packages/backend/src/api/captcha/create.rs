use std::{net::IpAddr, str::FromStr};

use actix_web::{
    web::{Data, Json},
    HttpRequest,
};
use apistos::api_operation;
use palform_client_common::errors::error::APIError;

use crate::{
    api_entities::captcha::APICaptchaChallenge, captcha::manager::CaptchaManager, config::Config,
    memory_db::memory_db::MemoryDB,
};

#[api_operation(tag = "Captcha", operation_id = "captcha.create")]
pub async fn captcha_create(
    memory_db: Data<MemoryDB>,
    config: Data<Config>,
    req: HttpRequest,
) -> Result<Json<APICaptchaChallenge>, APIError> {
    let connection_info = req.connection_info().to_owned();
    let client_ip = connection_info
        .realip_remote_addr()
        .ok_or(APIError::BadRequest("No IP address".to_string()))?;
    let client_ip =
        IpAddr::from_str(client_ip).map_err(|e| APIError::report_internal_error("Parse IP", e))?;

    let challenge = CaptchaManager::create_challenge(&client_ip, &memory_db, &config)
        .await
        .map_err(|e| APIError::report_internal_error("create captcha challenge", e))?;

    Ok(Json(APICaptchaChallenge { challenge }))
}
