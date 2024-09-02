use anyhow::anyhow;
use sequoia_openpgp::{
    crypto::KeyPair,
    packet::{PKESK, SKESK},
    parse::stream::{DecryptionHelper, VerificationHelper},
    types::SymmetricAlgorithm,
    Cert, Fingerprint, KeyID,
};

use crate::{
    keys::{encryption_key::resolve_encryption_key, parse::secret_key_from_pem},
    policy::recipient_cert_policy,
};

#[derive(Clone)]
#[cfg_attr(feature = "frontend-js", wasm_bindgen::prelude::wasm_bindgen)]
pub struct KeyResolver {
    keys: Vec<(KeyPair, Fingerprint)>,
}

#[cfg(feature = "frontend-js")]
#[wasm_bindgen::prelude::wasm_bindgen]
impl KeyResolver {
    #[wasm_bindgen::prelude::wasm_bindgen(constructor)]
    pub fn new(key_pems: Vec<String>) -> Result<KeyResolver, wasm_bindgen::JsValue> {
        let keys = parse_key_pems(key_pems)
            .map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))?;
        Ok(Self { keys })
    }
}

impl KeyResolver {
    fn find_key_for_id(&self, key_id: &KeyID) -> Option<(&KeyPair, &Fingerprint)> {
        for (kp, fp) in &self.keys {
            if &KeyID::from(fp) == key_id {
                return Some((kp, fp));
            }
        }
        None
    }
}

impl VerificationHelper for KeyResolver {
    fn get_certs(
        &mut self,
        _ids: &[sequoia_openpgp::KeyHandle],
    ) -> sequoia_openpgp::Result<Vec<Cert>> {
        Ok(Vec::new())
    }

    fn check(
        &mut self,
        _: sequoia_openpgp::parse::stream::MessageStructure,
    ) -> sequoia_openpgp::Result<()> {
        Ok(())
    }
}
impl DecryptionHelper for KeyResolver {
    fn decrypt<D>(
        &mut self,
        pkesks: &[PKESK],
        _skesks: &[SKESK],
        sym_algo: Option<SymmetricAlgorithm>,
        mut decrypt: D,
    ) -> sequoia_openpgp::Result<Option<Fingerprint>>
    where
        D: FnMut(
            sequoia_openpgp::types::SymmetricAlgorithm,
            &sequoia_openpgp::crypto::SessionKey,
        ) -> bool,
    {
        for pkesk in pkesks {
            let matching_key = self.find_key_for_id(pkesk.recipient());
            if let Some((kp, fp)) = matching_key {
                if pkesk
                    .decrypt(&mut kp.clone(), sym_algo)
                    .map(|(algo, sk)| decrypt(algo, &sk))
                    .unwrap_or(false)
                {
                    return Ok(Some(fp.clone()));
                }
            }
        }

        Err(anyhow!("Cannot decrypt message with key"))
    }
}

fn parse_key_pems(key_pems: Vec<String>) -> Result<Vec<(KeyPair, Fingerprint)>, anyhow::Error> {
    let mut key_pairs = Vec::<(KeyPair, Fingerprint)>::new();
    let p = recipient_cert_policy();

    for key_pem in key_pems {
        let cert = secret_key_from_pem(key_pem)?;
        let enc_key = resolve_encryption_key(&cert, &p)?;
        key_pairs.push((
            enc_key.key().parts_as_secret()?.clone().into_keypair()?,
            enc_key.fingerprint(),
        ))
    }

    Ok(key_pairs)
}
