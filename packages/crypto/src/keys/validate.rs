use anyhow::anyhow;
use sequoia_openpgp::Cert;

use crate::policy::recipient_cert_policy;

pub fn validate_cert(cert: &Cert) -> Result<(), anyhow::Error> {
    let policy = recipient_cert_policy();
    let valid_cert = cert.with_policy(&policy, None)?;

    let mut has_encryption_key = false;
    for key in valid_cert.keys() {
        if key.for_storage_encryption() {
            if key.key_expiration_time() == None {
                return Err(anyhow!("Encryption key must expire"));
            }

            has_encryption_key = true;
            break;
        }
    }

    if !has_encryption_key {
        return Err(anyhow!("Certificate has no encryption key!"));
    }

    Ok(())
}
