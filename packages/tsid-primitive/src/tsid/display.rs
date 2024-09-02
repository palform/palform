use crate::tsid::ALPHABET;
use crate::TSID;
use std::fmt::{Display, Formatter};

impl Display for TSID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut chars = String::with_capacity(13);

        chars.push(ALPHABET[((self.number >> 60) & 0b11111) as usize]);
        chars.push(ALPHABET[((self.number >> 55) & 0b11111) as usize]);
        chars.push(ALPHABET[((self.number >> 50) & 0b11111) as usize]);
        chars.push(ALPHABET[((self.number >> 45) & 0b11111) as usize]);
        chars.push(ALPHABET[((self.number >> 40) & 0b11111) as usize]);
        chars.push(ALPHABET[((self.number >> 35) & 0b11111) as usize]);
        chars.push(ALPHABET[((self.number >> 30) & 0b11111) as usize]);
        chars.push(ALPHABET[((self.number >> 25) & 0b11111) as usize]);
        chars.push(ALPHABET[((self.number >> 20) & 0b11111) as usize]);
        chars.push(ALPHABET[((self.number >> 15) & 0b11111) as usize]);
        chars.push(ALPHABET[((self.number >> 10) & 0b11111) as usize]);
        chars.push(ALPHABET[((self.number >> 5) & 0b11111) as usize]);
        chars.push(ALPHABET[(self.number & 0b11111) as usize]);

        return f.write_str(chars.as_str());
    }
}

#[cfg(test)]
mod tests {
    use crate::TSID;

    #[test]
    fn string_representations_should_be_also_ordered() {
        let id1 = TSID::new(9);
        let id2 = TSID::new(10);

        assert!(
            id1.to_string() < id2.to_string(),
            "Id2:{} should be greater than Id1:{} because it was created later",
            id2.to_string(),
            id1
        );
    }
}
