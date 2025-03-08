use anyhow::anyhow;
use sequoia_openpgp::{
    crypto::{KeyPair, SessionKey},
    packet::{PKESK, SKESK},
    parse::stream::{DecryptionHelper, VerificationHelper},
    types::SymmetricAlgorithm,
    Cert, Fingerprint, KeyHandle,
};

use crate::{
    keys::{encryption_key::resolve_encryption_key, parse::secret_key_from_pem},
    policy::recipient_cert_policy,
};

#[derive(Clone)]
#[cfg_attr(feature = "frontend-js", wasm_bindgen::prelude::wasm_bindgen)]
pub struct KeyResolver {
    keys: Vec<(KeyPair, Fingerprint, Cert)>,
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
    fn find_key_for_handle(
        &self,
        key_handle: &KeyHandle,
    ) -> Option<(&KeyPair, &Fingerprint, &Cert)> {
        for (kp, this_fingerprint, cert) in &self.keys {
            if KeyHandle::from(this_fingerprint).aliases(key_handle) {
                return Some((kp, this_fingerprint, cert));
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
    fn decrypt(
        &mut self,
        pkesks: &[PKESK],
        _skesks: &[SKESK],
        sym_algo: Option<SymmetricAlgorithm>,
        decrypt: &mut dyn FnMut(Option<SymmetricAlgorithm>, &SessionKey) -> bool,
    ) -> sequoia_openpgp::Result<Option<Cert>> {
        for pkesk in pkesks {
            let matching_key = self.find_key_for_handle(
                &pkesk
                    .recipient()
                    .ok_or(anyhow!("Session key had no recipient metadata"))?,
            );

            if let Some((kp, _, cert)) = matching_key {
                if pkesk
                    .decrypt(&mut kp.clone(), sym_algo)
                    .map(|(algo, sk)| decrypt(algo, &sk))
                    .unwrap_or(false)
                {
                    return Ok(Some(cert.to_owned()));
                }
            }
        }

        Err(anyhow!("None of your keys are able to decrypt this data"))
    }
}

fn parse_key_pems(
    key_pems: Vec<String>,
) -> Result<Vec<(KeyPair, Fingerprint, Cert)>, anyhow::Error> {
    let mut key_pairs = Vec::new();
    let p = recipient_cert_policy();

    for key_pem in key_pems {
        let cert = secret_key_from_pem(key_pem)?;
        let enc_key = resolve_encryption_key(&cert, &p)?;
        key_pairs.push((
            enc_key.key().parts_as_secret()?.clone().into_keypair()?,
            enc_key.key().fingerprint(),
            cert,
        ))
    }

    Ok(key_pairs)
}
