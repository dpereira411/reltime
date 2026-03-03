use chrono::{DateTime, Local, LocalResult, NaiveDate, NaiveDateTime, TimeZone, Utc};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidFormat,
    AmbiguousLocalTime,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidFormat => {
                write!(f, "invalid timestamp format")
            }
            ParseError::AmbiguousLocalTime => {
                write!(f, "timestamp maps to an ambiguous or invalid local time")
            }
        }
    }
}

impl Error for ParseError {}

pub fn parse_timestamp(input: &str) -> Result<DateTime<Utc>, ParseError> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(input) {
        return Ok(dt.with_timezone(&Utc));
    }

    for fmt in [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M",
    ] {
        if let Ok(naive) = NaiveDateTime::parse_from_str(input, fmt) {
            return local_naive_to_utc(naive);
        }
    }

    if let Ok(date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        if let Some(naive) = date.and_hms_opt(0, 0, 0) {
            return local_naive_to_utc(naive);
        }
    }

    Err(ParseError::InvalidFormat)
}

fn local_naive_to_utc(naive: NaiveDateTime) -> Result<DateTime<Utc>, ParseError> {
    match Local.from_local_datetime(&naive) {
        LocalResult::Single(local_dt) => Ok(local_dt.with_timezone(&Utc)),
        LocalResult::Ambiguous(_, _) | LocalResult::None => Err(ParseError::AmbiguousLocalTime),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn parses_rfc3339_z() {
        let parsed = parse_timestamp("2025-10-01T12:00:00Z").expect("should parse");
        assert_eq!(parsed.to_rfc3339(), "2025-10-01T12:00:00+00:00");
    }

    #[test]
    fn parses_rfc3339_with_offset() {
        let parsed = parse_timestamp("2025-10-01T12:00:00+02:00").expect("should parse");
        assert_eq!(parsed.to_rfc3339(), "2025-10-01T10:00:00+00:00");
    }

    #[test]
    fn parses_space_separated_datetime() {
        let parsed = parse_timestamp("2025-10-01 12:34:56").expect("should parse");
        assert_eq!(parsed.second(), 56);
    }

    #[test]
    fn parses_date_only_as_midnight_local() {
        let parsed = parse_timestamp("2025-10-01").expect("should parse");
        let local = parsed.with_timezone(&Local);
        assert_eq!(local.hour(), 0);
        assert_eq!(local.minute(), 0);
        assert_eq!(local.second(), 0);
    }

    #[test]
    fn rejects_invalid_text() {
        let err = parse_timestamp("not-a-date").expect_err("should fail");
        assert_eq!(err, ParseError::InvalidFormat);
    }

    #[test]
    fn rejects_impossible_date() {
        let err = parse_timestamp("2025-13-01").expect_err("should fail");
        assert_eq!(err, ParseError::InvalidFormat);
    }
}
