use std::{collections::HashMap, time::SystemTimeError};

use chrono::{Duration, NaiveDateTime, Utc};
use palform_entities::{
    admin_user_second_authentication_factor, admin_user_second_authentication_factor_session,
    prelude::*,
};
use palform_migration::all;
use palform_tsid::{
    resources::{
        IDAdminUser, IDAdminUserSecondAuthenticationFactor,
        IDAdminUserSecondAuthenticationFactorSession,
    },
    tsid::PalformDatabaseID,
};
use sea_orm::{
    prelude::Uuid, ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, Order,
    QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde_json::Value;
use thiserror::Error;
use totp_rs::{Algorithm, Secret, SecretParseError, TotpUrlError, TOTP};
use webauthn_rs::{
    prelude::{
        CreationChallengeResponse, Passkey, PasskeyAuthentication, PasskeyRegistration,
        PublicKeyCredential, RegisterPublicKeyCredential, RequestChallengeResponse, WebauthnError,
    },
    Webauthn, WebauthnBuilder,
};

use crate::{
    api_entities::admin_user_second_factor::{
        APIAdminUserSecondAuthenticationFactor, APIAdminUserSecondAuthenticationFactorMethod,
    },
    config::Config,
};

use super::admin_users::AdminUserManager;

const SESSION_DURATION_MINS: u8 = 10;

pub struct AdminUserSecondFactorManager {
    user_id: PalformDatabaseID<IDAdminUser>,
    webauthn: Webauthn,
}

#[derive(Debug, Error)]
pub enum ProcessWebauthnError {
    #[error("{0}")]
    DBError(#[from] DbErr),
    #[error("Webauthn: {0}")]
    Webauthn(#[from] WebauthnError),
    #[error("Serialization: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid config: {0}")]
    Config(String),
    #[error("Session invalid: {0}")]
    Session(String),
    #[error("Credential: {0}")]
    Credential(String),
}

#[derive(Debug, Error)]
pub enum VerifyTOTPError {
    #[error("{0}")]
    DBError(#[from] DbErr),
    #[error("Your session has expired, please sign in again.")]
    SessionExpired,
    #[error("Your session was not found, please sign in again.")]
    SessionNotFound,
    #[error("TOTP setup: {0}")]
    TotpUrl(#[from] TotpUrlError),
    #[error("TOTP secret: {0}")]
    TotpSecretParse(#[from] SecretParseError),
    #[error("Getting current time: {0}")]
    TotpTime(#[from] SystemTimeError),
}

impl AdminUserSecondFactorManager {
    pub fn new(
        user_id: PalformDatabaseID<IDAdminUser>,
        config: &Config,
    ) -> Result<Self, ProcessWebauthnError> {
        let builder = WebauthnBuilder::new(
            config
                .frontend_url
                .host_str()
                .ok_or(ProcessWebauthnError::Config("URL missing host".to_string()))?,
            &config.frontend_url,
        )?;
        let webauthn = builder.build()?;

        Ok(Self { user_id, webauthn })
    }

    pub async fn list<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<Vec<APIAdminUserSecondAuthenticationFactor>, DbErr> {
        Ok(AdminUserSecondAuthenticationFactor::find()
            .filter(admin_user_second_authentication_factor::Column::UserId.eq(self.user_id))
            .order_by(
                admin_user_second_authentication_factor::Column::CreatedAt,
                Order::Desc,
            )
            .all(conn)
            .await?
            .iter()
            .map(|f| APIAdminUserSecondAuthenticationFactor {
                id: f.id,
                nickname: f.nickname.clone(),
                created_at: f.created_at,
                method: if f.webauthn_public_key.is_some() {
                    APIAdminUserSecondAuthenticationFactorMethod::Webauthn
                } else {
                    APIAdminUserSecondAuthenticationFactorMethod::TOTP
                },
            })
            .collect())
    }

    fn user_id_to_passkey_id(user_id: PalformDatabaseID<IDAdminUser>) -> Uuid {
        Uuid::from_u128(user_id.number().into())
    }

    fn expires_at() -> NaiveDateTime {
        (Utc::now() + Duration::minutes(SESSION_DURATION_MINS.into())).naive_utc()
    }

    pub async fn start_webauthn_register<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<
        (
            CreationChallengeResponse,
            PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
        ),
        ProcessWebauthnError,
    > {
        let user = AdminUserManager::get_user_by_id(conn, self.user_id)
            .await?
            .ok_or(DbErr::RecordNotFound("User".to_string()))?;

        let (ccr, skr) = self.webauthn.start_passkey_registration(
            Self::user_id_to_passkey_id(self.user_id),
            user.email.as_str(),
            user.display_name.unwrap_or(user.email.clone()).as_str(),
            None,
        )?;

        let encoded_secret = serde_json::to_value(skr)?;

        let new_session_id =
            PalformDatabaseID::<IDAdminUserSecondAuthenticationFactorSession>::random();
        let new_session = admin_user_second_authentication_factor_session::ActiveModel {
            id: Set(new_session_id),
            user_id: Set(self.user_id),
            expires_at: Set(Self::expires_at()),
            webauthn_secret: Set(Some(encoded_secret)),
            is_create: Set(true),
            ..Default::default()
        };

        new_session.insert(conn).await?;
        Ok((ccr, new_session_id))
    }

    pub async fn register_webauthn<T: ConnectionTrait>(
        &self,
        conn: &T,
        nickname: String,
        session_id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
        credential: RegisterPublicKeyCredential,
    ) -> Result<PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>, ProcessWebauthnError>
    {
        let session = AdminUserSecondAuthenticationFactorSession::find_by_id(session_id)
            .one(conn)
            .await?
            .ok_or(DbErr::RecordNotFound("Session".to_string()))?;

        if !session.is_create || session.webauthn_secret.is_none() {
            return Err(ProcessWebauthnError::Session(
                "not for webauthn enroll".to_string(),
            ));
        }

        let secret = session.webauthn_secret.unwrap();
        let secret: PasskeyRegistration = serde_json::from_value(secret)?;

        let passkey = self
            .webauthn
            .finish_passkey_registration(&credential, &secret)?;
        let passkey = serde_json::to_value(passkey)?;
        println!("{}", passkey.to_string());

        let new_id = PalformDatabaseID::<IDAdminUserSecondAuthenticationFactor>::random();
        let new_factor = admin_user_second_authentication_factor::ActiveModel {
            id: Set(new_id),
            user_id: Set(self.user_id),
            nickname: Set(nickname),
            webauthn_public_key: Set(Some(passkey)),
            ..Default::default()
        };
        new_factor.insert(conn).await?;

        Ok(new_id)
    }

    pub async fn register_totp<T: ConnectionTrait>(
        &self,
        conn: &T,
        nickname: String,
        secret: String,
    ) -> Result<PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>, DbErr> {
        let new_id = PalformDatabaseID::<IDAdminUserSecondAuthenticationFactor>::random();
        let new_factor = admin_user_second_authentication_factor::ActiveModel {
            id: Set(new_id),
            user_id: Set(self.user_id),
            nickname: Set(nickname),
            totp_secret: Set(Some(secret)),
            ..Default::default()
        };
        new_factor.insert(conn).await?;
        Ok(new_id)
    }

    pub async fn delete<T: ConnectionTrait>(
        &self,
        conn: &T,
        id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>,
    ) -> Result<(), DbErr> {
        AdminUserSecondAuthenticationFactor::delete_by_id(id)
            .filter(admin_user_second_authentication_factor::Column::UserId.eq(self.user_id))
            .exec(conn)
            .await?;
        Ok(())
    }

    pub async fn user_requires_2fa<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<(bool, bool), DbErr> {
        let methods: Vec<(Option<String>, Option<Value>)> =
            AdminUserSecondAuthenticationFactor::find()
                .filter(admin_user_second_authentication_factor::Column::UserId.eq(self.user_id))
                .select_only()
                .column(admin_user_second_authentication_factor::Column::TotpSecret)
                .column(admin_user_second_authentication_factor::Column::WebauthnPublicKey)
                .into_tuple()
                .all(conn)
                .await?;

        let requires_totp = methods.iter().any(|(totp, _)| totp.is_some());
        let requires_webauthn = methods.iter().any(|(_, webauthn)| webauthn.is_some());

        Ok((requires_totp, requires_webauthn))
    }

    async fn get_passkeys<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<
        HashMap<PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>, Passkey>,
        ProcessWebauthnError,
    > {
        let webauthn_methods = AdminUserSecondAuthenticationFactor::find()
            .filter(all![
                admin_user_second_authentication_factor::Column::UserId.eq(self.user_id),
                admin_user_second_authentication_factor::Column::WebauthnPublicKey.is_not_null()
            ])
            .all(conn)
            .await?;

        webauthn_methods
            .iter()
            .map(
                |f| -> Result<
                    (
                        PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>,
                        Passkey,
                    ),
                    ProcessWebauthnError,
                > {
                    let passkey = serde_json::from_value::<Passkey>(
                        f.webauthn_public_key
                            .clone()
                            .expect("sql-filtered non-null value to be Some"),
                    )?;
                    Ok((f.id, passkey))
                },
            )
            .collect()
    }

    pub async fn create_auth_session<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<
        (
            PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
            Option<RequestChallengeResponse>,
        ),
        ProcessWebauthnError,
    > {
        let new_id = PalformDatabaseID::<IDAdminUserSecondAuthenticationFactorSession>::random();
        let mut new_session = admin_user_second_authentication_factor_session::ActiveModel {
            id: Set(new_id),
            user_id: Set(self.user_id),
            expires_at: Set(Self::expires_at()),
            is_create: Set(false),
            ..Default::default()
        };

        let mut rcr = Option::<RequestChallengeResponse>::None;
        let passkeys = self.get_passkeys(conn).await?;
        if !passkeys.is_empty() {
            let passkeys: Vec<Passkey> = passkeys.values().cloned().collect();
            let (_rcr, skr) = self.webauthn.start_passkey_authentication(&passkeys)?;
            rcr = Some(_rcr);
            let skr = serde_json::to_value(skr)?;
            new_session.webauthn_secret = Set(Some(skr));
        }

        new_session.insert(conn).await?;
        Ok((new_id, rcr))
    }

    pub async fn delete_auth_session<T: ConnectionTrait>(
        conn: &T,
        id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
    ) -> Result<(), DbErr> {
        AdminUserSecondAuthenticationFactorSession::delete_by_id(id)
            .exec(conn)
            .await?;
        Ok(())
    }

    pub async fn verify_totp_session<T: ConnectionTrait>(
        conn: &T,
        session_id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
        token: String,
    ) -> Result<(bool, PalformDatabaseID<IDAdminUser>), VerifyTOTPError> {
        let session = AdminUserSecondAuthenticationFactorSession::find_by_id(session_id)
            .one(conn)
            .await?
            .ok_or(VerifyTOTPError::SessionNotFound)?;

        if session.expires_at < Utc::now().naive_utc() {
            return Err(VerifyTOTPError::SessionExpired);
        }

        if session.is_create {
            return Err(VerifyTOTPError::SessionNotFound);
        }

        let secrets: Vec<Option<String>> = AdminUserSecondAuthenticationFactor::find()
            .filter(all![
                admin_user_second_authentication_factor::Column::UserId.eq(session.user_id),
                admin_user_second_authentication_factor::Column::TotpSecret.is_not_null()
            ])
            .select_only()
            .column(admin_user_second_authentication_factor::Column::TotpSecret)
            .into_tuple()
            .all(conn)
            .await?;

        for secret in secrets {
            let totp = TOTP::new(
                Algorithm::SHA1,
                6,
                1,
                30,
                Secret::Encoded(secret.expect("sql-filtered non-null value to be Some"))
                    .to_bytes()?,
            )?;
            if totp.check_current(token.as_str())? {
                return Ok((true, session.user_id));
            }
        }

        Ok((false, session.user_id))
    }

    pub async fn verify_webauthn_session<T: ConnectionTrait>(
        conn: &T,
        config: &Config,
        session_id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
        pkc: PublicKeyCredential,
    ) -> Result<(bool, PalformDatabaseID<IDAdminUser>), ProcessWebauthnError> {
        let session = AdminUserSecondAuthenticationFactorSession::find_by_id(session_id)
            .one(conn)
            .await?
            .ok_or(ProcessWebauthnError::Session("not found. Please sign in again.".to_string()))?;

        if session.expires_at < Utc::now().naive_utc() {
            return Err(ProcessWebauthnError::Session("expired. Please sign in again.".to_string()));
        }

        if session.webauthn_secret.is_none() {
            return Err(ProcessWebauthnError::Session("not webauthn".to_string()))?;
        }

        if session.is_create {
            return Err(ProcessWebauthnError::Session(
                "not an authentication session".to_string(),
            ))?;
        }

        let s = Self::new(session.user_id, config)?;
        let passkeys = s.get_passkeys(conn).await?;
        let skr =
            serde_json::from_value::<PasskeyAuthentication>(session.webauthn_secret.unwrap())?;

        let result = s.webauthn.finish_passkey_authentication(&pkc, &skr);

        if let Ok(result) = result {
            let (factor_id, passkey) = passkeys
                .iter()
                .find(|(_, p)| p.cred_id() == result.cred_id())
                .ok_or(ProcessWebauthnError::Credential(
                    "Passkey not found".to_string(),
                ))?
                .clone();

            let mut passkey = passkey.to_owned();
            if result.needs_update() {
                let updated = passkey.update_credential(&result);
                if updated.is_some_and(|v| v) {
                    let passkey = serde_json::to_value(passkey)?;

                    admin_user_second_authentication_factor::ActiveModel {
                        id: Set(factor_id.to_owned()),
                        webauthn_public_key: Set(Some(passkey)),
                        ..Default::default()
                    }
                    .update(conn)
                    .await?;
                }
            }

            return Ok((true, session.user_id));
        } else {
            return Ok((false, session.user_id));
        }
    }
}
