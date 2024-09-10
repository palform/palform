use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use palform_entities::{admin_user, prelude::*};
use palform_tsid::{
    resources::{IDAdminUser, IDOrganisation},
    tsid::PalformDatabaseID,
};
use rand::rngs::OsRng;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, QueryFilter, Set,
};
use thiserror::Error;

use crate::api_entities::admin_users::APIAdminUser;

#[derive(Debug, Error)]
pub enum AdminUserManagementError {
    #[error("{0}")]
    DBError(#[from] DbErr),
    #[error("Hashing: {0}")]
    Password(argon2::password_hash::Error),
    #[error("Your account does not have a password.")]
    NoPassword,
    #[error("Your account is connected to an organisation. Please sign in through it instead.")]
    OrgOnly,
}

pub struct AdminUserManager;

impl AdminUserManager {
    pub async fn get_user_by_id<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<Option<APIAdminUser>, DbErr> {
        AdminUser::find_by_id(user_id).into_model().one(conn).await
    }

    pub async fn get_user_by_sub<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        sub: String,
    ) -> Result<Option<APIAdminUser>, DbErr> {
        AdminUser::find()
            .filter(
                Condition::all()
                    .add(admin_user::Column::OrgAuthSub.eq(Some(sub)))
                    .add(admin_user::Column::OrgAuthOrganisationId.eq(Some(org_id))),
            )
            .into_model()
            .one(conn)
            .await
    }

    pub async fn get_user_by_email<T: ConnectionTrait>(
        conn: &T,
        email: String,
    ) -> Result<Option<admin_user::Model>, DbErr> {
        AdminUser::find()
            .filter(admin_user::Column::Email.eq(email))
            .one(conn)
            .await
    }

    pub async fn get_user_by_service_account_token<T: ConnectionTrait>(
        conn: &T,
        token: String,
    ) -> Result<Option<admin_user::Model>, DbErr> {
        AdminUser::find()
            .filter(admin_user::Column::ServiceAccountAuthToken.eq(Some(token)))
            .one(conn)
            .await
    }

    fn get_argon2<'k>() -> Argon2<'k> {
        Argon2::default()
    }

    fn gen_password(password: String) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let password = Self::get_argon2().hash_password(password.as_bytes(), &salt)?;
        Ok(password.to_string())
    }

    pub fn verify_user_password(
        user: &admin_user::Model,
        password: String,
    ) -> Result<bool, AdminUserManagementError> {
        if user.org_auth_sub.is_some() {
            return Err(AdminUserManagementError::OrgOnly);
        }

        let hash = user
            .manual_auth_password_hash
            .clone()
            .ok_or(AdminUserManagementError::NoPassword)?;
        let parsed_hash =
            PasswordHash::new(&hash).map_err(|e| AdminUserManagementError::Password(e))?;

        Ok(Self::get_argon2()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub async fn set_user_password<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        password: String,
    ) -> Result<(), AdminUserManagementError> {
        let password =
            Self::gen_password(password).map_err(|e| AdminUserManagementError::Password(e))?;
        let updated_user = admin_user::ActiveModel {
            id: Set(user_id),
            manual_auth_password_hash: Set(Some(password)),
            ..Default::default()
        };
        updated_user.update(conn).await?;
        Ok(())
    }

    pub async fn create_user<T: ConnectionTrait>(
        conn: &T,
        email: String,
        password: String,
    ) -> Result<PalformDatabaseID<IDAdminUser>, AdminUserManagementError> {
        let password_hash =
            Self::gen_password(password).map_err(|e| AdminUserManagementError::Password(e))?;

        let id = PalformDatabaseID::<IDAdminUser>::random();
        let new_user = admin_user::ActiveModel {
            id: Set(id.clone()),
            email: Set(email),
            manual_auth_email_verified: Set(Some(false)),
            manual_auth_password_hash: Set(Some(password_hash)),
            ..Default::default()
        };

        new_user.insert(conn).await?;
        Ok(id)
    }

    pub async fn create_user_for_org<T: ConnectionTrait>(
        conn: &T,
        display_name: String,
        email: String,
        org_id: PalformDatabaseID<IDOrganisation>,
        sub: String,
    ) -> Result<PalformDatabaseID<IDAdminUser>, AdminUserManagementError> {
        let id = PalformDatabaseID::<IDAdminUser>::random();
        let new_user = admin_user::ActiveModel {
            id: Set(id.clone()),
            display_name: Set(Some(display_name)),
            email: Set(email),
            org_auth_organisation_id: Set(Some(org_id)),
            org_auth_sub: Set(Some(sub)),
            ..Default::default()
        };

        new_user.insert(conn).await?;
        Ok(id)
    }

    pub async fn associate_user_with_org<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        org_id: PalformDatabaseID<IDOrganisation>,
        sub: String,
    ) -> Result<(), DbErr> {
        let updated_user = admin_user::ActiveModel {
            id: Set(user_id),
            org_auth_organisation_id: Set(Some(org_id)),
            org_auth_sub: Set(Some(sub)),
            manual_auth_password_hash: Set(None),
            manual_auth_email_verified: Set(None),
            ..Default::default()
        };

        updated_user.update(conn).await?;
        Ok(())
    }

    pub async fn mark_user_email_verified<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<(), DbErr> {
        let updated_user = admin_user::ActiveModel {
            id: Set(user_id),
            manual_auth_email_verified: Set(Some(true)),
            ..Default::default()
        };
        updated_user.update(conn).await.map(|_| ())
    }

    pub async fn update_user_profile<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        display_name: Option<String>,
    ) -> Result<(), DbErr> {
        let updated_user = admin_user::ActiveModel {
            id: Set(user_id),
            display_name: Set(display_name),
            ..Default::default()
        };
        updated_user.update(conn).await.map(|_| ())
    }
}
