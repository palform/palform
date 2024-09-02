use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Local, SecondsFormat};
use palform_tsid::{resources::{IDForm, IDQuestion, IDQuestionGroup}, tsid::PalformDatabaseID};
use serde::{Deserialize, Serialize};

use crate::address::{APIGenericAddress, APIGenericLocation};

use super::question_types::{APIQuestion, APIQuestionConfiguration};

#[cfg_attr(feature = "frontend-js", derive(ts_rs::TS))]
#[cfg_attr(feature = "frontend-js", ts(export))]
#[derive(Deserialize, Serialize, Clone)]
pub struct InProgressSubmission {
    pub form_id: PalformDatabaseID<IDForm>,
    pub groups_completed: Vec<PalformDatabaseID<IDQuestionGroup>>,
    pub questions: Vec<QuestionSubmission>,
}

#[cfg_attr(feature = "frontend-js", derive(ts_rs::TS))]
#[cfg_attr(feature = "frontend-js", ts(export))]
#[derive(Deserialize, Serialize, Clone)]
pub struct QuestionSubmission {
    pub question_id: PalformDatabaseID<IDQuestion>,
    pub data: QuestionSubmissionData,
}

#[cfg_attr(feature = "frontend-js", derive(ts_rs::TS))]
#[cfg_attr(feature = "frontend-js", ts(export))]
#[derive(Deserialize, Serialize, Clone)]
pub enum QuestionSubmissionData {
    Text {
        value: String,
    },
    Choice {
        option: Vec<String>,
    },
    Scale {
        value: Option<i32>,
    },
    Address {
        address: APIGenericAddress,
        point: APIGenericLocation,
    },
    PhoneNumber {
        calling_code: String,
        number: String,
    },
    FileUpload {
        file_id: String,
        content_type: String,
    },
    Signature {
        freeform: Vec<Vec<Vec<f32>>>,
        initial: String,
        full_name: String,
    },
    ChoiceMatrix {
        #[cfg_attr(feature = "frontend-js", ts(type = "Map<string, string[]>"))]
        options: HashMap<String, Vec<String>>,
    },
    DateTime {
        #[cfg_attr(feature = "frontend-js", ts(type = "string"))]
        value: DateTime<Local>,
    },
    Hidden {
        value: String,
    },
}

impl QuestionSubmissionData {
    pub fn is_empty(&self) -> bool {
        match self {
            QuestionSubmissionData::Text { value } => value.trim().is_empty(),
            QuestionSubmissionData::Choice { option } => option.is_empty(),
            QuestionSubmissionData::ChoiceMatrix { options } => {
                options.iter().all(|(_, v)| v.is_empty())
            }
            QuestionSubmissionData::Scale { value } => value.is_none(),
            QuestionSubmissionData::Address { address, point } => {
                address.is_empty() && point.is_empty()
            }
            QuestionSubmissionData::PhoneNumber {
                calling_code,
                number,
            } => calling_code.is_empty() || number.is_empty(),
            QuestionSubmissionData::FileUpload {
                file_id,
                content_type,
            } => file_id.is_empty() || content_type.is_empty(),
            QuestionSubmissionData::Signature {
                freeform,
                initial,
                full_name,
            } => freeform.is_empty() && initial.is_empty() && full_name.is_empty(),
            QuestionSubmissionData::DateTime { value } => value.timestamp() == 0,
            QuestionSubmissionData::Hidden { value } => value.is_empty(),
        }
    }
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn question_submission_is_empty_js(
    question_submission_data: wasm_bindgen::JsValue,
) -> Result<bool, wasm_bindgen::JsValue> {
    let question_submission_data: QuestionSubmissionData =
        serde_wasm_bindgen::from_value(question_submission_data)?;
    Ok(question_submission_data.is_empty())
}

impl Display for QuestionSubmissionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestionSubmissionData::Text { value } => write!(f, "{}", value),
            QuestionSubmissionData::Choice { option } => {
                write!(f, "{}", option.join(","))
            }
            QuestionSubmissionData::ChoiceMatrix { options } => {
                for (index, (column, items)) in options.iter().enumerate() {
                    write!(f, "{}:{}", column, items.join(","))?;
                    if index != options.len() - 1 {
                        write!(f, ",")?;
                    }
                }

                Ok(())
            }
            QuestionSubmissionData::Scale { value } => {
                if let Some(value) = value {
                    write!(f, "{}", value)
                } else {
                    write!(f, "unspecified")
                }
            }
            QuestionSubmissionData::Address { address, point: _ } => address.fmt(f),
            QuestionSubmissionData::PhoneNumber {
                calling_code,
                number,
            } => write!(f, "+{}{}", calling_code, number),
            QuestionSubmissionData::FileUpload {
                file_id,
                content_type: _,
            } => {
                write!(f, "{}", file_id)
            }
            QuestionSubmissionData::Signature {
                freeform,
                initial,
                full_name,
            } => {
                if !freeform.is_empty() {
                    write!(f, "freeform signature")
                } else if !initial.is_empty() {
                    write!(f, "{}", initial)
                } else if !full_name.is_empty() {
                    write!(f, "{}", full_name)
                } else {
                    write!(f, "")
                }
            }
            QuestionSubmissionData::DateTime { value } => {
                write!(f, "{}", value.to_rfc3339_opts(SecondsFormat::Millis, true))
            }
            QuestionSubmissionData::Hidden { value } => {
                write!(f, "{}", value)
            }
        }
    }
}

