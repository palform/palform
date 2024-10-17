use std::time::SystemTimeError;

use chrono::{Duration, Utc};
use palform_entities::{
    admin_user_second_authentication_factor, admin_user_second_authentication_factor_session,
    prelude::*,
};
use palform_tsid::{
    resources::{
        IDAdminUser, IDAdminUserSecondAuthenticationFactor,
        IDAdminUserSecondAuthenticationFactorSession,
    },
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, Order, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};
use thiserror::Error;
use totp_rs::{Algorithm, Secret, SecretParseError, TotpUrlError, TOTP};

use crate::api_entities::admin_user_second_factor::APIAdminUserSecondAuthenticationFactor;

pub struct AdminUserSecondFactorManager {
    user_id: PalformDatabaseID<IDAdminUser>,
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
    pub fn new(user_id: PalformDatabaseID<IDAdminUser>) -> Self {
        Self { user_id }
    }

    pub async fn list<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<Vec<APIAdminUserSecondAuthenticationFactor>, DbErr> {
        AdminUserSecondAuthenticationFactor::find()
            .filter(admin_user_second_authentication_factor::Column::UserId.eq(self.user_id))
            .order_by(
                admin_user_second_authentication_factor::Column::CreatedAt,
                Order::Desc,
            )
            .into_model()
            .all(conn)
            .await
    }

    pub async fn create<T: ConnectionTrait>(
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
            totp_secret: Set(secret),
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

    pub async fn user_requires_2fa<T: ConnectionTrait>(&self, conn: &T) -> Result<bool, DbErr> {
        AdminUserSecondAuthenticationFactor::find()
            .filter(admin_user_second_authentication_factor::Column::UserId.eq(self.user_id))
            .count(conn)
            .await
            .map(|c| c > 0)
    }

    pub async fn create_auth_session<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>, DbErr> {
        let new_id = PalformDatabaseID::<IDAdminUserSecondAuthenticationFactorSession>::random();
        let new_session = admin_user_second_authentication_factor_session::ActiveModel {
            id: Set(new_id),
            user_id: Set(self.user_id),
            expires_at: Set((Utc::now() + Duration::minutes(10)).naive_utc()),
            ..Default::default()
        };

        new_session.insert(conn).await?;
        Ok(new_id)
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

    pub async fn verify_auth_session<T: ConnectionTrait>(
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

        let secrets: Vec<String> = AdminUserSecondAuthenticationFactor::find()
            .filter(admin_user_second_authentication_factor::Column::UserId.eq(session.user_id))
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
                Secret::Encoded(secret).to_bytes()?,
            )?;
            if totp.check_current(token.as_str())? {
                return Ok((true, session.user_id));
            }
        }

        Ok((false, session.user_id))
    }
}
