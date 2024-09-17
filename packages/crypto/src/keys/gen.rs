use core::time;

use sequoia_openpgp::{cert::CertBuilder, serialize::SerializeInto};
use serde::{Deserialize, Serialize};

use crate::policy::NEW_KEY_SUITE;

#[cfg_attr(
    feature = "frontend-js",
    wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
)]
#[derive(Serialize, Deserialize, Clone)]
pub struct NewKeypair {
    pub public: String,
    pub private: String,
    pub key_id: String,
}

pub fn generate_certificate(
    org_id: String,
    user_id: String,
    validity_period: time::Duration,
) -> Result<NewKeypair, anyhow::Error> {
    let (cert, _) = CertBuilder::new()
        .set_cipher_suite(NEW_KEY_SUITE)
        .set_validity_period(if validity_period.is_zero() {
            time::Duration::from_secs(u64::from(u32::MAX))
        } else {
            validity_period
        })
        .add_userid(format!("palform_{}_{}", org_id, user_id))
        .add_storage_encryption_subkey()
        .generate()?;

    let tsk_vec = cert.as_tsk().armored().to_vec()?;
    let secret_pem = String::from_utf8(tsk_vec)?;

    let cert_vec = cert.armored().to_vec()?;
    let public_pem = String::from_utf8(cert_vec)?;

    Ok(NewKeypair {
        key_id: cert.fingerprint().to_hex(),
        public: public_pem,
        private: secret_pem,
    })
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn generate_certificate_js(
    org_id: String,
    user_id: String,
    validity_seconds: u32,
) -> Result<NewKeypair, wasm_bindgen::JsValue> {
    generate_certificate(
        org_id,
        user_id,
        time::Duration::new(u64::from(validity_seconds), 0),
    )
    .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