impl TryFrom<APIQuestion> for QuestionSubmission {
    type Error = String;
    fn try_from(value: APIQuestion) -> Result<Self, Self::Error> {
        let data: QuestionSubmissionData = match value.configuration {
            APIQuestionConfiguration::Info {
                background_color: _,
            } => return Err("Cannot create submission for Info question".to_string()),
            APIQuestionConfiguration::Text {
                is_long: _,
                validator: _,
            } => QuestionSubmissionData::Text {
                value: String::default(),
            },
            APIQuestionConfiguration::Choice {
                options: _,
                multi: _,
            } => QuestionSubmissionData::Choice { option: vec![] },
            APIQuestionConfiguration::ChoiceMatrix {
                columns: _,
                rows: _,
                multi_cols: _,
            } => QuestionSubmissionData::ChoiceMatrix {
                options: HashMap::new(),
            },
            APIQuestionConfiguration::Scale {
                min: _,
                min_label: _,
                max: _,
                max_label: _,
                icon: _,
            } => QuestionSubmissionData::Scale { value: None },
            APIQuestionConfiguration::Address { search_centre: _ } => {
                QuestionSubmissionData::Address {
                    address: APIGenericAddress::default(),
                    point: APIGenericLocation::default(),
                }
            }
            APIQuestionConfiguration::PhoneNumber {} => QuestionSubmissionData::PhoneNumber {
                calling_code: String::default(),
                number: String::default(),
            },
            APIQuestionConfiguration::FileUpload { allowed_types: _ } => {
                QuestionSubmissionData::FileUpload {
                    file_id: String::default(),
                    content_type: String::default(),
                }
            }
            APIQuestionConfiguration::Signature {
                allow_freeform: _,
                allow_initial: _,
                allow_full_name: _,
            } => QuestionSubmissionData::Signature {
                freeform: Vec::new(),
                initial: String::default(),
                full_name: String::default(),
            },
            APIQuestionConfiguration::DateTime {
                collect_date: _,
                collect_time: _,
                min,
                max,
            } => QuestionSubmissionData::DateTime {
                value: min.unwrap_or(max.unwrap_or(Local::now())),
            },
            APIQuestionConfiguration::Hidden { parameter_name: _ } => {
                QuestionSubmissionData::Hidden {
                    value: String::default(),
                }
            }
        };
        Ok(Self {
            question_id: value.id,
            data,
        })
    }
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn api_question_default_submission(
    api_question: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use crate::wasm_serializer::get_wasm_serializer;

    let api_question = serde_wasm_bindgen::from_value::<APIQuestion>(api_question)?;
    let default_submission = QuestionSubmission::try_from(api_question)
        .map_err(|e| wasm_bindgen::JsValue::from_str(e.as_str()))?;
    let submission_data = default_submission.data.serialize(&get_wasm_serializer())?;
    Ok(submission_data)
}
