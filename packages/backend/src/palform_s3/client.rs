use std::marker::PhantomData;

use s3::{
    creds::{error::CredentialsError, Credentials},
    error::S3Error,
    Bucket, Region,
};
use sea_orm::DbErr;
use thiserror::Error;

use crate::config::Config;

use super::buckets::PalformS3Bucket;

#[derive(Debug, Error)]
pub enum PalformS3Error {
    #[error("Initialising credentials: {0}")]
    Credentials(#[from] CredentialsError),
    #[error("S3: {0}")]
    S3(#[from] S3Error),
    #[error("db: {0}")]
    DBError(#[from] DbErr),
    #[error("Asset not found")]
    AssetNotFound,
}

pub struct PalformS3Client<T: PalformS3Bucket> {
    bucket_type: PhantomData<T>,
    pub bucket: Box<Bucket>,
}

impl<T: PalformS3Bucket> PalformS3Client<T> {
    pub fn init(config: &Config) -> Result<Self, PalformS3Error> {
        let creds = Credentials::new(
            Some(&config.s3_access_key),
            Some(&config.s3_secret_key),
            None,
            None,
            None,
        )?;

        let region = Region::Custom {
            region: config.s3_region.clone(),
            endpoint: config.s3_endpoint_url.clone(),
        };

        let bucket = Bucket::new(T::name(config).as_str(), region, creds)?;
        Ok(Self {
            bucket_type: PhantomData,
            bucket,
        })
    }
}
