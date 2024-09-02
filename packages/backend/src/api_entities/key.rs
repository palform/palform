use chrono::NaiveDateTime;
use palform_entities::admin_public_key;
use palform_tsid::{
    resources::{IDAdminPublicKey, IDAdminUser},
    tsid::PalformDatabaseID,
};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::FromQueryResult;
use sequoia_openpgp::packet::key::PublicParts;
use serde::Serialize;

use crate::crypto::keys::{CryptoKeyRepr, KeyConversionError};

#[derive(Serialize, JsonSchema)]
pub struct APIUserKey {
    pub id: String,
    pub user_id: String,
    pub key_pem: String,
    pub has_backup: bool,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

impl TryFrom<admin_public_key::Model> for APIUserKey {
    type Error = KeyConversionError;
    fn try_from(value: admin_public_key::Model) -> Result<Self, Self::Error> {
        let key = CryptoKeyRepr::<PublicParts>::from_database_bytes(&value.public_key)?;
        let key_pem = key.to_pem_string()?;
        Ok(APIUserKey {
            id: value.id.to_string(),
            user_id: value.user_id.to_string(),
            key_pem,
            has_backup: value.private_key_backup.is_some(),
            created_at: value.created_at,
            expires_at: value.expires_at,
        })
    }
}

#[derive(FromQueryResult)]
pub struct UserKeyWithIdentity {
    pub id: PalformDatabaseID<IDAdminPublicKey>,
    pub public_key: Vec<u8>,
    pub user_id: PalformDatabaseID<IDAdminUser>,
    pub user_display_name: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Serialize, JsonSchema, Clone)]
pub struct APIUserKeyWithIdentity {
    pub id: PalformDatabaseID<IDAdminPublicKey>,
    pub key_fingerprint: String,
    pub user_id: PalformDatabaseID<IDAdminUser>,
    pub user_display_name: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}
