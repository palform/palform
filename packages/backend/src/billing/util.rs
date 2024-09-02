use chrono::{DateTime, Utc};

use super::error::BillingError;

pub fn stripe_timestamp_to_chrono(
    timestamp: Option<i64>,
    field_name: &str,
) -> Result<DateTime<Utc>, BillingError> {
    DateTime::<Utc>::from_timestamp(
        timestamp.ok_or(BillingError::FieldNone(field_name.to_string()))?,
        0,
    )
    .ok_or(BillingError::FieldNone(format!(
        "{} is invalid date",
        field_name
    )))
}
