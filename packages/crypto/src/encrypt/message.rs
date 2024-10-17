use std::io::Write;

use anyhow::anyhow;
use palform_client_common::form_management::submission::InProgressSubmission;
use sequoia_openpgp::{
    armor::{Kind, Reader, ReaderMode, Writer},
    parse::Parse,
    serialize::stream::{Encryptor2, LiteralWriter, Message},
    Cert,
};

use crate::policy::recipient_cert_policy;

fn encrypt_anything(data: &[u8], recipient_certs: Vec<String>) -> Result<Vec<u8>, anyhow::Error> {
    let policy = recipient_cert_policy();

    let recipient_certs: Result<Vec<Cert>, anyhow::Error> = recipient_certs
        .iter()
        .map(|e| {
            let reader =
                Reader::from_bytes(e.as_bytes(), ReaderMode::Tolerant(Some(Kind::PublicKey)));
            Cert::from_reader(reader)
        })
        .collect();
    let recipient_certs = recipient_certs.map_err(|e| anyhow!("parse keys: {}", e))?;

    let mut recipients = Vec::new();
    for cert in recipient_certs.iter() {
        let valid_cert = cert.with_policy(&policy, None);
        match valid_cert {
            Ok(valid_cert) => {
                for key in valid_cert
                    .keys()
                    .supported()
                    .alive()
                    .revoked(false)
                    .for_storage_encryption()
                {
                    recipients.push(key);
                }
            }
            Err(_) => continue,
        }
    }

    if recipients.is_empty() {
        return Err(anyhow!(
            "No valid keys found. Please ask the form owner to configure their keys."
        ));
    }

    let mut sink = Vec::new();
    let message = Encryptor2::for_recipients(Message::new(&mut sink), recipients).build()?;
    let mut w = LiteralWriter::new(message).build()?;

    w.write_all(data)?;
    w.finalize()?;
    Ok(sink)
}

fn armor_message(message: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let mut armor_writer = Writer::new(Vec::new(), Kind::Message)?;
    armor_writer.write_all(message)?;
    Ok(armor_writer.finalize()?)
}

fn stringify_armor(armor: &[u8]) -> String {
    String::from_utf8_lossy(armor).to_string()
}

pub fn encrypt_submission(
    submission: InProgressSubmission,
    recipient_certs: Vec<String>,
) -> Result<Vec<u8>, anyhow::Error> {
    let submission =
        serde_json::to_vec(&submission).map_err(|e| anyhow!("serialize submission: {}", e))?;

    let encrypted = encrypt_anything(&submission, recipient_certs)?;
    Ok(encrypted)
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn encrypt_submission_js(
    submission: wasm_bindgen::JsValue,
    recipient_certs: Vec<String>,
) -> Result<String, wasm_bindgen::JsValue> {
    let submission: InProgressSubmission = serde_wasm_bindgen::from_value(submission)?;
    let encrypted_submission = encrypt_submission(submission, recipient_certs)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;

    let armor = armor_message(&encrypted_submission)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;

    Ok(stringify_armor(&armor))
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn encrypt_blob_js(
    blob: Vec<u8>,
    recipient_certs: Vec<String>,
) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    let encrypted = encrypt_anything(&blob, recipient_certs)
        .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    let armor =
        armor_message(&encrypted).map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
    Ok(armor)
}
