use crate::tsid::{ParseErrorReason, TsidError, REVERSE, TSID};

impl From<TSID> for u64 {
    fn from(val: TSID) -> Self {
        val.number
    }
}

impl From<u64> for TSID {
    fn from(value: u64) -> Self {
        TSID::new(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::tsid::TSID;

    #[test]
    fn should_convert_from_u64() {
        let val = 496830748901259230u64;
        let id = TSID::from(val);
        assert_eq!("0DS8RXW6W0DYY", id.to_string())
    }
}

impl TryFrom<&str> for TSID {
    type Error = TsidError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 13 {
            return Err(TsidError::ParseError(ParseErrorReason::InvalidLength));
        }
        let chars = value.as_bytes();

        let mut number = 0u64;
        number |= REVERSE[&chars[0x00]] << 60;
        number |= REVERSE[&chars[0x01]] << 55;
        number |= REVERSE[&chars[0x02]] << 50;
        number |= REVERSE[&chars[0x03]] << 45;
        number |= REVERSE[&chars[0x04]] << 40;
        number |= REVERSE[&chars[0x05]] << 35;
        number |= REVERSE[&chars[0x06]] << 30;
        number |= REVERSE[&chars[0x07]] << 25;
        number |= REVERSE[&chars[0x08]] << 20;
        number |= REVERSE[&chars[0x09]] << 15;
        number |= REVERSE[&chars[0x0a]] << 10;
        number |= REVERSE[&chars[0x0b]] << 5;
        number |= REVERSE[&chars[0x0c]];

        Ok(TSID::new(number))
    }
}
