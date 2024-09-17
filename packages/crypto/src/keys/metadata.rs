use crate::{keys::parse::secret_or_public_key_from_pem, policy::recipient_cert_policy};

use super::encryption_key::resolve_encryption_key;

#[derive(Clone)]
#[cfg_attr(
    feature = "frontend-js",
    wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
)]
pub struct KeyMetadata {
    pub fingerprint: String,
    pub algo: String,
    pub has_secret: bool,
}

pub fn get_key_metadata(cert_pem: String) -> Result<KeyMetadata, anyhow::Error> {
    let cert = secret_or_public_key_from_pem(cert_pem)?;
    let policy = recipient_cert_policy();
    let enc_key = resolve_encryption_key(&cert, &policy)?;
    Ok(KeyMetadata {
        fingerprint: cert.fingerprint().to_hex(),
        algo: format!("{}", enc_key.pk_algo()),
        has_secret: enc_key.has_unencrypted_secret(),
    })
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn get_key_metadata_js(cert_pem: String) -> Result<KeyMetadata, wasm_bindgen::JsValue> {
    get_key_metadata(cert_pem).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
