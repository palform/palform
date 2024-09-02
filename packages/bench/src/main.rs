use std::{fs, ops::AddAssign};

use chrono::{Duration, Utc};
use figment::{providers::Env, Figment};
use palform_client_common::{
    encrypt::message::encrypt_submission,
    form_management::submission::{
        InProgressSubmission, QuestionSubmission, QuestionSubmissionData,
    },
};
use palform_entities::{prelude::*, submission};
use palform_tsid::{
    resources::{IDForm, IDQuestion, IDQuestionGroup, IDSubmission},
    tsid::PalformDatabaseID,
};
use rand::seq::SliceRandom;
use sea_orm::{
    AccessMode, ActiveModelTrait, ColumnTrait, Database, EntityTrait, IsolationLevel, QueryFilter,
    Set, TransactionTrait,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub submission_count: usize,
    pub form_id: PalformDatabaseID<IDForm>,
    pub choice_question_ids: Vec<PalformDatabaseID<IDQuestion>>,
    pub choice_question_group_id: PalformDatabaseID<IDQuestionGroup>,
    pub choice_question_choices: Vec<String>,
    pub public_key_file: String,
    pub delete_rest: bool,
}

#[tokio::main]
async fn main() {
    let config: Config = Figment::new()
        .merge(Env::prefixed("PALBENCH_").split("__").global())
        .extract()
        .unwrap();

    let db = Database::connect(&config.database_url)
        .await
        .expect("Connecting to DB");
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await
        .expect("start db txn");

    if config.delete_rest {
        let delete_result = Submission::delete_many()
            .filter(submission::Column::FormId.eq(config.form_id))
            .exec(&txn)
            .await
            .expect("delete all submissions");
        println!(
            "deleted {} existing submissions for form {}",
            delete_result.rows_affected, config.form_id
        );
    }

    let public_key = fs::read_to_string(config.public_key_file).expect("read public key file");
    println!("using public key: {}", public_key);

    println!(
        "inserting {} submissions for form {}, each with {} question(s)",
        config.submission_count,
        config.form_id,
        config.choice_question_ids.len(),
    );
    let mut current_time = Utc::now();
    for i in 0..config.submission_count {
        let chosen_option = config
            .choice_question_choices
            .choose(&mut rand::thread_rng())
            .expect("random choice");

        let mut question_submissions = Vec::<QuestionSubmission>::new();
        for choice_question_id in &config.choice_question_ids {
            let ques_sub = QuestionSubmission {
                question_id: choice_question_id.clone(),
                data: QuestionSubmissionData::Choice {
                    option: vec![chosen_option.clone()],
                },
            };
            question_submissions.push(ques_sub);
        }

        let submission_data = InProgressSubmission {
            form_id: config.form_id,
            groups_completed: vec![config.choice_question_group_id],
            questions: question_submissions,
        };

        let encrypted_submission_data =
            encrypt_submission(submission_data, vec![public_key.clone()])
                .expect("encrypt submission");

        let new_submission = submission::ActiveModel {
            id: Set(PalformDatabaseID::<IDSubmission>::random()),
            encrypted_data: Set(encrypted_submission_data),
            form_id: Set(config.form_id),
            for_token: Set(None),
            created_at: Set(current_time.naive_utc()),
            ..Default::default()
        };
        new_submission
            .insert(&txn)
            .await
            .expect("insert submission");

        current_time.add_assign(Duration::milliseconds(1));
        println!("done {}/{}", i + 1 , config.submission_count);
    }

    txn.commit().await.expect("commit db txn");
    println!("done heheh");
}
