use crate::tsid::{ParseErrorReason, TsidError};
use crate::TSID;
use serde::{de::Error, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

impl Serialize for TSID {
    #[cfg(feature = "serde_as_string")]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }

    #[cfg(not(feature = "serde_as_string"))]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(self.to_string().as_str())
        } else {
            serializer.serialize_u64(self.number)
        }
    }
}

impl<'de> Deserialize<'de> for TSID {
    fn deserialize<D>(deserializer: D) -> Result<TSID, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TSIDVisitor)
    }
}

struct TSIDVisitor;

impl<'de> Visitor<'de> for TSIDVisitor {
    type Value = TSID;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an integer or string representation of TSID")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(TSID::new(v as u64))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(TSID::new(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let res = TSID::try_from(v);
        Self::convert_error(v, res)
    }
}

impl TSIDVisitor {
    fn convert_error<E>(v: &str, res: Result<TSID, TsidError>) -> Result<TSID, E>
    where
        E: Error,
    {
        match res {
            Ok(parsed) => Ok(parsed),
            Err(e) => match e {
                TsidError::ParseError(ParseErrorReason::InvalidLength) => {
                    Err(Error::invalid_length(v.len(), &"13"))
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TSID;

    #[test]
    #[cfg(feature = "serde")]
    fn serialize_to_human_readable_form() {
        let id1 = TSID::new(496830748901259172);
        println!(
            "{}",
            serde_json::to_string_pretty(&id1).expect("Unable to serialize")
        )
    }
}
