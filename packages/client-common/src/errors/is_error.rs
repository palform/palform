#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn is_api_error_js(value: wasm_bindgen::JsValue) -> bool {
    serde_wasm_bindgen::from_value::<super::error::APIError>(value).is_ok()
}
