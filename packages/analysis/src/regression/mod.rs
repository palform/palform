use ndarray::{Array2, Axis};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{common::FormAnalysisManager, util::array::array2_to_vec};

#[derive(Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct LinearRegressionResult {
    pub intercept: f64,
    pub gradient: f64,
    pub points: JsValue,
}

#[wasm_bindgen]
impl FormAnalysisManager {
    pub fn regression_for_question_pair(
        &self,
        from_question_id: String,
        from_feature_label: String,
        to_question_id: String,
        to_feature_label: String,
    ) -> Result<LinearRegressionResult, JsValue> {
        let mut mini_feature_mat = Array2::<f64>::zeros((0, self.feature_mat.len_of(Axis(1))));

        let from_row = self
            .get_feature_row_for_question_with_label(from_question_id, from_feature_label)
            .map_err(|e| JsValue::from(&e.to_string()))?;
        mini_feature_mat
            .push_row(from_row)
            .map_err(|e| JsValue::from(format!("push feature row to features: {}", e)))?;

        let target_row = self
            .get_feature_row_for_question_with_label(to_question_id, to_feature_label)
            .map_err(|e| JsValue::from(&e.to_string()))?;
        mini_feature_mat
            .push_row(target_row)
            .map_err(|e| JsValue::from(format!("push target row to features: {}", e)))?;

        // Ordinary least squares with a single explanatory variable
        let n = from_row.len() as f64;
        if n == 0.0 {
            return Err(JsValue::from("run linear regression: no data points"));
        }
        let x_mean = from_row.sum() / n;
        let y_mean = target_row.sum() / n;
        let x_dev = from_row.mapv(|x| x - x_mean);
        let x_variance = x_dev.mapv(|x| x * x).sum();
        if x_variance == 0.0 {
            return Err(JsValue::from(
                "run linear regression: all values of the explanatory variable are identical",
            ));
        }
        let gradient = (&x_dev * &target_row.mapv(|y| y - y_mean)).sum() / x_variance;
        let intercept = y_mean - gradient * x_mean;

        let points = array2_to_vec(mini_feature_mat);
        let points = serde_wasm_bindgen::to_value(&points)
            .map_err(|e| JsValue::from(format!("serialize: {}", e)))?;

        Ok(LinearRegressionResult {
            points,
            intercept,
            gradient,
        })
    }
}
