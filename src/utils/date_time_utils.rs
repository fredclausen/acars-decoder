use time::{OffsetDateTime, Time};

/// Expects a string in the format "HHMM"
pub(crate) fn utc_to_string(utc: String) -> String {
    let mut utc_date = OffsetDateTime::now_utc();
    let hours = &utc[0..2].parse::<u8>().unwrap();
    let minutes = &utc[2..4].parse::<u8>().unwrap();

    let time: Time = Time::from_hms(*hours, *minutes, 0).unwrap();

    utc_date = utc_date.replace_time(time);

    let format_description = time::format_description::parse(
        "[hour]:[minute]:[second] GMT-[offset_hour]:[offset_minute]",
    )
    .unwrap();
    // convert to local time zone

    utc_date.format(&format_description).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utc_to_string() {
        let utc = "1200".to_string();
        let result = utc_to_string(utc);
        assert_eq!(result, "2023-10-01 12:00:00 UTC");
    }
}
