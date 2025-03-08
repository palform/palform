use sequoia_openpgp::{
    armor::{Kind, Reader, ReaderMode, Writer},
    packet::Packet,
    parse::Parse,
    serialize::SerializeInto,
    KeyHandle, Message,
};
use std::io::Write;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SubmissionConversionError {
    #[error("PGP: {0}")]
    #[allow(clippy::upper_case_acronyms)]
    PGP(String),
}

pub struct CryptoSubmissionRepr {
    message: Message,
}
impl CryptoSubmissionRepr {
    pub fn from_pem_string(pem_string: String) -> Result<Self, SubmissionConversionError> {
        let r = Reader::from_bytes(
            pem_string.as_bytes(),
            ReaderMode::Tolerant(Some(Kind::Message)),
        );
        let message =
            Message::from_reader(r).map_err(|e| SubmissionConversionError::PGP(e.to_string()))?;
        Ok(Self { message })
    }

    pub fn to_database_bytes(&self) -> Result<Vec<u8>, SubmissionConversionError> {
        self.message
            .to_vec()
            .map_err(|e| SubmissionConversionError::PGP(e.to_string()))
    }

    pub fn from_database_bytes(bytes: &[u8]) -> Result<Self, SubmissionConversionError> {
        let message = Message::from_bytes(bytes)
            .map_err(|e| SubmissionConversionError::PGP(e.to_string()))?;
        Ok(Self { message })
    }

    pub fn to_pem_string(bytes: &[u8]) -> Result<String, SubmissionConversionError> {
        let mut w = Writer::new(Vec::new(), Kind::Message)
            .map_err(|e| SubmissionConversionError::PGP(e.to_string()))?;
        w.write_all(bytes)
            .map_err(|e| SubmissionConversionError::PGP(e.to_string()))?;

        let pem_bytes = w
            .finalize()
            .map_err(|e| SubmissionConversionError::PGP(e.to_string()))?;
        let pem_string = String::from_utf8(pem_bytes).map_err(|e| {
            SubmissionConversionError::PGP(format!("Received non-utf8 string from Writer: {}", e))
        })?;

        Ok(pem_string)
    }

    pub fn get_decrypting_key_handles(&self) -> Vec<KeyHandle> {
        let mut handles = Vec::new();
        for child in self.message.packets().children() {
            if let Packet::PKESK(child) = child {
                if let Some(recipient) = child.recipient().clone() {
                    handles.push(recipient);
                }
            }
        }
        handles
    }
}
