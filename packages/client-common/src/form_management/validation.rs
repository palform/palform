use super::{
    question_types::{APIQuestion, APIQuestionConfiguration, APIQuestionTextValidator},
    submission::{QuestionSubmission, QuestionSubmissionData},
};
use anyhow::anyhow;
use validator::ValidateEmail;

#[derive(Clone)]
#[cfg_attr(
    feature = "frontend-js",
    wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
)]
pub struct ValidationError {
    pub question_id: String,
    pub error: String,
}

pub fn validate_questions(
    questions: Vec<APIQuestion>,
    submissions: Vec<QuestionSubmission>,
) -> Result<Vec<ValidationError>, anyhow::Error> {
    let mut errors = Vec::<ValidationError>::new();
    for question in questions {
        if !question.configuration.requires_submission() {
            continue;
        }

        let submission = submissions
            .iter()
            .find(|e| e.question_id == question.id)
            .ok_or(anyhow!(
                "All questions must have a corresponding submission, even if not required"
            ))?;

        let is_empty = submission.data.is_empty();
        if question.required && is_empty {
            errors.push(ValidationError {
                question_id: question.id.to_string(),
                error: "This question is required".to_string(),
            });
        }

        // The remaining validation rules aren't important if the submission is empty
        if is_empty {
            continue;
        }

        if let APIQuestionConfiguration::Text {
            is_long: _,
            validator,
        } = question.configuration
        {
            if let QuestionSubmissionData::Text { value } = submission.data.clone() {
                if let Some(validator) = validator {
                    match validator {
                        APIQuestionTextValidator::Email => {
                            if !value.validate_email() {
                                errors.push(ValidationError {
                                    question_id: question.id.to_string(),
                                    error: "Value must be a valid email address".to_string(),
                                })
                            }
                        }
                        APIQuestionTextValidator::Integer => {
                            if let Err(_) = value.parse::<i32>() {
                                errors.push(ValidationError {
                                    question_id: question.id.to_string(),
                                    error: "Value must be a number".to_string(),
                                })
                            }
                        }
                        APIQuestionTextValidator::Float => {
                            if let Err(_) = value.parse::<f32>() {
                                errors.push(ValidationError {
                                    question_id: question.id.to_string(),
                                    error: "Value must be a decimal number".to_string(),
                                })
                            }
                        }
                    };
                }
            }
        }
    }

    Ok(errors)
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn validate_questions_js(
    questions: wasm_bindgen::JsValue,
    submissions: wasm_bindgen::JsValue,
) -> Result<Vec<ValidationError>, wasm_bindgen::JsValue> {
    let questions = serde_wasm_bindgen::from_value::<Vec<APIQuestion>>(questions)?;
    let submissions = serde_wasm_bindgen::from_value::<Vec<QuestionSubmission>>(submissions)?;

    validate_questions(questions, submissions)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
