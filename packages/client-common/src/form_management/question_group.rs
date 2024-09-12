use std::collections::HashSet;

use anyhow::anyhow;
use chrono::{DateTime, Local};
use geo::{GeodesicDistance, Point};
use palform_tsid::{resources::{IDQuestion, IDQuestionGroup}, tsid::PalformDatabaseID};
use serde::{Deserialize, Serialize};

use crate::{address::APIGenericLocation, datetime::normalise_date_time};

use super::submission::{QuestionSubmission, QuestionSubmissionData};

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Serialize, Deserialize)]
pub struct APIQuestionGroup {
    pub id: PalformDatabaseID<IDQuestionGroup>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub step_strategy: APIQuestionGroupStepStrategy,
}

impl APIQuestionGroup {
    pub fn next_step(
        &self,
        steps: Vec<APIQuestionGroup>,
        submission: Vec<QuestionSubmission>,
    ) -> Result<Option<PalformDatabaseID<IDQuestionGroup>>, anyhow::Error> {
        let self_index = steps
            .iter()
            .position(|e| e.id == self.id)
            .ok_or(anyhow!("Current question group not found"))?;

        if let APIQuestionGroupStepStrategy::NextPosition = self.step_strategy.clone() {
            return Ok(steps.get(self_index + 1).map(|e| e.id.clone()));
        }

        if let APIQuestionGroupStepStrategy::JumpToSection(cases) = self.step_strategy.clone() {
            if cases.is_empty() {
                return Ok(None);
            }

            for case in cases {
                if case.check_condition_match(&submission)? {
                    return Ok(case.target_group_id);
                }
            }
        }

        Ok(None)
    }
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn next_question_group_step_js(
    current_step_id: String,
    all_steps: wasm_bindgen::JsValue,
    submissions: wasm_bindgen::JsValue,
) -> Result<Option<String>, wasm_bindgen::JsValue> {
    let step_list: Vec<APIQuestionGroup> = serde_wasm_bindgen::from_value(all_steps)?;
    let submission_list: Vec<QuestionSubmission> = serde_wasm_bindgen::from_value(submissions)?;

    let current_step_id = PalformDatabaseID::<IDQuestionGroup>::from_str(&current_step_id)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    let current_step = step_list
        .iter()
        .find(|e| e.id == current_step_id)
        .ok_or(wasm_bindgen::JsValue::from_str("Cannot find current group"))?;

    let next_step = current_step
        .next_step(step_list.clone(), submission_list)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;

    Ok(next_step.map(|e| e.to_string()))
}

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize)]
pub enum APIQuestionGroupStepStrategy {
    /// Continue to the group with the next `position`. This is the default option.
    NextPosition,
    /// Jump to one of a list of sections, based on which one's condition is the first to match
    JumpToSection(Vec<APIQuestionGroupStepStrategyJumpCase>),
}

impl Default for APIQuestionGroupStepStrategy {
    fn default() -> Self {
        Self::NextPosition
    }
}

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize)]
pub struct APIQuestionGroupStepStrategyJumpCase {
    /// If Some(uuid) jump to the group with that `uuid`. If None, submit the form.
    target_group_id: Option<PalformDatabaseID<IDQuestionGroup>>,
    conditions: APIQuestionGroupStepStrategyJumpCaseConditionList,
}

