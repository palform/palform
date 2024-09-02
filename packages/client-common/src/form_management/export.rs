use std::collections::HashMap;

use anyhow::anyhow;
use palform_tsid::{resources::IDQuestion, tsid::PalformDatabaseID};
use serde::Deserialize;

use super::{
    question_group::APIQuestionGroup, question_types::APIQuestion, submission::InProgressSubmission,
};

#[derive(PartialEq, Clone, Deserialize)]
pub enum ExportSubmissionsFormat {
    CSV,
    JSON,
}

#[derive(Clone, Deserialize)]
pub struct ExportSubmissionsConfig {
    pub use_question_ids: bool,
    pub use_group_ids: bool,
    pub format: ExportSubmissionsFormat,
}

type ExportIntermediate = HashMap<String, HashMap<String, String>>;

pub fn export_submissions(
    groups: Vec<APIQuestionGroup>,
    submissions: Vec<InProgressSubmission>,
    questions: Vec<APIQuestion>,
    config: ExportSubmissionsConfig,
) -> Result<String, anyhow::Error> {
    match config.format {
        ExportSubmissionsFormat::JSON => {
            let mut intermediates = Vec::<ExportIntermediate>::new();
            for submission in submissions {
                let mut intermediate = ExportIntermediate::new();

                for (group_index, group) in groups.iter().enumerate() {
                    let group_questions: Vec<APIQuestion> = questions
                        .iter()
                        .filter(|e| e.group_id == group.id)
                        .cloned()
                        .collect();

                    let mut group_map = HashMap::<String, String>::new();
                    for question in group_questions {
                        let matching_question_submission = submission
                            .questions
                            .iter()
                            .find(|e| e.question_id == question.id);

                        if let Some(matching_question_submission) = matching_question_submission {
                            let key = question
                                .to_export_key(&groups, config.use_question_ids, true, false)
                                .ok_or(anyhow!("Group ID of question failed to resolve"))?;
                            group_map.insert(key, matching_question_submission.data.to_string());
                        }
                    }

                    intermediate.insert(
                        if config.use_group_ids {
                            group.id.to_string()
                        } else {
                            format!("Section {}", group_index + 1)
                        },
                        group_map,
                    );
                }
                intermediates.push(intermediate);
            }
            serde_json::to_string(&intermediates).map_err(|e| anyhow!("json serialize: {}", e))
        }
        ExportSubmissionsFormat::CSV => {
            let mut w = csv::Writer::from_writer(Vec::new());
            let mut question_key_map = HashMap::<String, PalformDatabaseID<IDQuestion>>::new();
            let mut header_row = Vec::<String>::new();
            for group in &groups {
                let group_questions: Vec<APIQuestion> = questions
                    .iter()
                    .filter(|e| e.group_id == group.id)
                    .cloned()
                    .collect();

                for question in group_questions {
                    let key = question
                        .to_export_key(
                            &groups,
                            config.use_question_ids,
                            false,
                            config.use_group_ids,
                        )
                        .ok_or(anyhow!("Group ID of question failed to resolve"))?;
                    question_key_map.insert(key.clone(), question.id);
                    header_row.push(key);
                }
            }

            w.write_record(&header_row)
                .map_err(|e| anyhow!("write header row: {}", e))?;

            for submission in submissions {
                let mut row = Vec::<String>::new();
                for question_key in header_row.clone() {
                    let matching_question_id = question_key_map.get(&question_key);
                    if let Some(matching_question_id) = matching_question_id {
                        let matching_question_submission = submission
                            .questions
                            .iter()
                            .find(|e| e.question_id == matching_question_id.clone());

                        if let Some(matching_question_submission) = matching_question_submission {
                            row.push(matching_question_submission.data.to_string());
                        } else {
                            row.push(String::default());
                        }
                    } else {
                        row.push(String::default());
                    }
                }

                w.write_record(row)
                    .map_err(|e| anyhow!("write submission row: {}", e))?;
            }

            String::from_utf8(
                w.into_inner()
                    .map_err(|e| anyhow!("csv serialize: {}", e))?,
            )
            .map_err(|e| anyhow!("parse serialized csv: {}", e))
        }
    }
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn export_submissions_js(
    groups: wasm_bindgen::JsValue,
    submissions: wasm_bindgen::JsValue,
    questions: wasm_bindgen::JsValue,
    config: wasm_bindgen::JsValue,
) -> Result<String, wasm_bindgen::JsValue> {
    let groups = serde_wasm_bindgen::from_value(groups)?;
    let submissions = serde_wasm_bindgen::from_value(submissions)?;
    let questions = serde_wasm_bindgen::from_value(questions)?;
    let config = serde_wasm_bindgen::from_value(config)?;
    export_submissions(groups, submissions, questions, config)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
