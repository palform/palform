use std::{future::Future, ops::Deref, pin::Pin};

use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use apistos::ApiSecurity;
use base64::prelude::*;
use chrono::{DateTime, Duration, Utc};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::{admin_user, auth_token, prelude::*};
use palform_tsid::{
    resources::{IDAdminUser, IDAuthToken},
    tsid::PalformDatabaseID,
};
use rand::distr::{Alphanumeric, SampleString};
use schemars::JsonSchema;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr,
    EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{config::Config, entity_managers::admin_users::AdminUserManager};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct NewAPIAuthToken {
    pub id: PalformDatabaseID<IDAuthToken>,
    pub secret: String,
    pub expires_at: DateTime<Utc>,
}

pub trait APIAuthTokenSource: Clone + Send {
    fn allow_service_account() -> bool;
    fn allow_personal() -> bool;

    fn from_personal(model: auth_token::Model) -> Self;
    fn from_service_account(model: admin_user::Model) -> Self;
    fn get_user_id(&self) -> PalformDatabaseID<IDAdminUser>;
}

#[derive(Clone)]
pub struct APIAuthTokenSourcePersonal {
    pub model: auth_token::Model,
}
impl APIAuthTokenSource for APIAuthTokenSourcePersonal {
    fn allow_service_account() -> bool {
        false
    }
    fn allow_personal() -> bool {
        true
    }
    fn from_personal(model: auth_token::Model) -> Self {
        Self { model }
    }
    fn from_service_account(_: admin_user::Model) -> Self {
        unimplemented!("Cannot construct Personal token for ServiceAccount")
    }
    fn get_user_id(&self) -> PalformDatabaseID<IDAdminUser> {
        self.model.user_id
    }
}

/*
#[derive(Clone)]
pub struct APIAuthTokenSourceServiceAccount {
    pub model: admin_user::Model,
}
impl APIAuthTokenSource for APIAuthTokenSourceServiceAccount {
    fn allow_service_account() -> bool {
        true
    }
    fn allow_personal() -> bool {
        false
    }
    fn from_personal(_: auth_token::Model) -> Self {
        unimplemented!("Cannot construct ServiceAccount token for Personal")
    }
    fn from_service_account(model: admin_user::Model) -> Self {
        Self { model }
    }
    fn get_user_id(&self) -> PalformDatabaseID<IDAdminUser> {
        self.model.id
    }
}*/

#[derive(Clone)]
pub enum APIAuthTokenSourceAny {
    Personal(auth_token::Model),
    ServiceAccount(admin_user::Model),
}
impl APIAuthTokenSource for APIAuthTokenSourceAny {
    fn allow_personal() -> bool {
        true
    }
    fn allow_service_account() -> bool {
        true
    }
    fn from_personal(model: auth_token::Model) -> Self {
        Self::Personal(model)
    }
    fn from_service_account(model: admin_user::Model) -> Self {
        Self::ServiceAccount(model)
    }
    fn get_user_id(&self) -> PalformDatabaseID<IDAdminUser> {
        match self {
            Self::ServiceAccount(m) => m.id,
            Self::Personal(m) => m.user_id,
        }
    }
}

#[derive(Clone, ApiSecurity)]
#[openapi_security(scheme(security_type(http(scheme = "basic"))))]
pub struct APIAuthToken<Source: APIAuthTokenSource> {
    pub source: Source,
}

