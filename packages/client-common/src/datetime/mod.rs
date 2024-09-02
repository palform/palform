use anyhow::anyhow;
use chrono::{DateTime, Datelike, Local, Timelike};

pub fn normalise_date_time(
    value: DateTime<Local>,
    date: bool,
    time: bool,
) -> Result<DateTime<Local>, anyhow::Error> {
    let normalised_value = if date && time {
        value
    } else if time {
        value
            .with_day(1)
            .ok_or(anyhow!("idk"))?
            .with_month(1)
            .ok_or(anyhow!("idk"))?
            .with_year(1970)
            .ok_or(anyhow!("idk"))?
            .with_second(0)
            .ok_or(anyhow!("idk"))?
            .with_nanosecond(0)
            .ok_or(anyhow!("idk"))?
    } else if date {
        value
            .with_hour(0)
            .ok_or(anyhow!("idk"))?
            .with_minute(0)
            .ok_or(anyhow!("idk"))?
            .with_second(0)
            .ok_or(anyhow!("idk"))?
            .with_nanosecond(0)
            .ok_or(anyhow!("idk"))?
    } else {
        return Err(anyhow!(
            "Cannot normalise date where neither date nor time is requested"
        ));
    };

    Ok(normalised_value)
}
