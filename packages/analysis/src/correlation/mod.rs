use std::collections::HashMap;

use ndarray_stats::CorrelationExt;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::common::FormAnalysisManager;

pub type CorrelationHashMap =
    HashMap<String, HashMap<String, HashMap<String, HashMap<String, f64>>>>;

#[wasm_bindgen]
impl FormAnalysisManager {
    pub fn question_influencers(&self) -> Result<JsValue, JsValue> {
        if self.feature_mat.len() == 0 {
            let m = CorrelationHashMap::new();
            return serde_wasm_bindgen::to_value(&m).map_err(|e| e.into());
        }

        let corr = self
            .feature_mat
            .pearson_correlation()
            .map_err(|e| JsValue::from(format!("build correlation matrix: {}", e)))?;

        let mut m = CorrelationHashMap::new();
        for (from_feature_index, from_feature_row) in corr.rows().into_iter().enumerate() {
            let from_question_id = self
                .get_question_label_at_feature_index(from_feature_index)
                .map_err(|e| JsValue::from(format!("Missing outer label index: {}", e)))?;

            let from_feature_label =
                self.feature_labels
                    .get(from_feature_index)
                    .ok_or(JsValue::from(format!(
                        "Missing outer feature label index: {}",
                        from_feature_index
                    )))?;

            for (to_feature_index, correlation_value) in from_feature_row.into_iter().enumerate() {
                let to_question_id = self
                    .get_question_label_at_feature_index(to_feature_index)
                    .map_err(|e| JsValue::from(&e.to_string()))?;

                if to_question_id == from_question_id || from_feature_index == to_feature_index {
                    continue;
                }

                let to_feature_label =
                    self.feature_labels
                        .get(to_feature_index)
                        .ok_or(JsValue::from(format!(
                            "Missing inner (feature) label index: {}",
                            to_feature_index
                        )))?;

                m.entry(from_question_id.clone())
                    .or_default()
                    .entry(from_feature_label.clone())
                    .or_default()
                    .entry(to_question_id.clone())
                    .or_default()
                    .insert(to_feature_label.clone(), correlation_value.clone());
            }
        }

        let v = serde_wasm_bindgen::to_value(&m)?;
        Ok(v)
    }
}
