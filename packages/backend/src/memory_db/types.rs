use std::fmt::Display;

use palform_tsid::{resources::IDCaptchaChallenge, tsid::PalformDatabaseID};
use serde::{de::DeserializeOwned, Serialize};

pub trait MemoryDBType {
    type Key: Display;
    type Value: Serialize + DeserializeOwned;
    fn key_prefix() -> String;
}

pub struct CaptchaChallengeMemoryDBType;
impl MemoryDBType for CaptchaChallengeMemoryDBType {
    type Key = PalformDatabaseID<IDCaptchaChallenge>;
    type Value = altcha::Challenge;
    fn key_prefix() -> String {
        "captcha_challenge".to_string()
    }
}

pub struct CaptchaCostScalerMemoryDBType;
impl MemoryDBType for CaptchaCostScalerMemoryDBType {
    type Key = blake3::Hash;
    type Value = isize;
    fn key_prefix() -> String {
        "captcha_cost".to_string()
    }
}
