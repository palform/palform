use anyhow::anyhow;
use ndarray::{Array1, Array2, ArrayView1};
use palform_client_common::form_management::{
    question_types::APIQuestion,
    submission::{InProgressSubmission, QuestionSubmissionData},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::features::Featureable;

#[wasm_bindgen]
pub struct FormAnalysisManager {
    pub(crate) feature_mat: Array2<f64>,
    pub(crate) feature_labels: Vec<String>,
    pub(crate) question_indices_for_features: Vec<usize>,
    pub(crate) question_labels: Vec<String>,
}

#[wasm_bindgen]
impl FormAnalysisManager {
    #[wasm_bindgen(constructor)]
    pub fn new(
        questions: wasm_bindgen::JsValue,
        submissions: wasm_bindgen::JsValue,
    ) -> Result<FormAnalysisManager, JsValue> {
        let questions: Vec<APIQuestion> = serde_wasm_bindgen::from_value(questions)?;
        let submissions: Vec<InProgressSubmission> = serde_wasm_bindgen::from_value(submissions)?;

        let interp_y = vec![-10_f64, 10_f64];
        Self::build_correlation_matrix(&questions, &submissions, &interp_y)
            .map_err(|e| JsValue::from(&e.to_string()))
    }

    fn build_correlation_matrix(
        questions: &[APIQuestion],
        submissions: &[InProgressSubmission],
        interp_y: &[f64],
    ) -> Result<Self, anyhow::Error> {
        let mut question_labels = Vec::<String>::new();
        let mut feature_labels = Vec::<String>::new();
        let mut question_indices = Vec::<usize>::new();
        let mut corr_mat = Array2::<f64>::zeros((0, submissions.len()));

        for (question_index, question) in questions.iter().enumerate() {
            question_labels.push(question.id.to_string());
            let mut labels = QuestionSubmissionData::feature_labels(&question.configuration);
            let feature_count = labels.len();
            feature_labels.append(&mut labels);
            let mut indices = vec![question_index; feature_count];
            question_indices.append(&mut indices);

            for feature_index in 0..feature_count {
                let mut mat_row = Array1::<f64>::zeros(submissions.len());

                for (submission_index, submission) in submissions.iter().enumerate() {
                    let matching_submission_question = submission
                        .questions
                        .iter()
                        .find(|e| e.question_id == question.id);
                    if let Some(matching_submission_question) = matching_submission_question {
                        let feature_val = matching_submission_question.data.feature_value(
                            interp_y,
                            &question.configuration,
                            feature_index,
                        )?;

                        if let Some(feature_val) = feature_val {
                            mat_row[submission_index] = feature_val;
                        }
                    }
                }

                corr_mat.push_row(mat_row.view()).map_err(|e| {
                    anyhow!("Push row of submissions for a feature to matrix: {}", e)
                })?;
            }
        }

        Ok(Self {
            feature_labels,
            question_indices_for_features: question_indices,
            feature_mat: corr_mat,
            question_labels,
        })
    }

    pub(crate) fn get_question_label_at_feature_index(
        &self,
        feature_index: usize,
    ) -> Result<&String, anyhow::Error> {
        let question_index = self
            .question_indices_for_features
            .get(feature_index)
            .ok_or(anyhow!("Feature index {} not found", feature_index))?;

        Ok(self
            .question_labels
            .get(question_index.clone())
            .ok_or(anyhow!("Question index {} not found", question_index))?)
    }

    pub(crate) fn get_feature_row_for_question_with_label(
        &self,
        question_id: String,
        feature_label: String,
    ) -> Result<ArrayView1<f64>, anyhow::Error> {
        let question_index = self
            .question_labels
            .iter()
            .position(|q| q == &question_id)
            .ok_or(anyhow!("question with id {} not found", question_id))?;

        let mat_row_index = self
            .question_indices_for_features
            .iter()
            .enumerate()
            .filter(|(_, &this_question_index)| this_question_index == question_index)
            .map(|(feature_index, _)| feature_index)
            .find(|feature_index| {
                self.feature_labels
                    .get(feature_index.clone())
                    .is_some_and(|v| v == &feature_label)
            })
            .ok_or(anyhow!(
                "Matrix row index not found for feature {}",
                feature_label
            ))?;

        let row = self.feature_mat.row(mat_row_index);
        Ok(row)
    }
}
