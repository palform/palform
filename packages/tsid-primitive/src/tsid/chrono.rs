use crate::consts::RANDOM_BITS;
use crate::TSID;
use chrono::{DateTime, Duration, TimeZone, Utc};

impl TSID {
    /// Returns a chrono::DateTime<Utc>
    pub fn timestamp(&self) -> DateTime<Utc> {
        let dt = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        dt + Duration::milliseconds((self.number >> RANDOM_BITS) as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::TSID;
    use chrono::{TimeZone, Utc};

    #[test]
    fn should_set_timestamp_to_tsid_epoch() {
        let id_min = TSID::new(0);
        assert_eq!(
            id_min.timestamp(),
            Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap()
        )
    }

    #[test]
    fn should_set_timestamp_to_max_timestamp_value() {
        let id_max = TSID::new(u64::MAX);
        assert_eq!(
            id_max.timestamp(),
            chrono::DateTime::parse_from_rfc3339("2159-05-15T07:35:11.103Z").unwrap()
        )
    }
}
