use std::str::Chars;
use anyhow::anyhow;

use chrono::TimeDelta;

use crate::property::*;

/// RFC 5545 3.3.6 Duration
/// Syntax: ["+" / "-"] "P" (date / time / week)
///  date = day "D" time
///  time = "T" [hours "H"] [minutes "M"] [seconds "S"]
///  week = weeks "W"
/// Examples:
///  "P15DT5H0M20S" = 15 days, 5 hours, 20 seconds
///  "P7W" = 7 weeks
///  "-P1D" = Negative 1 day
pub type ICalDuration = TimeDelta;

impl ICalPropValueTrait for ICalDuration {
    /// The RFC is strict on either being date (day + time), time, or week
    /// but this system is more relaxed
    /// This also does not require time to include a T
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        // println!("parsing duration {}", value);
        let mut chars = value.chars();
        let inverted = find_sign(&mut chars)?;
        let duration = parse_duration(&mut chars)?;
        Ok(if inverted { -duration } else { duration })
    }

    fn serialize(&self) -> String {
        let is_neg = self.le(&TimeDelta::zero());
        let prefix = if is_neg { "-P" } else { "P" };
        let dur_abs = self.abs();
        let mut str = prefix.to_string();

        let weeks = dur_abs.num_weeks();
        let total_days = dur_abs.num_days();
        let days = total_days - (weeks * 7);
        let hours = dur_abs.num_hours() - (total_days * 24);
        let minutes = dur_abs.num_minutes() - (dur_abs.num_hours() * 60);
        let seconds = dur_abs.num_seconds() - (dur_abs.num_minutes() * 60);
        let has_time = hours > 0 || minutes > 0 || seconds > 0;

        //dur-weeks
        if !has_time && days == 0 { //has exact number of weeks
            push_comp(&mut str, weeks, "W");
        }
        else {
            if total_days > 0 {
                push_comp(&mut str, total_days, "D");
            }
            if has_time {
                serialize_time(&mut str, hours, minutes, seconds);
            }
        }

        str
    }
}

fn serialize_time(str: &mut String, hours: i64, minutes: i64, seconds: i64) {
    *str += "T";
    if seconds > 0 {
        push_comp(str, hours, "H");
        push_comp(str, minutes, "M");
        push_comp(str, seconds, "S");
    }
    else if minutes > 0 {
        push_comp(str, hours, "H");
        push_comp(str, minutes, "M");
    }
    else if hours > 0 {
        push_comp(str, hours, "H");
    }
}

fn push_comp(str: &mut String, num: i64, typ: &str) {
    *str += num.to_string().as_str();
    *str += typ;
}

//TODO switch to split_inclusive
fn parse_duration(chars: &mut Chars<'_>) -> anyhow::Result<TimeDelta> {
    let mut comps = [0; 5];
    let mut num_buffer = String::new();
    for char in chars {
        match char {
            'T' => continue,
            '0'..='9' => num_buffer.push(char),
            'W' => comps[0] = num_buffer.parse()?,
            'D' => comps[1] = num_buffer.parse()?,
            'H' => comps[2] = num_buffer.parse()?,
            'M' => comps[3] = num_buffer.parse()?,
            'S' => comps[4] = num_buffer.parse()?,
            _ => return Err(anyhow!("Unexpected character {} in duration string", char)),
        }
        if !char.is_numeric() {
            num_buffer.clear();
        }
    }
    Ok(TimeDelta::weeks(comps[0]) + TimeDelta::days(comps[1]) +
       TimeDelta::hours(comps[2]) + TimeDelta::minutes(comps[3]) +
       TimeDelta::seconds(comps[4]))
}

fn find_sign(chars: &mut Chars<'_>) -> anyhow::Result<bool> {
    let sign = match chars.next() {
        Some('-') => true,
        Some('+') => false,
        Some('P') => return Ok(false),
        Some(c) => return Err(anyhow!("Duration string should start with P but found {}", c)),
        None => return Err(anyhow!("Empty duration String")),
    };
    match chars.next() {
        Some('P') => Ok(sign),
        _ => Err(anyhow!("Duration string too short")),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::property::*;
    use crate::values::duration::*;

    #[test]
    fn test_duration_date() {
        assert_duration("P15DT5H0M20S", 2, 1, 5, 0, 20);
    }

    #[test]
    fn test_duration_weeks() {
        assert_duration("P7W", 7, 0, 0, 0, 0);
    }

    #[test]
    fn test_negative_duration() {
        assert_duration("-P1D", 0, -1, 0, 0, 0);
    }

    fn assert_duration(value: &str, expected_weeks: i64, expected_days: i64, expected_hours: i64, expected_minutes: i64, expected_seconds: i64) {
        let dur = ICalDuration::parse(value, &HashMap::new()).expect("Failed to parse!");

        let weeks = dur.num_weeks();
        let days = dur.num_days() - (weeks * 7);
        let hours = dur.num_hours() - (dur.num_days() * 24);
        let minutes = dur.num_minutes() - (dur.num_hours() * 60);
        let seconds = dur.num_seconds() - (dur.num_minutes() * 60);

        assert_eq!(weeks, expected_weeks, "Weeks wrong");
        assert_eq!(days, expected_days, "Days wrong");
        assert_eq!(hours, expected_hours, "Hours wrong");
        assert_eq!(minutes, expected_minutes, "Minutes wrong");
        assert_eq!(seconds, expected_seconds, "Seconds wrong");

        let s = ICalPropValueTrait::serialize(&dur);
        assert_eq!(s, value, "Serialization wrong");
    }
}
