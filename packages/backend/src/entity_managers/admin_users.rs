use std::str::FromStr;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use palform_entities::{admin_user, prelude::*, social_auth_connection};
use palform_tsid::{
    resources::{IDAdminUser, IDOrganisation},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, JoinType,
    QueryFilter, QuerySelect, RelationTrait, Set,
};
use thiserror::Error;

use crate::{
    api_entities::admin_users::{APIAdminUser, APIServiceAccountAdminUser},
    auth::social::SocialAuthService,
    crypto::util::create_hex_secret,
};

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
    #[error("The provided service account token was invalid or incomplete.")]
    ServiceAccountTokenParse,
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
        org_id: Option<PalformDatabaseID<IDOrganisation>>,
        service: Option<SocialAuthService>,
        sub: String,
    ) -> Result<Option<admin_user::Model>, DbErr> {
        if let Some(org_id) = org_id {
            AdminUser::find()
                .filter(
                    Condition::all()
                        .add(admin_user::Column::OrgAuthSub.eq(Some(sub)))
                        .add(admin_user::Column::OrgAuthOrganisationId.eq(Some(org_id))),
                )
                .one(conn)
                .await
        } else if let Some(service) = service {
            AdminUser::find()
                .join(
                    JoinType::InnerJoin,
                    admin_user::Relation::SocialAuthConnection.def(),
                )
                .filter(
                    Condition::all()
                        .add(social_auth_connection::Column::Sub.eq(sub))
                        .add(social_auth_connection::Column::Service.eq(service.to_string())),
                )
                .one(conn)
                .await
        } else {
            return Err(DbErr::Custom(
                "Either org_id or service must be specified".to_string(),
            ));
        }
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

    fn get_argon2<'k>() -> Argon2<'k> {
        Argon2::default()
    }

    fn hash_password(password: String) -> Result<String, argon2::password_hash::Error> {
        let password: PasswordHash = Self::get_argon2().hash_password(password.as_bytes())?;
        Ok(password.to_string())
    }

    fn verify_password_hash(raw: &str, hashed: &str) -> bool {
        let parsed_hash = PasswordHash::new(&hashed);

        if parsed_hash.is_err() {
            return false;
        }

        let parsed_hash = parsed_hash.unwrap();
        Self::get_argon2()
            .verify_password(raw.as_bytes(), &parsed_hash)
            .is_ok()
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

        Ok(Self::verify_password_hash(&password, &hash))
    }

    pub async fn set_user_password<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        password: String,
    ) -> Result<(), AdminUserManagementError> {
        let password = Self::hash_password(password).map_err(AdminUserManagementError::Password)?;
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
            Self::hash_password(password).map_err(AdminUserManagementError::Password)?;

        let id = PalformDatabaseID::<IDAdminUser>::random();
        let new_user = admin_user::ActiveModel {
            id: Set(id),
            email: Set(email),
            manual_auth_email_verified: Set(Some(false)),
            manual_auth_password_hash: Set(Some(password_hash)),
            ..Default::default()
        };

        new_user.insert(conn).await?;
        Ok(id)
    }

    pub async fn create_user_without_auth<T: ConnectionTrait>(
        conn: &T,
        email: String,
    ) -> Result<PalformDatabaseID<IDAdminUser>, AdminUserManagementError> {
        let id = PalformDatabaseID::<IDAdminUser>::random();
        let new_user = admin_user::ActiveModel {
            id: Set(id),
            email: Set(email),
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
            id: Set(id),
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

    pub async fn get_user_by_service_account_token<T: ConnectionTrait>(
        conn: &T,
        frontend_facing_token: String,
    ) -> Result<Option<admin_user::Model>, AdminUserManagementError> {
        let (user_id, unhashed_secret) = Self::parse_service_account_token(&frontend_facing_token)?;
        let user = AdminUser::find_by_id(user_id)
            .filter(admin_user::Column::ServiceAccountAuthToken.is_not_null())
            .one(conn)
            .await?;

        if user.is_none() {
            return Ok(None);
        }

        let user = user.unwrap();

        let hashed_secret = user
            .service_account_auth_token
            .as_ref()
            .expect("DB-validated non-null token to be Some");

        if !Self::verify_password_hash(&unhashed_secret, &hashed_secret) {
            return Ok(None);
        }

        Ok(Some(user))
    }

    pub async fn get_service_account_user_by_id<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<Option<APIServiceAccountAdminUser>, DbErr> {
        AdminUser::find_by_id(user_id)
            .filter(admin_user::Column::ServiceAccountAuthToken.is_not_null())
            .into_model()
            .one(conn)
            .await
    }

    fn format_service_account_token(
        user_id: PalformDatabaseID<IDAdminUser>,
        unhashed_secret: &str,
    ) -> String {
        format!("{}:{}", user_id, unhashed_secret)
    }

    fn parse_service_account_token(
        frontend_facing_token: &str,
    ) -> Result<(PalformDatabaseID<IDAdminUser>, String), AdminUserManagementError> {
        let mut split_token = frontend_facing_token.split(":");
        let user_id = split_token
            .next()
            .ok_or(AdminUserManagementError::ServiceAccountTokenParse)?;
        let user_id = PalformDatabaseID::<IDAdminUser>::from_str(user_id)
            .map_err(|_| AdminUserManagementError::ServiceAccountTokenParse)?;

        let unhashed_secret = split_token
            .next()
            .ok_or(AdminUserManagementError::ServiceAccountTokenParse)?;

        Ok((user_id, unhashed_secret.to_owned()))
    }

    pub async fn create_service_account_user<T: ConnectionTrait>(
        conn: &T,
    ) -> Result<APIServiceAccountAdminUser, AdminUserManagementError> {
        let id = PalformDatabaseID::<IDAdminUser>::random();
        let email = format!("{}@serviceaccount.palform.app", id);
        let token = create_hex_secret();
        let hashed_token =
            Self::hash_password(token.clone()).map_err(AdminUserManagementError::Password)?;

        let new_user = admin_user::ActiveModel {
            id: Set(id),
            email: Set(email.clone()),
            service_account_auth_token: Set(Some(hashed_token.clone())),
            ..Default::default()
        };
        let new_user = new_user.insert(conn).await?;

        let frontend_facing_token = Self::format_service_account_token(id, &token);

        Ok(APIServiceAccountAdminUser {
            id,
            email,
            service_account_auth_token: Some(frontend_facing_token),
            created_at: new_user.created_at.to_utc(),
        })
    }

    pub async fn delete_user<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<(), DbErr> {
        AdminUser::delete_by_id(user_id).exec(conn).await?;
        Ok(())
    }
}
