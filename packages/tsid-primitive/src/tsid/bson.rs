use crate::TSID;
use bson::Bson;

impl From<TSID> for Bson {
    fn from(value: TSID) -> Self {
        #[cfg(not(feature = "bson_as_string"))]
        return Bson::Int64(value.number as i64);

        #[cfg(feature = "bson_as_string")]
        return Bson::String(value.to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::TSID;

    #[test]
    fn serialize_to_bson() {
        let id1 = TSID::new(496830748901259172);
        println!("{}", bson::doc! {"id": id1 })
    }
}
