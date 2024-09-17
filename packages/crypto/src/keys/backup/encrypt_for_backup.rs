use sequoia_openpgp::{
    cert::amalgamation::key::PrimaryKey, crypto::Password, serialize::SerializeInto, Packet,
};

use crate::{keys::parse::secret_key_from_pem, policy::recipient_cert_policy};

pub fn encrypt_key_for_backup(
    secret_pem: String,
    passphrase: String,
) -> Result<String, anyhow::Error> {
    let cert = secret_key_from_pem(secret_pem)?;
    let p = recipient_cert_policy();
    let valid_cert = cert.with_policy(&p, None)?;

    let mut encrypted_keys: Vec<Packet> = Vec::new();
    let password: Password = passphrase.into();
    for key in valid_cert.keys().unencrypted_secret() {
        let encrypted_key = key.key().clone().encrypt_secret(&password)?;
        encrypted_keys.push(if key.primary() {
            encrypted_key.role_into_primary().into()
        } else {
            encrypted_key.role_into_subordinate().into()
        })
    }

    let cert = cert.insert_packets(encrypted_keys)?;
    let encrypted_cert_pem = cert.as_tsk().armored().to_vec()?;
    Ok(String::from_utf8(encrypted_cert_pem)?)
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn encrypt_key_for_backup_js(
    secret_pem: String,
    passphrase: String,
) -> Result<String, wasm_bindgen::JsValue> {
    encrypt_key_for_backup(secret_pem, passphrase)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