impl<Source: APIAuthTokenSource> Deref for APIAuthToken<Source> {
    type Target = Source;
    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl<Source: APIAuthTokenSource> FromRequest for APIAuthToken<Source> {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let authorization_header = req
                .headers()
                .get("Authorization")
                .ok_or(APIError::BadRequest(
                    "Missing Authorization header".to_string(),
                ))?
                .to_str()
                .map_err(|e| {
                    APIError::BadRequest(format!("Authorization header: {}", e.to_string()))
                })?;

            let db = req.app_data::<Data<DatabaseConnection>>().ok_or_else(|| {
                APIError::report_internal_error_without_error("DB not found in state")
            })?;

            let basic_header = authorization_header.strip_prefix("Basic ");
            let bearer_header = authorization_header.strip_prefix("Bearer ");

            if Source::allow_personal() {
                if let Some(basic_header) = basic_header {
                    let (token_id, token_secret) = BASE64_STANDARD
                        .decode(basic_header)
                        .map_err(|e| {
                            APIError::BadRequest(format!(
                                "Failed to parse Basic Authorization header: {}",
                                e
                            ))
                        })
                        .and_then(|v| {
                            String::from_utf8(v).map_err(|e| {
                                APIError::BadRequest(format!(
                                    "Failed to decode Basic Authorization data: {}",
                                    e
                                ))
                            })
                        })
                        .and_then(|basic_auth_string| {
                            let mut split = basic_auth_string.split(':');
                            split
                                .next()
                                .ok_or(APIError::BadRequest(
                                    "Token ID not in Authorization header".to_string(),
                                ))
                                .and_then(|token_id| {
                                    split
                                        .next()
                                        .ok_or(APIError::BadRequest(
                                            "Token secret not in Authorization header".to_string(),
                                        ))
                                        .map(|token_secret| {
                                            (token_id.to_owned(), token_secret.to_owned())
                                        })
                                })
                        })?;

                    let parsed_token_id: PalformDatabaseID<IDAuthToken> = token_id
                        .parse::<PalformDatabaseID<IDAuthToken>>()
                        .map_err(|e| APIError::BadRequest(e.to_string()))?;

                    let token = TokenManager::lookup_by_id(db.as_ref(), parsed_token_id)
                        .await
                        .map_internal_error()?
                        .ok_or(APIError::NotAllowed)?;

                    if token.hash != token_secret {
                        return Err(APIError::NotAllowed);
                    }

                    if token.expires_at < Utc::now() {
                        return Err(APIError::NotAllowed);
                    }

                    Ok(Self {
                        source: Source::from_personal(token),
                    })
                } else {
                    Err(APIError::NotAllowed)
                }
            } else if Source::allow_service_account() {
                if let Some(bearer_header) = bearer_header {
                    let matching_user = AdminUserManager::get_user_by_service_account_token(
                        db.as_ref(),
                        bearer_header.to_string(),
                    )
                    .await
                    .map_internal_error()?
                    .ok_or(APIError::NotAllowed)?;

                    Ok(Self {
                        source: Source::from_service_account(matching_user),
                    })
                } else {
                    Err(APIError::NotAllowed)
                }
            } else {
                Err(APIError::NotAllowed)
            }
        })
    }
}

#[derive(Debug, Error)]
pub enum IssueTokenError {
    #[error("Inserting token record: {0}")]
    InsertError(#[from] DbErr),
}

pub struct TokenManager;
impl TokenManager {
    async fn lookup_by_id<T: ConnectionTrait>(
        conn: &T,
        token_id: PalformDatabaseID<IDAuthToken>,
    ) -> Result<Option<auth_token::Model>, DbErr> {
        AuthToken::find_by_id(token_id).one(conn).await
    }

    pub async fn issue_token<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        config: &Config,
    ) -> Result<NewAPIAuthToken, IssueTokenError> {
        let expires_at = Utc::now() + Duration::hours(i64::from(config.auth_token_expiry_hours));

        let token_id = PalformDatabaseID::<IDAuthToken>::random();
        let token_secret = Alphanumeric.sample_string(&mut rand::rng(), 32);

        let new_token = auth_token::ActiveModel {
            id: Set(token_id),
            created_at: NotSet,
            expires_at: Set(expires_at.fixed_offset()),
            hash: Set(token_secret.clone()),
            user_id: Set(user_id),
        };
        new_token.insert(conn).await?;

        Ok(NewAPIAuthToken {
            id: token_id,
            secret: token_secret,
            expires_at: expires_at,
        })
    }

    pub async fn delete_all_old_tokens<T: ConnectionTrait>(conn: &T) -> Result<(), DbErr> {
        let now = Utc::now().naive_utc();
        AuthToken::delete_many()
            .filter(auth_token::Column::ExpiresAt.lt(now))
            .exec(conn)
            .await
            .map(|_| ())
    }

    pub async fn delete_token_by_id<T: ConnectionTrait>(
        conn: &T,
        token_id: PalformDatabaseID<IDAuthToken>,
    ) -> Result<(), DbErr> {
        AuthToken::delete_by_id(token_id)
            .exec(conn)
            .await
            .map(|_| ())
    }
}
