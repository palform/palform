use std::{collections::BTreeMap, net::IpAddr, str::FromStr};

use altcha::{
    create_challenge, verify_solution, Challenge, CreateChallengeOptions, HmacAlgorithm,
    VerifySolutionOptions,
};
use chrono::{Duration, Utc};
use palform_tsid::{resources::IDCaptchaChallenge, tsid::PalformDatabaseID};
use thiserror::Error;

use crate::{
    captcha::cost_scaler::{CaptchaCostScaler, CaptchaCostScalerError},
    config::Config,
    memory_db::{
        memory_db::{MemoryDB, MemoryDBError},
        types::CaptchaChallengeMemoryDBType,
    },
};

pub struct CaptchaManager;

#[derive(Debug, Error)]
pub enum CaptchaManagerError {
    #[error("Memory DB: {0}")]
    MemoryDB(#[from] MemoryDBError),
    #[error("Challenge: {0}")]
    Challenge(#[from] altcha::Error),
    #[error("Cost scaler: {0}")]
    CostScaler(#[from] CaptchaCostScalerError),
    #[error("Captcha invalid")]
    Invalid,
}

impl CaptchaManager {
    pub async fn create_challenge(
        ip_addr: &IpAddr,
        memory_db: &MemoryDB,
        config: &Config,
    ) -> Result<Challenge, CaptchaManagerError> {
        let expiry_duration = Duration::minutes(5);
        let expiry = Utc::now() + expiry_duration;

        let id = PalformDatabaseID::<IDCaptchaChallenge>::random();
        let mut data_map = BTreeMap::<String, serde_json::Value>::new();
        data_map.insert(
            "id".to_owned(),
            serde_json::to_value(id.to_string()).expect("String to value"),
        );

        let cost = CaptchaCostScaler::log(ip_addr, memory_db, config).await?;

        let challenge = create_challenge(CreateChallengeOptions {
            expires_at: Some(expiry.timestamp().unsigned_abs()),
            data: Some(data_map),
            cost,
            hmac_signature_secret: Some(config.captcha_secret_key.clone()),
            // HMAC signing is not needed since we store the authoritative challenge server-side
            ..Default::default()
        })?;

        memory_db
            .write::<CaptchaChallengeMemoryDBType>(&id, challenge.clone(), expiry_duration)
            .await?;

        Ok(challenge)
    }

    fn check_challenge_equality(c1: &Challenge, c2: &Challenge) -> bool {
        c1.signature
            .as_ref()
            .is_some_and(|s1| c2.signature.as_ref().is_some_and(|s2| s1 == s2))
    }

    pub async fn validate_challenge(
        payload: altcha::Payload,
        memory_db: &MemoryDB,
        config: &Config,
    ) -> Result<(), CaptchaManagerError> {
        let payload_data = payload
            .challenge
            .parameters
            .data
            .as_ref()
            .ok_or(CaptchaManagerError::Invalid)?;
        let challenge_id = payload_data
            .get("id")
            .ok_or(CaptchaManagerError::Invalid)?
            .as_str()
            .ok_or(CaptchaManagerError::Invalid)?;
        let challenge_id = PalformDatabaseID::<IDCaptchaChallenge>::from_str(challenge_id)
            .map_err(|_| CaptchaManagerError::Invalid)?;

        let stored_challenge = memory_db
            .read::<CaptchaChallengeMemoryDBType>(&challenge_id)
            .await
            .map_err(|e| match e {
                MemoryDBError::KeyMissing => CaptchaManagerError::Invalid,
                _ => CaptchaManagerError::from(e),
            })?;

        if !Self::check_challenge_equality(&payload.challenge, &stored_challenge) {
            return Err(CaptchaManagerError::Invalid);
        }

        let result = verify_solution(VerifySolutionOptions {
            challenge: &payload.challenge,
            solution: &payload.solution,
            hmac_algorithm: HmacAlgorithm::Sha256,
            hmac_key_signature_secret: None,
            hmac_signature_secret: config.captcha_secret_key.clone(),
        })?;
        if result.verified {
            // Prevent replays
            // This happens outside of any transaction; even if a later DB query fails,
            // the captcha still cannot be replayed. All new requests/retries need a new
            // challenge/solution.
            memory_db
                .delete::<CaptchaChallengeMemoryDBType>(&challenge_id)
                .await?;
            Ok(())
        } else {
            Err(CaptchaManagerError::Invalid)
        }
    }
}