impl APIQuestionGroupStepStrategyJumpCase {
    pub fn check_condition_match(
        &self,
        submissions: &Vec<QuestionSubmission>,
    ) -> Result<bool, anyhow::Error> {
        match self.conditions.clone() {
            APIQuestionGroupStepStrategyJumpCaseConditionList::Or(or_list) => {
                if or_list.is_empty() {
                    return Ok(true);
                }

                for condition in or_list {
                    if condition.check_condition_match(submissions)? {
                        return Ok(true);
                    }
                }

                return Ok(false);
            }
            APIQuestionGroupStepStrategyJumpCaseConditionList::And(and_list) => {
                for condition in and_list {
                    if !condition.check_condition_match(submissions)? {
                        return Ok(false);
                    }
                }

                return Ok(true);
            }
        }
    }
}

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize)]
pub enum APIQuestionGroupStepStrategyJumpCaseConditionList {
    Or(Vec<APIQuestionGroupStepStrategyJumpCaseCondition>),
    And(Vec<APIQuestionGroupStepStrategyJumpCaseCondition>),
}

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize)]
pub struct APIQuestionGroupStepStrategyJumpCaseCondition {
    question_id: PalformDatabaseID<IDQuestion>,
    matcher: APIQuestionGroupStepStrategyJumpCaseConditionMatcher,
}

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize)]
pub enum DirectionOperator {
    GreaterThan,
    GreaterThanEqualTo,
    LessThan,
    LessThanEqualTo,
    Equal,
}

#[cfg_attr(feature = "backend", derive(schemars::JsonSchema))]
#[derive(Clone, Deserialize, Serialize)]
pub enum APIQuestionGroupStepStrategyJumpCaseConditionMatcher {
    /// A single- or multi- choice question exactly matches this _set_ of items (regardless of
    /// order)
    Choice {
        options: Vec<String>,
        contains_any: bool,
    },
    Text {
        case_sensitive: bool,
        /// Checks exact match if false, otherwise checks whether the value is contained
        contains: bool,
        value: String,
    },
    Scale {
        direction: DirectionOperator,
        value: i32,
    },
    PhoneNumber {
        calling_code: String,
    },
    Address {
        near: Option<APIGenericLocation>,
        near_radius_km: Option<f64>,
        in_country: Option<String>,
    },
    ChoiceMatrix {
        row: String,
        column: String,
    },
    DateTime {
        direction: DirectionOperator,
        value: DateTime<Local>,
        match_date: bool,
        match_time: bool,
    },
    Hidden {
        value: String,
    },
}

impl APIQuestionGroupStepStrategyJumpCaseCondition {
    fn find_submission(
        question_id: &PalformDatabaseID<IDQuestion>,
        submissions: &Vec<QuestionSubmission>,
    ) -> Result<QuestionSubmission, anyhow::Error> {
        submissions
            .iter()
            .find(|e| &e.question_id == question_id)
            .ok_or(anyhow!(
                "Corresponding submission for question {} not found",
                question_id
            ))
            .cloned()
    }

