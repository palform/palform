use sequoia_openpgp::{
    armor::{Kind, Reader, ReaderMode},
    parse::Parse,
    Cert,
};

fn cert_from_pem(pem: String, kind: Kind) -> Result<Cert, anyhow::Error> {
    let r = Reader::from_bytes(pem.as_bytes(), ReaderMode::Tolerant(Some(kind)));
    Cert::from_reader(r)
}

pub fn public_key_from_pem(cert_pem: String) -> Result<Cert, anyhow::Error> {
    cert_from_pem(cert_pem, Kind::PublicKey)
}

pub fn secret_key_from_pem(cert_pem: String) -> Result<Cert, anyhow::Error> {
    cert_from_pem(cert_pem, Kind::SecretKey)
}

pub fn secret_or_public_key_from_pem(cert_pem: String) -> Result<Cert, anyhow::Error> {
    let secret = secret_key_from_pem(cert_pem.clone());
    match secret {
        Ok(secret) => Ok(secret),
        Err(_) => public_key_from_pem(cert_pem),
    }
}
