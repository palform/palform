use palform_entities::{prelude::*, team_asset};
use palform_tsid::{
    resources::{IDTeam, IDTeamAsset},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};

use crate::{
    api_entities::team_asset::APITeamAsset,
    palform_s3::{
        buckets::S3BucketTeamAssets,
        client::{PalformS3Client, PalformS3Error},
    },
};

pub struct TeamAssetsManager {
    team_id: PalformDatabaseID<IDTeam>,
}

impl TeamAssetsManager {
    pub fn new(team_id: PalformDatabaseID<IDTeam>) -> Self {
        Self { team_id }
    }

    fn asset_path(&self, asset_id: &PalformDatabaseID<IDTeamAsset>) -> String {
        format!("/{}/{}", self.team_id, asset_id)
    }

    pub async fn create<T: ConnectionTrait>(
        &self,
        conn: &T,
        s3: &PalformS3Client<S3BucketTeamAssets>,
        data: &[u8],
        content_type: &str,
    ) -> Result<PalformDatabaseID<IDTeamAsset>, PalformS3Error> {
        let asset_id = PalformDatabaseID::<IDTeamAsset>::random();
        let new_asset = team_asset::ActiveModel {
            id: Set(asset_id),
            team_id: Set(self.team_id),
            ..Default::default()
        };

        s3.bucket
            .put_object_with_content_type(self.asset_path(&asset_id), data, content_type)
            .await?;
        new_asset.insert(conn).await?;
        Ok(asset_id)
    }

    async fn create_api_team_asset(
        &self,
        s3: &PalformS3Client<S3BucketTeamAssets>,
        model: team_asset::Model,
    ) -> Result<APITeamAsset, PalformS3Error> {
        let url = s3
            .bucket
            .presign_get(self.asset_path(&model.id), 60 * 60, None)
            .await?;

        Ok(APITeamAsset::from(model, url))
    }

    pub async fn list<T: ConnectionTrait>(
        &self,
        conn: &T,
        s3: &PalformS3Client<S3BucketTeamAssets>,
    ) -> Result<Vec<APITeamAsset>, PalformS3Error> {
        let assets = TeamAsset::find()
            .filter(team_asset::Column::TeamId.eq(self.team_id))
            .all(conn)
            .await?;

        let mut api_assets = Vec::<APITeamAsset>::new();
        for asset in assets {
            api_assets.push(self.create_api_team_asset(s3, asset).await?);
        }

        Ok(api_assets)
    }

    pub async fn get<T: ConnectionTrait>(
        &self,
        conn: &T,
        s3: &PalformS3Client<S3BucketTeamAssets>,
        asset_id: PalformDatabaseID<IDTeamAsset>,
    ) -> Result<APITeamAsset, PalformS3Error> {
        let asset = TeamAsset::find_by_id(asset_id)
            .one(conn)
            .await?
            .ok_or(PalformS3Error::AssetNotFound)?;

        self.create_api_team_asset(s3, asset).await
    }

    pub async fn verify_asset_team<T: ConnectionTrait>(
        &self,
        conn: &T,
        asset_id: PalformDatabaseID<IDTeamAsset>,
    ) -> Result<bool, DbErr> {
        TeamAsset::find_by_id(asset_id)
            .filter(team_asset::Column::TeamId.eq(self.team_id))
            .count(conn)
            .await
            .map(|c| c == 1)
    }
}
