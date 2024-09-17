use super::key_resolver::KeyResolver;
use crate::policy::recipient_cert_policy;
use anyhow::anyhow;
use palform_client_common::form_management::submission::InProgressSubmission;
use sequoia_openpgp::parse::{stream::DecryptorBuilder, Parse};
use std::io::Read;

fn decrypt_anything(data: &[u8], key_resolver: KeyResolver) -> Result<Vec<u8>, anyhow::Error> {
    let p = recipient_cert_policy();
    let mut v = DecryptorBuilder::from_bytes(data)?.with_policy(&p, None, key_resolver)?;

    let mut content = Vec::new();
    v.read_to_end(&mut content)?;

    Ok(content)
}

fn decode_submission_message(
    decrypted_message: Vec<u8>,
) -> Result<InProgressSubmission, anyhow::Error> {
    serde_json::from_slice::<InProgressSubmission>(&decrypted_message)
        .map_err(|e| anyhow!("parse submission: {}", e))
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn decrypt_decode_submission_js(
    message_pem: String,
    frontend_key_store: &KeyResolver,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let decrypted_message = decrypt_anything(message_pem.as_bytes(), frontend_key_store.to_owned())
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    let decoded_message = decode_submission_message(decrypted_message)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&decoded_message).map_err(|e| e.into())
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn decrypt_blob_js(
    data: String,
    frontend_key_store: &KeyResolver,
) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    decrypt_anything(data.as_bytes(), frontend_key_store.to_owned())
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
