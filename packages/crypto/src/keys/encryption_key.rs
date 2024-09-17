use anyhow::anyhow;
use sequoia_openpgp::{
    cert::amalgamation::key::ValidKeyAmalgamation,
    packet::key::{PublicParts, UnspecifiedRole},
    policy::StandardPolicy,
    Cert,
};

pub fn resolve_encryption_key<'a>(
    cert: &'a Cert,
    policy: &'a StandardPolicy,
) -> Result<ValidKeyAmalgamation<'a, PublicParts, UnspecifiedRole, bool>, anyhow::Error> {
    let valid_cert = cert.with_policy(policy, None)?;
    let enc_key = valid_cert
        .keys()
        .find(|e| e.for_storage_encryption())
        .ok_or(anyhow!("No encryption key found"))?;
    Ok(enc_key)
}
