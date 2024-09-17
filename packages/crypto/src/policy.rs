use sequoia_openpgp::{
    cert::CipherSuite,
    policy::StandardPolicy,
};

pub const NEW_KEY_SUITE: CipherSuite = CipherSuite::Cv25519;

pub fn recipient_cert_policy<'a>() -> StandardPolicy<'a> {
    StandardPolicy::new()
}
