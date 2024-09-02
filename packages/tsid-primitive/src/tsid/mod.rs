use phf::phf_map;

use crate::consts::RANDOM_MASK;

pub mod conversions;
pub mod display;

#[cfg(feature = "debug")]
pub mod debug;

#[cfg(feature = "bson")]
pub mod bson;

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(feature = "chrono")]
pub mod chrono;

#[derive(Hash, Eq, PartialEq, PartialOrd, Copy, Clone)]
pub struct TSID {
    number: u64,
}

const ALPHABET: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J',
    'K', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z',
];

static REVERSE: phf::Map<u8, u64> = phf_map! {
    b'0' => 0x00,
    b'1' => 0x01,
    b'2' => 0x02,
    b'3' => 0x03,
    b'4' => 0x04,
    b'5' => 0x05,
    b'6' => 0x06,
    b'7' => 0x07,
    b'8' => 0x08,
    b'9' => 0x09,
    b'a' => 0x0a,
    b'b' => 0x0b,
    b'c' => 0x0c,
    b'd' => 0x0d,
    b'e' => 0x0e,
    b'f' => 0x0f,
    b'g' => 0x10,
    b'h' => 0x11,
    b'j' => 0x12,
    b'k' => 0x13,
    b'm' => 0x14,
    b'n' => 0x15,
    b'p' => 0x16,
    b'q' => 0x17,
    b'r' => 0x18,
    b's' => 0x19,
    b't' => 0x1a,
    b'v' => 0x1b,
    b'w' => 0x1c,
    b'x' => 0x1d,
    b'y' => 0x1e,
    b'z' => 0x1f,
    b'o' => 0x00,
    b'i' => 0x01,
    b'l' => 0x01,
    b'A' => 0x0a,
    b'B' => 0x0b,
    b'C' => 0x0c,
    b'D' => 0x0d,
    b'E' => 0x0e,
    b'F' => 0x0f,
    b'G' => 0x10,
    b'H' => 0x11,
    b'J' => 0x12,
    b'K' => 0x13,
    b'M' => 0x14,
    b'N' => 0x15,
    b'P' => 0x16,
    b'Q' => 0x17,
    b'R' => 0x18,
    b'S' => 0x19,
    b'T' => 0x1a,
    b'V' => 0x1b,
    b'W' => 0x1c,
    b'X' => 0x1d,
    b'Y' => 0x1e,
    b'Z' => 0x1f,
    b'O' => 0x00,
    b'I' => 0x01,
    b'L' => 0x01,
};

impl TSID {
    pub(crate) fn new(number: u64) -> Self {
        Self { number }
    }

    /// Returns numeric representation of TSID
    pub fn number(&self) -> u64 {
        self.number
    }

    //Returns a random part (node bits and random bits)
    pub fn random_part(&self) -> u64 {
        self.number & RANDOM_MASK
    }
}

#[derive(Debug)]
pub enum ParseErrorReason {
    InvalidLength,
}

#[derive(Debug)]
pub enum TsidError {
    ParseError(ParseErrorReason),
}

#[cfg(test)]
mod tests {
    use crate::TSID;

    #[test]
    fn tsid_should_have_small_size() {
        assert_eq!(
            8,
            std::mem::size_of::<TSID>(),
            "TSID should have size of exactly 8 bytes"
        );
    }

    #[test]
    fn should_implement_ordering() {
        let id1 = TSID::new(0);
        let id2 = TSID::new(10);

        assert!(
            id1 < id2,
            "Id2:{} should be greater than Id1:{} because it was created later",
            id2.to_string(),
            id1
        );
    }

    #[test]
    #[cfg(feature = "bson")]
    fn serialize_to_bson() {
        let id1 = TSID::new(496830748901259172);
        println!("{}", bson::doc! {"id": id1 })
    }
}
