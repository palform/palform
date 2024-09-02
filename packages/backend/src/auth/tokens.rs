use std::ops::Deref;

use base64::prelude::*;
use chrono::{Duration, NaiveDateTime, Utc};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::{admin_user, auth_token, prelude::*};
use palform_tsid::{
    resources::{IDAdminUser, IDAuthToken},
    tsid::PalformDatabaseID,
};
use rand::distributions::{Alphanumeric, DistString};
use rocket::{
    request::{self, FromRequest},
    serde::json::Json,
};
use rocket_okapi::{okapi::schemars, request::OpenApiFromRequest};
use rocket_okapi::{
    okapi::{
        openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData},
        schemars::JsonSchema,
    },
    request::RequestHeaderInput,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr,
    EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{entity_managers::admin_users::AdminUserManager, into_outcome};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct NewAPIAuthToken {
    pub id: PalformDatabaseID<IDAuthToken>,
    pub secret: String,
    pub expires_at: NaiveDateTime,
}

pub trait APIAuthTokenSource: Clone + Send {
    fn allow_service_account() -> bool;
    fn allow_personal() -> bool;

    fn from_personal(model: auth_token::Model) -> Self;
    fn from_service_account(model: admin_user::Model) -> Self;
    fn get_user_id(&self) -> PalformDatabaseID<IDAdminUser>;

    fn to_string() -> String;
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
        self.model.user_id.clone()
    }
    fn to_string() -> String {
        "Personal".to_string()
    }
}

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
        self.model.id.clone()
    }
    fn to_string() -> String {
        "ServiceAccount".to_string()
    }
}

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
            Self::ServiceAccount(m) => m.id.clone(),
            Self::Personal(m) => m.user_id.clone(),
        }
    }
    fn to_string() -> String {
        "Any".to_string()
    }
}

#[derive(Clone)]
pub struct APIAuthToken<Source: APIAuthTokenSource> {
    pub source: Source,
}

impl<Source: APIAuthTokenSource> Deref for APIAuthToken<Source> {
    type Target = Source;
    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

#[rocket::async_trait]
impl<'a, Source: APIAuthTokenSource> FromRequest<'a> for APIAuthToken<Source> {
    type Error = Json<APIError>;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        let authorization_header = into_outcome!(
            request
                .headers()
                .get_one("Authorization")
                .ok_or(APIError::BadRequest(
                    "Missing Authorization header".to_string()
                )),
            request
        );

        let db = into_outcome!(
            request
                .rocket()
                .state::<DatabaseConnection>()
                .ok_or_else(|| APIError::report_internal_error_without_error(
                    "Database not found in state"
                )),
            request
        );

        let basic_header = authorization_header.strip_prefix("Basic ");
        let bearer_header = authorization_header.strip_prefix("Bearer ");

        if Source::allow_personal() {
            if let Some(basic_header) = basic_header {
                let (token_id, token_secret) = into_outcome!(
                    BASE64_STANDARD
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
                                        .and_then(|token_secret| {
                                            Ok((token_id.to_owned(), token_secret.to_owned()))
                                        })
                                })
                        }),
                    request
                );

                let parsed_token_id = into_outcome!(
                    PalformDatabaseID::<IDAuthToken>::from_str(&token_id)
                        .map_err(|e| APIError::BadRequest(e.to_string())),
                    request
                );

                let token = into_outcome!(
                    into_outcome!(
                        TokenManager::lookup_by_id(db, parsed_token_id)
                            .await
                            .map_internal_error(),
                        request
                    )
                    .ok_or(APIError::NotAllowed),
                    request
                );

                if token.hash != token_secret {
                    into_outcome!(Err(APIError::NotAllowed), request)
                }

                if token.expires_at < Utc::now().naive_utc() {
                    into_outcome!(Err(APIError::NotAllowed), request);
                }

                return request::Outcome::Success(Self {
                    source: Source::from_personal(token),
                });
            }
        }

        if Source::allow_service_account() {
            if let Some(bearer_header) = bearer_header {
                let matching_user = into_outcome!(
                    AdminUserManager::get_user_by_service_account_token(
                        db,
                        bearer_header.to_string()
                    )
                    .await
                    .map_internal_error(),
                    request
                );

                let matching_user =
                    into_outcome!(matching_user.ok_or(APIError::NotAllowed), request);

                return request::Outcome::Success(Self {
                    source: Source::from_service_account(matching_user),
                });
            }
        }

        into_outcome!(Err(APIError::NotAllowed), request)
    }
}

impl<'a, Source: APIAuthTokenSource> OpenApiFromRequest<'a> for APIAuthToken<Source> {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        let mut description = "Requires authentication to access.".to_string();

        if Source::allow_personal() {
            description = description + " Allows frontend user tokens (HTTP basic auth) generated with the authentication endpoints.";
        }
        if Source::allow_service_account() {
            description = description + " Allows service-account tokens via the public Palform API (HTTP bearer auth). Please use the format `Bearer {service_account_token}`.";
        }

        let security_scheme = SecurityScheme {
            description: Some(description),
            data: SecuritySchemeData::Http {
                scheme: if Source::allow_personal() {
                    "basic".to_owned()
                } else {
                    "bearer".to_owned()
                },
                bearer_format: if Source::allow_service_account() {
                    Some("Service Account authentication token".to_owned())
                } else {
                    None
                },
            },
            extensions: Object::default(),
        };

        let mut security_req = SecurityRequirement::new();
        security_req.insert("APIAuthToken".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "APIAuthToken".to_owned(),
            security_scheme,
            security_req,
        ))
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
    ) -> Result<NewAPIAuthToken, IssueTokenError> {
        let expires_at = Utc::now() + Duration::hours(3);

        let token_id = PalformDatabaseID::<IDAuthToken>::random();
        let token_secret = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

        let new_token = auth_token::ActiveModel {
            id: Set(token_id.clone()),
            created_at: NotSet,
            expires_at: Set(expires_at.naive_utc()),
            hash: Set(token_secret.clone()),
            user_id: Set(user_id),
        };
        new_token.insert(conn).await?;

        Ok(NewAPIAuthToken {
            id: token_id,
            secret: token_secret,
            expires_at: expires_at.naive_utc(),
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
