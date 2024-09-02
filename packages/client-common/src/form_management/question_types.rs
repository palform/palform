use anyhow::anyhow;
use chrono::{DateTime, Local};
use palform_tsid::{
    resources::{IDQuestion, IDQuestionGroup},
    tsid::PalformDatabaseID,
};
use serde::{Deserialize, Serialize};

use crate::address::APIGenericLocation;

use super::question_group::APIQuestionGroup;

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize)]
pub struct APIQuestion {
    pub id: PalformDatabaseID<IDQuestion>,
    pub title: String,
    pub description: Option<String>,
    pub required: bool,
    pub configuration: APIQuestionConfiguration,
    pub position: i32,
    pub group_id: PalformDatabaseID<IDQuestionGroup>,
}

impl APIQuestion {
    pub fn to_export_key(
        &self,
        groups: &Vec<APIQuestionGroup>,
        use_question_id: bool,
        exclude_prefix: bool,
        use_prefix_group_id: bool,
    ) -> Option<String> {
        let prefix = {
            if exclude_prefix {
                Some(String::default())
            } else {
                let mut v = None;
                if use_prefix_group_id {
                    v = Some(self.group_id.to_string())
                } else {
                    for (index, group) in groups.iter().enumerate() {
                        if group.id != self.group_id {
                            continue;
                        }
                        v = Some(
                            group
                                .title
                                .clone()
                                .unwrap_or(format!("Section {}", index + 1)),
                        )
                    }
                }
                v.map(|e| format!("{}#", e))
            }
        };

        if let Some(prefix) = prefix {
            if use_question_id {
                Some(format!("{}{}", prefix, self.id.to_string()))
            } else {
                Some(format!("{}{}", prefix, self.title.clone()))
            }
        } else {
            None
        }
    }
}

impl PartialEq for APIQuestion {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.title == other.title
            && self.description == other.description
            && self.required == other.required
            && self.configuration == other.configuration
    }
}

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum APIQuestionConfiguration {
    /// A purely informational non-interactive question that serves as (e.g.) a section heading.
    #[serde(rename = "info")]
    Info { background_color: Option<String> },
    #[serde(rename = "text")]
    Text {
        /// Will use a textarea instead of an input
        is_long: bool,
        /// Use client-side validation to ensure a valid input. Not validated server-side due to
        /// encryption, so this doesn't guarantee valid data.
        validator: Option<APIQuestionTextValidator>,
    },
    #[serde(rename = "choice")]
    Choice {
        options: Vec<String>,
        /// Support multiple options in a submission (i.e. checkboxes instead of radio buttons)
        multi: bool,
    },
    #[serde(rename = "scale")]
    Scale {
        min: i32,
        min_label: Option<String>,
        max: i32,
        max_label: Option<String>,
        #[serde(default)]
        icon: APIQuestionScaleIcon,
    },
    #[serde(rename = "address")]
    Address {
        search_centre: Option<APIGenericLocation>,
    },
    #[serde(rename = "phone_number")]
    PhoneNumber {},
    #[serde(rename = "file_upload")]
    FileUpload {
        allowed_types: Vec<APIQuestionFileUploadType>,
    },
    #[serde(rename = "signature")]
    Signature {
        allow_freeform: bool,
        allow_initial: bool,
        allow_full_name: bool,
    },
    #[serde(rename = "choice_matrix")]
    ChoiceMatrix {
        columns: Vec<String>,
        rows: Vec<String>,
        multi_cols: bool,
    },
    #[serde(rename = "date_time")]
    DateTime {
        collect_date: bool,
        collect_time: bool,
        min: Option<DateTime<Local>>,
        max: Option<DateTime<Local>>,
    },
    #[serde(rename = "hidden")]
    Hidden { parameter_name: String },
}

impl APIQuestionConfiguration {
    pub fn requires_submission(&self) -> bool {
        if let Self::Info {
            background_color: _,
        } = self
        {
            false
        } else {
            true
        }
    }
}

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum APIQuestionTextValidator {
    Email,
    Integer,
    Float,
}

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum APIQuestionFileUploadType {
    Image,
    Video,
    Document,
    Slideshow,
    Spreadsheet,
    Any,
}

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum APIQuestionScaleIcon {
    Numeric,
    Stars,
    Hearts,
    Thumbs,
}

impl Default for APIQuestionScaleIcon {
    fn default() -> Self {
        Self::Numeric
    }
}

impl APIQuestionConfiguration {
    pub fn default_for_type(
        question_type: String,
    ) -> Result<APIQuestionConfiguration, anyhow::Error> {
        match question_type.as_str() {
            "info" | "Info" => Ok(APIQuestionConfiguration::Info {
                background_color: None,
            }),
            "text" | "Text" => Ok(APIQuestionConfiguration::Text {
                is_long: false,
                validator: None,
            }),
            "choice" | "Choice" => Ok(APIQuestionConfiguration::Choice {
                options: vec![
                    "Option 1".to_string(),
                    "Option 2".to_string(),
                    "Option 3".to_string(),
                ],
                multi: false,
            }),
            "choice_matrix" | "ChoiceMatrix" => Ok(APIQuestionConfiguration::ChoiceMatrix {
                columns: vec![
                    "Option 1".to_string(),
                    "Option 2".to_string(),
                    "Option 3".to_string(),
                ],
                rows: vec![
                    "Item 1".to_string(),
                    "Item 2".to_string(),
                    "Item 3".to_string(),
                ],
                multi_cols: false,
            }),
            "scale" | "Scale" => Ok(APIQuestionConfiguration::Scale {
                min: 0,
                min_label: None,
                max: 10,
                max_label: None,
                icon: APIQuestionScaleIcon::default(),
            }),
            "address" | "Address" => Ok(APIQuestionConfiguration::Address {
                search_centre: None,
            }),
            "phone_number" | "PhoneNumber" => Ok(APIQuestionConfiguration::PhoneNumber {}),
            "file_upload" | "FileUpload" => Ok(APIQuestionConfiguration::FileUpload {
                allowed_types: vec![APIQuestionFileUploadType::Any],
            }),
            "signature" | "Signature" => Ok(APIQuestionConfiguration::Signature {
                allow_freeform: true,
                allow_initial: true,
                allow_full_name: true,
            }),
            "date_time" | "DateTime" => Ok(APIQuestionConfiguration::DateTime {
                collect_date: true,
                collect_time: false,
                min: None,
                max: None,
            }),
            "hidden" | "Hidden" => Ok(APIQuestionConfiguration::Hidden {
                parameter_name: String::default(),
            }),
            _ => Err(anyhow!("Question type {} not identified", question_type)),
        }
    }
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn default_question_for_type_js(
    question_type: String,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use crate::wasm_serializer::get_wasm_serializer;

    let question = APIQuestionConfiguration::default_for_type(question_type)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    let resp = question.serialize(&get_wasm_serializer())?;
    Ok(resp)
}
