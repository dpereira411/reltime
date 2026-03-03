use chrono::{DateTime, Utc};

const MINUTE: i64 = 60;
const HOUR: i64 = 60 * MINUTE;
const DAY: i64 = 24 * HOUR;
const MONTH: i64 = 30 * DAY;
const YEAR: i64 = 365 * DAY;
const MAX_EXACT_PARTS: usize = 3;

pub fn format_relative(target: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let delta = target.signed_duration_since(now).num_seconds();
    let is_future = delta > 0;
    let abs_secs = delta.unsigned_abs() as i64;

    let (qty, unit) = if abs_secs < MINUTE {
        (abs_secs.max(1), unit_label(abs_secs.max(1), "sec", "secs"))
    } else if abs_secs < HOUR {
        let qty = (abs_secs / MINUTE).max(1);
        (qty, unit_label(qty, "min", "mins"))
    } else if abs_secs < DAY {
        let qty = (abs_secs / HOUR).max(1);
        (qty, unit_label(qty, "hr", "hrs"))
    } else if abs_secs < MONTH {
        let qty = (abs_secs / DAY).max(1);
        (qty, unit_label(qty, "day", "days"))
    } else if abs_secs < YEAR {
        let qty = (abs_secs / MONTH).max(1);
        (qty, unit_label(qty, "month", "months"))
    } else {
        let qty = (abs_secs / YEAR).max(1);
        (qty, unit_label(qty, "year", "years"))
    };

    if is_future {
        format!("in {qty} {unit}")
    } else {
        format!("{qty} {unit} ago")
    }
}

pub fn format_relative_exact(target: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let delta = target.signed_duration_since(now).num_seconds();
    let is_future = delta > 0;
    let abs_secs = delta.unsigned_abs() as i64;

    if abs_secs == 0 {
        return "0 secs ago".to_string();
    }

    let parts = exact_parts(abs_secs)
        .into_iter()
        .take(MAX_EXACT_PARTS)
        .map(|(qty, singular, plural)| format!("{qty} {}", unit_label(qty, singular, plural)))
        .collect::<Vec<_>>()
        .join(" ");

    if is_future {
        format!("in {parts}")
    } else {
        format!("{parts} ago")
    }
}

fn exact_parts(mut abs_secs: i64) -> Vec<(i64, &'static str, &'static str)> {
    let units = [
        (YEAR, "year", "years"),
        (MONTH, "month", "months"),
        (DAY, "day", "days"),
        (HOUR, "hr", "hrs"),
        (MINUTE, "min", "mins"),
        (1, "sec", "secs"),
    ];

    let mut parts = Vec::new();

    for (unit_size, singular, plural) in units {
        let qty = abs_secs / unit_size;
        if qty > 0 {
            parts.push((qty, singular, plural));
            abs_secs %= unit_size;
        }
    }

    parts
}

fn unit_label<'a>(qty: i64, singular: &'a str, plural: &'a str) -> &'a str {
    if qty == 1 {
        singular
    } else {
        plural
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone};

    #[test]
    fn formats_past_minutes() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let target = now - Duration::minutes(5);
        assert_eq!(format_relative(target, now), "5 mins ago");
    }

    #[test]
    fn formats_singular_minute() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let target = now - Duration::minutes(1);
        assert_eq!(format_relative(target, now), "1 min ago");
    }

    #[test]
    fn formats_future_years() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let target = now + Duration::days(365 * 4);
        assert_eq!(format_relative(target, now), "in 4 years");
    }

    #[test]
    fn formats_past_months() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let target = now - Duration::days(30 * 7);
        assert_eq!(format_relative(target, now), "7 months ago");
    }

    #[test]
    fn formats_seconds() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let target = now - Duration::seconds(59);
        assert_eq!(format_relative(target, now), "59 secs ago");
    }

    #[test]
    fn boundary_sixty_seconds_to_minute() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let target = now - Duration::seconds(60);
        assert_eq!(format_relative(target, now), "1 min ago");
    }

    #[test]
    fn boundary_hour() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let target = now - Duration::seconds(3600);
        assert_eq!(format_relative(target, now), "1 hr ago");
    }

    #[test]
    fn boundary_month() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let target = now - Duration::days(30);
        assert_eq!(format_relative(target, now), "1 month ago");
    }

    #[test]
    fn boundary_year() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let target = now - Duration::days(365);
        assert_eq!(format_relative(target, now), "1 year ago");
    }

    #[test]
    fn exact_formats_past_three_parts() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let target = now - Duration::days((30 * 5) + 3) - Duration::hours(2);
        assert_eq!(format_relative_exact(target, now), "5 months 3 days 2 hrs ago");
    }

    #[test]
    fn exact_formats_future_three_parts() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let target = now + Duration::days(365 + 30 + 1);
        assert_eq!(format_relative_exact(target, now), "in 1 year 1 month 1 day");
    }

    #[test]
    fn exact_truncates_to_three_non_zero_parts() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let target =
            now + Duration::days(365 + 30 + 1) + Duration::hours(2) + Duration::minutes(3);
        assert_eq!(format_relative_exact(target, now), "in 1 year 1 month 1 day");
    }

    #[test]
    fn exact_formats_seconds() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let target = now - Duration::seconds(59);
        assert_eq!(format_relative_exact(target, now), "59 secs ago");
    }

    #[test]
    fn exact_formats_zero_delta() {
        let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(format_relative_exact(now, now), "0 secs ago");
    }
}
