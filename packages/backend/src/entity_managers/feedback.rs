use palform_entities::feedback_item;
use palform_tsid::tsid::PalformDatabaseID;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ConnectionTrait, DbErr};

pub struct FeedbackManager;

impl FeedbackManager {
    pub async fn create_feedback_item<T: ConnectionTrait>(
        conn: &T,
        score: i32,
        comment: Option<String>,
    ) -> Result<(), DbErr> {
        let new_feedback_item = feedback_item::ActiveModel {
            id: Set(PalformDatabaseID::random()),
            score: Set(score),
            comment: Set(comment),
            ..Default::default()
        };

        new_feedback_item.insert(conn).await?;
        Ok(())
    }
}
