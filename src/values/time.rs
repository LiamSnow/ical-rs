use std::str::FromStr;
use anyhow::anyhow;

use chrono::{NaiveTime, TimeZone};
use chrono_tz::Tz;

use crate::ical::objects::generics::ICalParameterMap;

use super::base::*;

/// RFC 5545 3.3.12
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ICalTime {
    pub time: NaiveTime,
    pub timezone: Option<Tz>
}

const FORMAT: &str = "%H%M%S";

impl ICalPropType for ICalTime {
    fn parse(value: &str, params: &ICalParameterMap) -> anyhow::Result<Self> {
        let is_utc = value.ends_with('Z');
        let value = if is_utc { value.trim_end_matches('Z') } else { value };
        let time = NaiveTime::parse_from_str(value, FORMAT)?;
        let timezone = match params.get("TZID") {
            Some(tz_str) => Tz::from_str(tz_str).ok(),
            None if is_utc => Some(Tz::UTC),
            _ => None,
        };
        Ok(Self {time, timezone})
    }

    fn serialize(&self) -> String {
        let suffix = if self.timezone == Some(Tz::UTC) { "Z" } else { "" };
        self.time.format(FORMAT).to_string() + suffix
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveTime;
    use std::collections::HashMap;

    use crate::ical::values::base::*;
    use crate::ical::values::time::*;

    #[test]
    fn test_datetime_local() {
        let value = "230000";
        let expected = NaiveTime::from_hms_opt(23, 0, 0).unwrap();
        assert_time(value, &HashMap::new(), expected, None);
    }

    #[test]
    fn test_datetime_utc() {
        let value = "183005Z";
        let expected = NaiveTime::from_hms_opt(18, 30, 5).unwrap();
        assert_time(value, &HashMap::new(), expected, Some(Tz::UTC));
    }

    #[test]
    fn test_datetime_tz() {
        let mut params = HashMap::new();
        params.insert("TZID".to_string(), "America/New_York".to_string());
        let value = "013010";
        let expected = NaiveTime::from_hms_opt(1, 30, 10).unwrap();
        assert_time(value, &params, expected, Some(Tz::America__New_York));
    }

    fn assert_time(value: &str, params: &HashMap<String, String>, expected_time: NaiveTime, expected_timezone: Option<Tz>) {
        let icaltime = ICalTime::parse(value, &params).expect("Failed to parse!");
        assert_eq!(icaltime.time, expected_time);
        assert_eq!(icaltime.timezone, expected_timezone);
        let s = ICalPropType::serialize(&icaltime);
        assert_eq!(s, value);
    }
}
