use anyhow::anyhow;
use interp::interp;
use palform_client_common::form_management::{
    question_types::APIQuestionConfiguration, submission::QuestionSubmissionData,
};

pub(crate) trait Featureable {
    fn feature_value(
        &self,
        interp_y: &[f64],
        configuration: &APIQuestionConfiguration,
        index: usize,
    ) -> Result<Option<f64>, anyhow::Error>;

    fn feature_labels(configuration: &APIQuestionConfiguration) -> Vec<String>;
}

impl Featureable for QuestionSubmissionData {
    fn feature_labels(configuration: &APIQuestionConfiguration) -> Vec<String> {
        match configuration {
            APIQuestionConfiguration::Scale {
                min: _,
                min_label: _,
                max: _,
                max_label: _,
                icon: _,
            } => vec![String::default()],
            APIQuestionConfiguration::Choice { options, multi: _ } => options.clone(),
            APIQuestionConfiguration::ChoiceMatrix {
                columns,
                rows,
                multi_cols: _,
            } => {
                let mut labels = Vec::<String>::new();
                for row in rows {
                    for col in columns {
                        labels.push(format!("{}: {}", row, col));
                    }
                }

                labels
            }
            APIQuestionConfiguration::Address { search_centre: _ } => {
                vec!["Latitude".to_string(), "Longitude".to_string()]
            }
            APIQuestionConfiguration::DateTime {
                collect_date: _,
                collect_time: _,
                min: _,
                max: _,
            } => vec![String::default()],
            _ => vec![],
        }
    }

    fn feature_value(
        &self,
        interp_y: &[f64],
        configuration: &APIQuestionConfiguration,
        index: usize,
    ) -> Result<Option<f64>, anyhow::Error> {
        let incorrect_config_err = anyhow!("Configuration was not the right type");

        Ok(match self {
            QuestionSubmissionData::Scale { value } => {
                if let Some(value) = value {
                    if let APIQuestionConfiguration::Scale {
                        min,
                        min_label: _,
                        max,
                        max_label: _,
                        icon: _,
                    } = configuration
                    {
                        let x = vec![f64::from(min.clone()), f64::from(max.clone())];
                        Some(interp(&x, interp_y, f64::from(value.to_owned())))
                    } else {
                        return Err(incorrect_config_err);
                    }
                } else {
                    None
                }
            }
            QuestionSubmissionData::Choice { option } => {
                if let APIQuestionConfiguration::Choice { options, multi: _ } = configuration {
                    let selected_option = options.get(index);

                    if let Some(selected_option) = selected_option {
                        let x = vec![0_f64, 1_f64];
                        if option.contains(selected_option) {
                            Some(interp(&x, interp_y, 1_f64))
                        } else {
                            Some(interp(&x, interp_y, 0_f64))
                        }
                    } else {
                        None
                    }
                } else {
                    return Err(incorrect_config_err);
                }
            }
            QuestionSubmissionData::ChoiceMatrix { options } => {
                if let APIQuestionConfiguration::ChoiceMatrix {
                    columns,
                    rows,
                    multi_cols: _,
                } = configuration
                {
                    let col_count = columns.len();
                    let row_index = (index as f64 / col_count as f64).floor() as usize;
                    let col_index = ((index as f64 % col_count as f64) - 1_f64) as usize;

                    let row_label = rows
                        .get(row_index)
                        .ok_or(anyhow!("Index not found for row"))?;
                    let col_label = columns
                        .get(col_index)
                        .ok_or(anyhow!("Index not found for col"))?;

                    let exists = options
                        .get(row_label)
                        .is_some_and(|v| v.contains(col_label));
                    let x = vec![0_f64, 1_f64];

                    Some(interp(&x, interp_y, if exists { 1_f64 } else { 0_f64 }))
                } else {
                    return Err(incorrect_config_err);
                }
            }
            QuestionSubmissionData::Address { address: _, point } => {
                if index == 0 {
                    let lati_x = vec![-90_f64, 90_f64];
                    let lati = interp(&lati_x, interp_y, point.get_lat());
                    Some(lati)
                } else if index == 1 {
                    let longi_x = vec![-180_f64, 180_f64];
                    let longi = interp(&longi_x, interp_y, point.get_lng());
                    Some(longi)
                } else {
                    None
                }
            }
            QuestionSubmissionData::DateTime { value } => {
                if let APIQuestionConfiguration::DateTime {
                    collect_date: _,
                    collect_time: _,
                    min,
                    max,
                } = configuration
                {
                    let min = min.map_or(0, |v| v.timestamp_millis());
                    let max = max.map_or(1893452400, |v| v.timestamp_millis());
                    let x = vec![min as f64, max as f64];
                    Some(interp(&x, interp_y, value.timestamp_millis() as f64))
                } else {
                    return Err(incorrect_config_err);
                }
            }
            _ => None,
        })
    }
}
