use anyhow::anyhow;

use chrono::TimeDelta;

use crate::property::ICalParameterMap;
use super::ICalValueTrait;

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

impl ICalValueTrait for ICalDuration {
    /// The RFC is strict on either being date (day + time), time, or week
    /// but this system is more relaxed
    /// This also does not require time to include a T
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        // println!("parsing duration {}", value);
        let (inverted, rest) = pop_sign(value);
        let duration = parse_duration(rest)?;
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
            push_comp(&mut str, weeks, 'W');
        }
        else {
            if total_days > 0 {
                push_comp(&mut str, total_days, 'D');
            }
            if has_time {
                serialize_time(&mut str, hours, minutes, seconds);
            }
        }

        str
    }
}

fn serialize_time(str: &mut String, hours: i64, minutes: i64, seconds: i64) {
    str.push('T');
    if seconds > 0 {
        push_comp(str, hours, 'H');
        push_comp(str, minutes, 'M');
        push_comp(str, seconds, 'S');
    }
    else if minutes > 0 {
        push_comp(str, hours, 'H');
        push_comp(str, minutes, 'M');
    }
    else if hours > 0 {
        push_comp(str, hours, 'H');
    }
}

fn push_comp(str: &mut String, num: i64, typ: char) {
    str.push_str(num.to_string().as_str());
    str.push(typ);
}

fn parse_duration(v: &str) -> anyhow::Result<TimeDelta> {
    let mut parts = v.split_inclusive(char::is_uppercase);
    let first_part = parts.next().ok_or(anyhow!("Duration is empty"))?;
    if first_part != "P" {
        return Err(anyhow!("Duration string missing P start char"))
    }
    let comps = parts.into_iter()
        .filter(|&p| p != "T")
        .try_fold([0; 5], |mut acc, part| -> anyhow::Result<[i64; 5]> {
            let (num, label) = part.split_at(part.len() - 1);
            let idx = "WDHMS".find(label)
                .ok_or(anyhow!("Unexpected label {} in duration string", label))?;
            acc[idx] = num.parse()?;
            Ok(acc)
        })?;
    Ok(TimeDelta::weeks(comps[0]) + TimeDelta::days(comps[1]) +
       TimeDelta::hours(comps[2]) + TimeDelta::minutes(comps[3]) +
       TimeDelta::seconds(comps[4]))
}

fn pop_sign(s: &str) -> (bool, &str) {
    if let Some(rest) = s.strip_prefix('-') {
        return (true, rest)
    }
    else if let Some(rest) = s.strip_prefix('+') {
        return (false, rest)
    }
    (false, s)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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
    fn test_duration_negative() {
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

        let s = ICalValueTrait::serialize(&dur);
        assert_eq!(s, value, "Serialization wrong");
    }
}
