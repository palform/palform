use chrono::Utc;
use palform_entities::admin_public_key;
use palform_entities::admin_user;
use palform_entities::prelude::*;
use palform_entities::team_membership;
use palform_migration::all;
use palform_tsid::resources::IDAdminPublicKey;
use palform_tsid::resources::IDAdminUser;
use palform_tsid::resources::IDOrganisation;
use palform_tsid::resources::IDTeam;
use palform_tsid::tsid::PalformDatabaseID;
use rocket::futures::TryFutureExt;
use sea_orm::Condition;
use sea_orm::PaginatorTrait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, ConnectionTrait, DbErr, EntityTrait,
    JoinType, Order, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
};
use sequoia_openpgp::packet::key::PublicParts;
use sequoia_openpgp::packet::key::SecretParts;
use sequoia_openpgp::Fingerprint;
use thiserror::Error;

use crate::api_entities::key::UserKeyWithIdentity;
use crate::crypto::keys::CryptoKeyRepr;
use crate::crypto::keys::KeyConversionError;

#[derive(Debug, Error)]
pub enum RegisterKeyError {
    #[error("Converting key: {0}")]
    KeyConversion(#[from] KeyConversionError),
    #[error("Database: {0}")]
    DBError(#[from] DbErr),
}

pub struct UserKeyManager;
impl UserKeyManager {
    pub async fn get_key_with_id<T: ConnectionTrait>(
        conn: &T,
        key_id: PalformDatabaseID<IDAdminPublicKey>,
    ) -> Result<Option<admin_public_key::Model>, DbErr> {
        AdminPublicKey::find_by_id(key_id).one(conn).await
    }

    pub async fn verify_key_org_and_user<T: ConnectionTrait>(
        conn: &T,
        key_id: PalformDatabaseID<IDAdminPublicKey>,
        organisation_id: PalformDatabaseID<IDOrganisation>,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<bool, DbErr> {
        AdminPublicKey::find_by_id(key_id)
            .filter(
                Condition::all()
                    .add(admin_public_key::Column::OrganisationId.eq(organisation_id))
                    .add(admin_public_key::Column::UserId.eq(user_id)),
            )
            .count(conn)
            .await
            .map(|c| c == 1)
    }

    pub async fn list_keys_for_user<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        organisation_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<Vec<admin_public_key::Model>, DbErr> {
        AdminPublicKey::find()
            .filter(admin_public_key::Column::UserId.eq(user_id))
            .filter(admin_public_key::Column::OrganisationId.eq(organisation_id))
            .order_by(admin_public_key::Column::CreatedAt, Order::Desc)
            .all(conn)
            .await
    }

    pub async fn list_all_team_keys<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        team_id: PalformDatabaseID<IDTeam>,
    ) -> Result<Vec<admin_public_key::Model>, DbErr> {
        AdminPublicKey::find()
            .join(
                JoinType::InnerJoin,
                admin_public_key::Relation::AdminUser.def(),
            )
            .join(
                JoinType::InnerJoin,
                admin_user::Relation::TeamMembership.def(),
            )
            .filter(all![
                team_membership::Column::TeamId.eq(team_id),
                admin_public_key::Column::OrganisationId.eq(org_id),
                admin_public_key::Column::ExpiresAt.gt(Utc::now().naive_utc())
            ])
            .all(conn)
            .await
    }

    pub async fn list_all_org_keys_with_identities<T: ConnectionTrait>(
        conn: &T,
        organisation_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<Vec<UserKeyWithIdentity>, DbErr> {
        AdminPublicKey::find()
            .join(
                JoinType::LeftJoin,
                admin_public_key::Relation::AdminUser.def(),
            )
            .column_as(admin_user::Column::DisplayName, "user_display_name")
            .column_as(admin_user::Column::Email, "user_email")
            .filter(admin_public_key::Column::OrganisationId.eq(organisation_id))
            .into_model::<UserKeyWithIdentity>()
            .all(conn)
            .await
    }

    pub async fn check_fingerprint_exists<T: ConnectionTrait>(
        conn: &T,
        fingerprint: Fingerprint,
    ) -> Result<bool, DbErr> {
        AdminPublicKey::find()
            .filter(admin_public_key::Column::CertFingerprint.eq(fingerprint.to_hex()))
            .count(conn)
            .await
            .map(|c| c == 1)
    }

    pub async fn register_key_for_user<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        organisation_id: PalformDatabaseID<IDOrganisation>,
        key_data: CryptoKeyRepr<PublicParts>,
    ) -> Result<admin_public_key::Model, RegisterKeyError> {
        let key_bytes = key_data.to_database_bytes()?;
        let key_expiration = key_data.expiry()?;

        let new_record = admin_public_key::ActiveModel {
            id: Set(PalformDatabaseID::<IDAdminPublicKey>::random()),
            user_id: Set(user_id),
            organisation_id: Set(organisation_id),
            public_key: Set(key_bytes),
            cert_fingerprint: Set(key_data.fingerprint().to_hex()),
            private_key_backup: Set(None),
            expires_at: Set(key_expiration.naive_utc()),
            created_at: NotSet,
        };
        new_record.insert(conn).map_err(|e| e.into()).await
    }

    pub async fn delete_key_with_id<T: ConnectionTrait>(
        conn: &T,
        key_id: PalformDatabaseID<IDAdminPublicKey>,
    ) -> Result<(), DbErr> {
        AdminPublicKey::delete_by_id(key_id)
            .exec(conn)
            .await
            .map(|_| ())
    }

    pub async fn register_user_key_backup<T: ConnectionTrait>(
        conn: &T,
        key_id: PalformDatabaseID<IDAdminPublicKey>,
        secret_key_data: CryptoKeyRepr<SecretParts>,
    ) -> Result<(), RegisterKeyError> {
        let key_bytes = secret_key_data.to_database_bytes()?;
        let updated_record = admin_public_key::ActiveModel {
            id: Set(key_id),
            private_key_backup: Set(Some(key_bytes)),
            ..Default::default()
        };
        updated_record
            .update(conn)
            .await
            .map(|_| ())
            .map_err(|e| e.into())
    }

    pub async fn delete_user_key_backup<T: ConnectionTrait>(
        conn: &T,
        key_id: PalformDatabaseID<IDAdminPublicKey>,
    ) -> Result<(), DbErr> {
        let updated_record = admin_public_key::ActiveModel {
            id: Set(key_id),
            private_key_backup: Set(None),
            ..Default::default()
        };
        updated_record.update(conn).await.map(|_| ())
    }

    pub async fn delete_all_in_org_for_user<T: ConnectionTrait>(
        conn: &T,
        organisation_id: PalformDatabaseID<IDOrganisation>,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<(), DbErr> {
        AdminPublicKey::delete_many()
            .filter(all![
                admin_public_key::Column::OrganisationId.eq(organisation_id),
                admin_public_key::Column::UserId.eq(user_id)
            ])
            .exec(conn)
            .await?;
        Ok(())
    }
}
