use rand::RngExt;

pub fn create_hex_secret() -> String {
    let mut arr = [0u8; 64];
    rand::rng().fill(&mut arr);
    faster_hex::hex_string(&arr)
}
