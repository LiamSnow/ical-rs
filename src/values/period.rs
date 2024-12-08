use std::fmt::Display;

use crate::property::*;
use super::{datetime::ICalDateTime, duration::ICalDuration};
use anyhow::{anyhow, Context};

#[derive(Clone)]
pub struct ICalPeriod {
    pub start: ICalDateTime,
    pub end_or_duration: EndOrDuration,
    ///calculated from start & end_or_duration, only here for reference/convinence
    pub end: ICalDateTime,
}

#[derive(Clone)]
pub enum EndOrDuration {
    End(ICalDateTime),
    Duration(ICalDuration)
}

impl ICalPropertyValueTrait for ICalPeriod {
    fn parse(value: &str, params: &ICalParameterMap) -> anyhow::Result<Self> {
        let parts: Vec<&str> = value.splitn(2, '/').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Period has extra part"))
        }
        let start = ICalDateTime::parse(parts[0], params)
            .context("Parsing period start")?;

        match ICalDateTime::parse(parts[1], params) {
            Ok(end) => Ok(ICalPeriod::new(start, EndOrDuration::End(end))),
            Err(_) => {
                let duration = ICalDuration::parse(parts[1], params)
                    .context("Period has invalid end and duration")?;
                Ok(ICalPeriod::new(start, EndOrDuration::Duration(duration)))
            },
        }
    }

    fn serialize(&self) -> String {
        let part1 = self.start.serialize();
        let part2 = match &self.end_or_duration {
            EndOrDuration::End(end) => end.serialize(),
            EndOrDuration::Duration(dur) => dur.serialize(),
        };
        format!("{part1}/{part2}")
    }
}

impl ICalPeriod {
    pub fn new(start: ICalDateTime, end_or_duration: EndOrDuration) -> Self {
        let end = match &end_or_duration {
            EndOrDuration::End(dt) => dt.clone(),
            EndOrDuration::Duration(dur) => {
                match start {
                    ICalDateTime::Local(dt) => ICalDateTime::Local(dt.clone() + dur.clone()),
                    ICalDateTime::Zoned(dt) => ICalDateTime::Zoned(dt.clone() + dur.clone()),
                }
            },
        };
        Self {
            start,
            end_or_duration,
            end,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use chrono::TimeZone;

    use chrono::DateTime;
    use chrono_tz::Tz;

    use crate::property::*;
    use crate::values::period::*;

    #[test]
    fn test_period_end() {
        // The period starting at 18:00:00 UTC, on January 1, 1997
        // and ending at 07:00:00 UTC on January 2, 1997 would be:
        let value = "19970101T180000Z/19970102T070000Z";
        let start = Tz::UTC.with_ymd_and_hms(1997, 1, 1, 18, 0, 0).unwrap();
        let end = Tz::UTC.with_ymd_and_hms(1997, 1, 2, 7, 0, 0).unwrap();
        assert_period(value, ICalDateTime::Zoned(start), ICalDateTime::Zoned(end));
    }

    #[test]
    fn test_period_duration() {
        // The period start at 18:00:00 on January 1, 1997
        // and lasting 2 hours and 30 minutes would be
        let value = "19970101T180000Z/PT2H30M";
        let start: DateTime<Tz> = Tz::UTC.with_ymd_and_hms(1997, 1, 1, 18, 0, 0).unwrap();
        let end: DateTime<Tz> = Tz::UTC.with_ymd_and_hms(1997, 1, 1, 20, 30, 0).unwrap();
        assert_period(value, ICalDateTime::Zoned(start), ICalDateTime::Zoned(end));
    }

    fn assert_period(value: &str, start: ICalDateTime, end: ICalDateTime) {
        let per = ICalPeriod::parse(value, &HashMap::new()).expect("Failed to parse!");
        assert_eq!(per.start, start);
        assert_eq!(per.end, end);
        let s = ICalPropertyValueTrait::serialize(&per);
        assert_eq!(s, value);
    }
}

impl Display for ICalPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self.end_or_duration {
            EndOrDuration::End(_) => "end",
            EndOrDuration::Duration(_) => "duration",
        };
        write!(f, "(start:{},end:{},type:{})(Period)", self.start, self.end, t)
    }
}
