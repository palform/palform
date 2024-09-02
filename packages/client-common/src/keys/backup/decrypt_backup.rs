use sequoia_openpgp::{
    cert::amalgamation::key::PrimaryKey, crypto::Password, serialize::SerializeInto, Packet,
};
use serde::{Deserialize, Serialize};

use crate::{keys::parse::secret_key_from_pem, policy::recipient_cert_policy};

#[cfg_attr(
    feature = "frontend-js",
    wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
)]
#[derive(Serialize, Deserialize, Clone)]
pub struct RestoredKeyResponse {
    pub decrypted_private_pem: String,
    pub fingerprint: String,
}

pub fn decrypt_backed_up_key(
    encrypted_secret_pem: String,
    passphrase: String,
) -> Result<RestoredKeyResponse, anyhow::Error> {
    let cert = secret_key_from_pem(encrypted_secret_pem)?;
    let p = recipient_cert_policy();
    let valid_cert = cert.with_policy(&p, None)?;

    let mut decrypted_keys: Vec<Packet> = Vec::new();
    let password: Password = passphrase.into();
    for key in valid_cert.keys().encrypted_secret() {
        let decrypted_key = key.key().clone().decrypt_secret(&password)?;
        decrypted_keys.push(if key.primary() {
            decrypted_key.role_into_primary().into()
        } else {
            decrypted_key.role_into_subordinate().into()
        })
    }


    let cert = cert.insert_packets(decrypted_keys)?;
    let decrypted_cert_pem = cert.as_tsk().armored().to_vec()?;
    Ok(RestoredKeyResponse {
        decrypted_private_pem: String::from_utf8(decrypted_cert_pem)?,
        fingerprint: cert.fingerprint().to_hex(),
    })
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn decrypt_backed_up_key_js(
    encrypted_secret_pem: String,
    passphrase: String,
) -> Result<RestoredKeyResponse, wasm_bindgen::JsValue> {
    decrypt_backed_up_key(encrypted_secret_pem, passphrase)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