    pub fn check_condition_match(
        &self,
        submissions: &Vec<QuestionSubmission>,
    ) -> Result<bool, anyhow::Error> {
        let submission = Self::find_submission(&self.question_id, submissions)?;
        match self.matcher.clone() {
            APIQuestionGroupStepStrategyJumpCaseConditionMatcher::Choice {
                options,
                contains_any,
            } => {
                if let QuestionSubmissionData::Choice {
                    option: selected_options,
                } = submission.data
                {
                    let target_options_set: HashSet<String> =
                        HashSet::from_iter(options.iter().cloned());

                    if contains_any {
                        for option in selected_options {
                            if target_options_set.contains(&option) {
                                return Ok(true);
                            }
                        }

                        return Ok(false);
                    } else {
                        let selected_options_set =
                            HashSet::from_iter(selected_options.iter().cloned());
                        return Ok(selected_options_set == target_options_set);
                    }
                } else {
                    return Err(anyhow!("Submission was not for a Choice"));
                }
            }
            APIQuestionGroupStepStrategyJumpCaseConditionMatcher::Text {
                case_sensitive,
                contains,
                value: target_value,
            } => {
                if let QuestionSubmissionData::Text {
                    value: actual_value,
                } = submission.data
                {
                    let target_value = if case_sensitive {
                        target_value
                    } else {
                        target_value.to_lowercase()
                    };
                    let actual_value = if case_sensitive {
                        actual_value
                    } else {
                        actual_value.to_lowercase()
                    };

                    if contains {
                        Ok(actual_value.contains(target_value.as_str()))
                    } else {
                        Ok(actual_value == target_value)
                    }
                } else {
                    return Err(anyhow!("Submission was not for Text"));
                }
            }
            APIQuestionGroupStepStrategyJumpCaseConditionMatcher::Scale { direction, value } => {
                if let QuestionSubmissionData::Scale {
                    value: actual_value,
                } = submission.data
                {
                    Ok(actual_value.is_some_and(|actual_value| match direction {
                        DirectionOperator::GreaterThan => actual_value > value,
                        DirectionOperator::GreaterThanEqualTo => actual_value >= value,
                        DirectionOperator::LessThan => actual_value < value,
                        DirectionOperator::LessThanEqualTo => actual_value <= value,
                        DirectionOperator::Equal => value == actual_value,
                    }))
                } else {
                    Err(anyhow!("Submission was not for Scale"))
                }
            }
            APIQuestionGroupStepStrategyJumpCaseConditionMatcher::PhoneNumber { calling_code } => {
                if let QuestionSubmissionData::PhoneNumber {
                    calling_code: actual_calling_code,
                    number: _,
                } = submission.data
                {
                    Ok(actual_calling_code == calling_code)
                } else {
                    Err(anyhow!("Submission was not for PhoneNumber"))
                }
            }
            APIQuestionGroupStepStrategyJumpCaseConditionMatcher::Address {
                near,
                near_radius_km: near_radius,
                in_country,
            } => {
                if let QuestionSubmissionData::Address { address, point } = submission.data {
                    if let Some(near) = near {
                        let submission_point = Point::from(point);
                        let target_point = Point::from(near);
                        let dist = submission_point.geodesic_distance(&target_point);

                        if dist > near_radius.map(|v| v * 1000_f64).unwrap_or(20_000_f64) {
                            return Ok(false);
                        }
                    }

                    if let Some(in_country) = in_country {
                        if !address.is_in_country(in_country) {
                            return Ok(false);
                        }
                    }

                    Ok(true)
                } else {
                    Err(anyhow!("Submission was not for Address"))
                }
            }
            APIQuestionGroupStepStrategyJumpCaseConditionMatcher::ChoiceMatrix { row, column } => {
                if let QuestionSubmissionData::ChoiceMatrix { options } = submission.data {
                    Ok(options.get(&row).is_some_and(|v| v.contains(&column)))
                } else {
                    Err(anyhow!("Submission was not for ChoiceMatrix"))
                }
            }
            APIQuestionGroupStepStrategyJumpCaseConditionMatcher::DateTime {
                direction,
                value,
                match_date,
                match_time,
            } => {
                let normalised_value = normalise_date_time(value, match_date, match_time)?;

                if let QuestionSubmissionData::DateTime {
                    value: actual_value,
                } = submission.data
                {
                    let normalised_actual_value =
                        normalise_date_time(actual_value, match_date, match_time)?;

                    Ok(match direction {
                        DirectionOperator::Equal => normalised_actual_value == normalised_value,
                        DirectionOperator::LessThan => normalised_actual_value < normalised_value,
                        DirectionOperator::LessThanEqualTo => {
                            normalised_actual_value <= normalised_value
                        }
                        DirectionOperator::GreaterThan => {
                            normalised_actual_value > normalised_value
                        }
                        DirectionOperator::GreaterThanEqualTo => {
                            normalised_actual_value >= normalised_value
                        }
                    })
                } else {
                    Err(anyhow!("Submission was not for DateTime"))
                }
            }
            APIQuestionGroupStepStrategyJumpCaseConditionMatcher::Hidden { value } => {
                if let QuestionSubmissionData::Hidden {
                    value: actual_value,
                } = submission.data
                {
                    Ok(actual_value == value)
                } else {
                    Err(anyhow!("Submission was not for Hidden"))
                }
            }
        }
    }
}
