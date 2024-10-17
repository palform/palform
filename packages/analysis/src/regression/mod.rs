use linfa::{traits::Fit, Dataset};
use linfa_linear::{FittedLinearRegression, LinearRegression};
use ndarray::{Array2, Axis, Order};
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

        let dataset = Dataset::new(
            from_row
                .to_shape(((from_row.len(), 1), Order::RowMajor))
                .map_err(|e| JsValue::from(format!("build dataset: {}", e)))?
                .to_owned(),
            target_row.to_owned(),
        );

        let lin_reg_model: FittedLinearRegression<f64> = LinearRegression::default()
            .fit(&dataset)
            .map_err(|e| JsValue::from(format!("run linear regression: {}", e)))?;
        let gradient = lin_reg_model.params()[0];

        let points = array2_to_vec(mini_feature_mat);
        let points = serde_wasm_bindgen::to_value(&points)
            .map_err(|e| JsValue::from(format!("serialize: {}", e)))?;

        Ok(LinearRegressionResult {
            points,
            intercept: lin_reg_model.intercept(),
            gradient,
        })
    }
}
