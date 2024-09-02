use std::marker::PhantomData;

use chrono::{DateTime, Utc};
use palform_client_common::{
    keys::{
        encryption_key::resolve_encryption_key,
        parse::{public_key_from_pem, secret_key_from_pem},
        validate::validate_cert,
    },
    policy::recipient_cert_policy,
};
use sequoia_openpgp::{
    cert::amalgamation::key::ValidKeyAmalgamation,
    packet::key::{KeyParts, PublicParts, UnspecifiedRole},
    parse::Parse,
    policy::StandardPolicy,
    serialize::SerializeInto,
    Cert, Fingerprint, KeyID,
};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum KeyConversionError {
    #[error("PGP: {0}")]
    PGP(String),
    #[error("Never-expiring encryption keys are not supported")]
    NeverExpiringKey,
}

pub struct CryptoKeyRepr<P: KeyParts> {
    cert: Cert,
    parts: PhantomData<P>,
}

impl<P: KeyParts> CryptoKeyRepr<P> {
    pub fn from_pem_string(key: &str) -> Result<Self, KeyConversionError> {
        let cert = match P::significant_secrets() {
            true => secret_key_from_pem(key.to_string()),
            false => public_key_from_pem(key.to_string()),
        }
        .map_err(|e| KeyConversionError::PGP(e.to_string()))?;
        validate_cert(&cert).map_err(|e| KeyConversionError::PGP(e.to_string()))?;
        Ok(CryptoKeyRepr {
            cert,
            parts: PhantomData,
        })
    }

    pub fn to_pem_string(&self) -> Result<String, KeyConversionError> {
        let base_cert = self.cert.clone().retain_user_attributes(|_| false);
        let armored_vec = match P::significant_secrets() {
            true => base_cert.as_tsk().armored().to_vec(),
            false => base_cert.armored().to_vec(),
        }
        .map_err(|e| KeyConversionError::PGP(e.to_string()))?;

        String::from_utf8(armored_vec).map_err(|e| KeyConversionError::PGP(e.to_string()))
    }

    pub fn from_database_bytes(key: &[u8]) -> Result<Self, KeyConversionError> {
        let cert = Cert::from_bytes(key).map_err(|e| KeyConversionError::PGP(e.to_string()))?;
        Ok(CryptoKeyRepr {
            cert,
            parts: PhantomData,
        })
    }

    pub fn to_database_bytes(&self) -> Result<Vec<u8>, KeyConversionError> {
        match P::significant_secrets() {
            true => self.cert.as_tsk().to_vec(),
            false => self.cert.to_vec(),
        }
        .map_err(|e| KeyConversionError::PGP(e.to_string()))
    }

    pub fn fingerprint(&self) -> Fingerprint {
        self.cert.fingerprint()
    }

    pub fn enc_key<'a>(
        &'a self,
        policy: &'a StandardPolicy,
    ) -> Result<ValidKeyAmalgamation<'_, PublicParts, UnspecifiedRole, bool>, KeyConversionError>
    {
        resolve_encryption_key(&self.cert, policy)
            .map_err(|e| KeyConversionError::PGP(e.to_string()))
    }

    pub fn all_subkey_ids<'a>(
        &'a self,
        policy: &'a StandardPolicy,
    ) -> Result<Vec<KeyID>, KeyConversionError> {
        let valid_cert = self
            .cert
            .with_policy(policy, None)
            .map_err(|e| KeyConversionError::PGP(e.to_string()))?;

        let mut id_vec = Vec::<KeyID>::new();
        for key in valid_cert.keys() {
            id_vec.push(key.keyid())
        }

        Ok(id_vec)
    }

    pub fn expiry(&self) -> Result<DateTime<Utc>, KeyConversionError> {
        let p = recipient_cert_policy();
        let enc_key = self.enc_key(&p)?;
        let exp_time = enc_key
            .key_expiration_time()
            .ok_or(KeyConversionError::NeverExpiringKey)?;

        let dt_exp_time: DateTime<Utc> = exp_time.clone().into();
        Ok(dt_exp_time)
    }
}
