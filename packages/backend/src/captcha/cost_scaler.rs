use std::{cmp, net::IpAddr};

use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::{
    config::Config,
    memory_db::{
        memory_db::{MemoryDB, MemoryDBError},
        types::CaptchaCostScalerMemoryDBType,
    },
};

pub struct CaptchaCostScaler;

#[derive(Debug, Error)]
pub enum CaptchaCostScalerError {
    #[error("Memory DB: {0}")]
    MemoryDB(#[from] MemoryDBError),
    #[error("{0}")]
    Other(String),
}

impl CaptchaCostScaler {
    fn create_key(
        ip_addr: &IpAddr,
        config: &Config,
    ) -> Result<(blake3::Hash, DateTime<Utc>), CaptchaCostScalerError> {
        let time_bucket = u32::try_from(Utc::now().timestamp())
            .map_err(|e| CaptchaCostScalerError::Other(e.to_string()))?
            .div_ceil(60) // conver to minutes
            .div_ceil(config.captcha_cost_scaler_bucket_mins);

        let bucket_end = DateTime::<Utc>::from_timestamp_secs(i64::from(
            (time_bucket + 2) * config.captcha_cost_scaler_bucket_mins * 60,
        ))
        .ok_or(CaptchaCostScalerError::Other(
            "Create timestamp".to_string(),
        ))?;

        println!("end {}", bucket_end.to_rfc3339());

        let key = format!("{}-{}", ip_addr.to_string(), time_bucket);
        let hash = blake3::hash(key.as_bytes());
        Ok((hash, bucket_end))
    }

    fn calculate_cost(count: usize, config: &Config) -> Result<u32, CaptchaCostScalerError> {
        let count =
            u32::try_from(count).map_err(|e| CaptchaCostScalerError::Other(e.to_string()))?;
        let count = f64::from(count);
        let cost = f64::from(config.captcha_cost_scaler_min_cost)
            * (config.captcha_cost_scaler_growth.powf(count));
        let cost = cmp::min(cost as u32, config.captcha_cost_scaler_max_cost);
        Ok(cost)
    }

    pub async fn log(
        ip_addr: &IpAddr,
        memory_db: &MemoryDB,
        config: &Config,
    ) -> Result<u32, CaptchaCostScalerError> {
        let (key, expiry_time) = Self::create_key(ip_addr, config)?;
        let expiry_duration = expiry_time - Utc::now();
        let val = memory_db
            .increment::<CaptchaCostScalerMemoryDBType>(&key, expiry_duration)
            .await?;
        Self::calculate_cost(val, config)
    }
}
