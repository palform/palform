use std::str::FromStr;

use anyhow::anyhow;
use sequoia_openpgp::{serialize::SerializeInto, Cert};

use super::validate::validate_cert;

pub fn strip_secret_bits(key_pem: String) -> Result<String, anyhow::Error> {
    let cert = Cert::from_str(&key_pem)?;
    validate_cert(&cert)?;
    String::from_utf8(cert.strip_secret_key_material().armored().to_vec()?).map_err(|e| anyhow!(e))
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn strip_secret_bits_js(key_pem: String) -> Result<String, wasm_bindgen::JsValue> {
    strip_secret_bits(key_pem).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
