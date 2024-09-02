#[cfg(feature = "frontend-js")]
pub fn get_wasm_serializer() -> serde_wasm_bindgen::Serializer {
    serde_wasm_bindgen::Serializer::new().serialize_missing_as_null(true)
}
