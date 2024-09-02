use crate::config::Config;

pub trait PalformS3Bucket {
    fn name(config: &Config) -> String;
}

pub struct S3BucketTeamAssets;
impl PalformS3Bucket for S3BucketTeamAssets {
    fn name(config: &Config) -> String {
        config.s3_team_assets_bucket.clone()
    }
}

pub struct S3BucketSubmissionAssets;
impl PalformS3Bucket for S3BucketSubmissionAssets {
    fn name(config: &Config) -> String {
        config.s3_submission_assets_bucket.clone()
    }
}
