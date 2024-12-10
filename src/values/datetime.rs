use std::str::FromStr;
use anyhow::anyhow;

use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

use crate::property::*;

/// RFC 5545 3.3.5
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ICalDateTime {
    ///FORM #1: Local Time
    Local(NaiveDateTime),
    ///FORM #2: UTC Time, FORM #3: Time zone
    Zoned(DateTime<Tz>),
}

const FORMAT: &str = "%Y%m%dT%H%M%S";

impl ICalPropertyValueTrait for ICalDateTime {
    fn parse(value: &str, params: &ICalParameterMap) -> anyhow::Result<Self> {
        let is_utc = value.ends_with('Z');
        let value = if is_utc { value.trim_end_matches('Z') } else { value };

        let local = NaiveDateTime::parse_from_str(value, FORMAT)?;

        if params.contains_key("TZID") {
            let timezone_str = params.get("TZID").unwrap();
            let timezone = Tz::from_str(timezone_str)?;
            let dt = timezone.from_local_datetime(&local).single().ok_or(anyhow!("Failed to transfer into timezone"))?;
            Ok(Self::Zoned(dt))
        } else if is_utc {
            let dt = Tz::UTC.from_local_datetime(&local).single().ok_or(anyhow!("Failed to tranfer into UTC"))?;
            Ok(Self::Zoned(dt))
        } else {
            Ok(Self::Local(local))
        }
    }

    fn serialize(&self) -> String {
        match self {
            ICalDateTime::Local(dt) => {
                dt.format(FORMAT).to_string()
            },
            ICalDateTime::Zoned(dt) => {
                let suffix = if dt.timezone() == Tz::UTC { "Z" } else { "" };
                dt.format(FORMAT).to_string() + suffix
            },
        }
    }
}

//TODO test
pub type ICalDateTimeList = Vec<ICalDateTime>;

impl ICalPropertyValueTrait for ICalDateTimeList {
    fn parse(values: &str, params: &ICalParameterMap) -> anyhow::Result<Self> {
        values.split(',').try_fold(Vec::new(), |mut acc, value| {
            acc.push(ICalDateTime::parse(value, params)?);
            Ok(acc)
        })
    }

    fn serialize(&self) -> String {
        self.iter().map(|d| d.serialize()).collect::<Vec<String>>().join(",")
    }
}

impl From<ICalDateTime> for ICalProperty {
    fn from(value: ICalDateTime) -> Self {
        Self::from_value(ICalPropertyValue::DateTime(value))
    }
}

impl From<NaiveDateTime> for ICalProperty {
    fn from(value: NaiveDateTime) -> Self {
        Self::from_value(ICalPropertyValue::DateTime(value.into()))
    }
}

impl From<DateTime<Tz>> for ICalProperty {
    fn from(value: DateTime<Tz>) -> Self {
        Self::from_value(ICalPropertyValue::DateTime(value.into()))
    }
}

impl From<NaiveDateTime> for ICalDateTime {
    fn from(value: NaiveDateTime) -> Self {
        Self::Local(value)
    }
}

impl From<DateTime<Tz>> for ICalDateTime {
    fn from(value: DateTime<Tz>) -> Self {
        Self::Zoned(value)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chrono::NaiveDate;
    use chrono::NaiveTime;

    use crate::property::*;
    use crate::values::datetime::*;

    #[test]
    fn test_datetime_local() {
        let value = "20140517T123456";
        let expected = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2014, 5, 17).unwrap(),
            NaiveTime::from_hms_opt(12, 34, 56).unwrap()
        );
        assert_datetime(value, &HashMap::new(), ICalDateTime::Local(expected));
    }

    #[test]
    fn test_datetime_utc() {
        let value = "20140517T123456Z";
        let expected = Tz::UTC.with_ymd_and_hms(2014, 5, 17, 12, 34, 56).unwrap();
        assert_datetime(value, &HashMap::new(), ICalDateTime::Zoned(expected));
    }

    #[test]
    fn test_datetime_tz() {
        let value = "19921217T123456";
        let mut params = HashMap::new();
        params.insert("TZID".to_string(), "America/New_York".to_string());
        let expected = Tz::America__New_York.with_ymd_and_hms(1992, 12, 17, 12, 34, 56).unwrap();
        assert_datetime(value, &params, ICalDateTime::Zoned(expected));
    }

    fn assert_datetime(value: &str, params: &HashMap<String, String>, expected: ICalDateTime) {
        let result = ICalDateTime::parse(value, &params).expect("Failed to parse!");
        assert_eq!(result, expected);
        let s = ICalPropertyValueTrait::serialize(&result);
        assert_eq!(s, value);
    }
}
