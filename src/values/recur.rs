use std::str::FromStr;

use crate::property::*;
use super::{date::ICalDate, datetime::ICalDateTime};
use anyhow::{anyhow, Context};

///RFC 5545 3.3.10 Recurrence Rule = rule ** ;
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct ICalRecur {
    pub freq: Frequency,
    pub until: Option<DateOrDateTime>,
    pub count: Option<u8>,
    pub interval: Option<u8>,
    ///0-60
    pub bysecond: Vec<u8>,
    ///0-59
    pub byminute: Vec<u8>,
    ///0-23
    pub byhour: Vec<u8>,
    /// ([[+/-] 1-53] weekday) ** ,
    pub byday: Vec<ByDay>,
    ///+/- 1-31
    pub bymonthday: Vec<i8>,
    ///+/- 1-366
    pub byyearday: Vec<i16>,
    ///+/- 1-53
    pub byweekno: Vec<i8>,
    ///1-12
    pub bymonth: Vec<u8>,
    ///+/- 1-366
    pub bysetpos: Vec<i16>,
    pub wkst: Option<Weekday>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Frequency {
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

/// Each BDAY can be preceded by a integer, which indicates nth occurance of a specific day within
/// the MONTHLY or YEARLY "RRULE" For example, within a MONTHLY rule, +1MO (or 1MO) represents the
/// first Monday in the month and -1MO represents the last
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ByDay {
    pub ordwk: Option<i8>,
    pub weekday: Weekday
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DateOrDateTime {
    Date(ICalDate),
    DateTime(ICalDateTime)
}

impl ICalPropertyValueTrait for ICalRecur {
    fn parse(value: &str, params: &ICalParameterMap) -> anyhow::Result<Self> {
        let mut rules = value.split(';');

        //TODO FIXME FREQ may not be first rule, but it is required
        let (name, val) = parse_rule(rules.next().ok_or(anyhow!("Recur has no rules"))?)?;
        if name != "FREQ" {
            return Err(anyhow!("First rule in RECUR must be FREQ"));
        }
        let freq = Frequency::from_str(val)?;

        let mut recur = ICalRecur { freq, ..Default::default() };

        for rule in rules {
            let (name, val) = parse_rule(rule)?;
            match name {
                "UNTIL" => recur.until = Some(DateOrDateTime::from_str(val, params)?),
                "COUNT" => recur.count = Some(val.parse()?),
                "INTERVAL" => recur.interval = Some(val.parse()?),
                "WKST" => recur.wkst = Some(Weekday::from_str(val)?),
                "BYDAY" => for part in val.split(',') {
                    recur.byday.push(part.parse()?)
                },
                "BYSECOND" => extend_parse_vec(&mut recur.bysecond, val)?,
                "BYMINUTE" => extend_parse_vec(&mut recur.byminute, val)?,
                "BYHOUR" => extend_parse_vec(&mut recur.byhour, val)?,
                "BYMONTHDAY" => extend_parse_vec(&mut recur.bymonthday, val)?,
                "BYYEARDAY" => extend_parse_vec(&mut recur.byyearday, val)?,
                "BYWEEKNO" => extend_parse_vec(&mut recur.byweekno, val)?,
                "BYMONTH" => extend_parse_vec(&mut recur.bymonth, val)?,
                "BYSETPOS" => extend_parse_vec(&mut recur.bysetpos, val)?,
                _ => return Err(anyhow!("Unknown rule in RECUR"))
            }
        }

        recur.validate()?;
        Ok(recur)
    }

    fn serialize(&self) -> String {
        let mut s = "FREQ=".to_string();
        s.push_str(&self.freq.to_string());
        s.push(';');
        serialize_opt(&mut s, "UNTIL", &self.until);
        serialize_opt(&mut s, "COUNT", &self.count);
        serialize_opt(&mut s, "INTERVAL", &self.interval);
        serialize_opt(&mut s, "WKST", &self.wkst);
        serialize_vec(&mut s, "BYDAY", &self.byday);
        serialize_vec(&mut s, "BYSECOND", &self.bysecond);
        serialize_vec(&mut s, "BYMINUTE", &self.byminute);
        serialize_vec(&mut s, "BYHOUR", &self.byhour);
        serialize_vec(&mut s, "BYMONTHDAY", &self.bymonthday);
        serialize_vec(&mut s, "BYYEARDAY", &self.byyearday);
        serialize_vec(&mut s, "BYWEEKNO", &self.byweekno);
        serialize_vec(&mut s, "BYMONTH", &self.bymonth);
        serialize_vec(&mut s, "BYSETPOS", &self.bysetpos);
        s.pop();
        s
    }
}

impl ICalRecur {
    pub fn validate(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

fn serialize_opt<T: ToString>(s: &mut String, name: &str, r: &Option<T>) {
    if let Some(val) = r {
        s.push_str(name);
        s.push('=');
        s.push_str(&val.to_string());
        s.push(';');
    }
}

fn serialize_vec<T: ToString>(s: &mut String, name: &str, r: &Vec<T>) {
    if r.len() > 0 {
        s.push_str(name);
        s.push('=');
        for (i, num) in r.iter().enumerate() {
            if i != 0 {
                s.push(',');
            }
            s.push_str(&num.to_string())
        }
        s.push(';');
    }
}

fn extend_parse_vec<T>(r: &mut Vec<T>, val: &str) -> anyhow::Result<()>
where
    T: FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    for part in val.split(',') {
        r.push(part.parse()?);
    }
    Ok(())
}


fn parse_rule(rule: &str) -> anyhow::Result<(&str, &str)> {
    let parts: Vec<&str> = rule.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(anyhow!("Recur rule has multiple ="));
    }
    Ok((parts[0], parts[1]))
}

impl FromStr for Frequency {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SECONDLY" => Ok(Self::Secondly),
            "MINUTELY" => Ok(Self::Minutely),
            "HOURLY" => Ok(Self::Hourly),
            "DAILY" => Ok(Self::Daily),
            "WEEKLY" => Ok(Self::Weekly),
            "MONTHLY" => Ok(Self::Monthly),
            "YEARLY" => Ok(Self::Yearly),
            _ => Err(anyhow!("Invalid frequency")) //TODO
        }
    }
}

impl ToString for Frequency {
    fn to_string(&self) -> String {
        match &self {
            Self::Secondly => "SECONDLY",
            Self::Minutely => "MINUTELY",
            Self::Hourly => "HOURLY",
            Self::Daily => "DAILY",
            Self::Weekly => "WEEKLY",
            Self::Monthly => "MONTHLY",
            Self::Yearly => "YEARLY",
        }.to_string()
    }
}

impl FromStr for Weekday {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SU" => Ok(Self::Sunday),
            "MO" => Ok(Self::Monday),
            "TU" => Ok(Self::Tuesday),
            "WE" => Ok(Self::Wednesday),
            "TH" => Ok(Self::Thursday),
            "FR" => Ok(Self::Friday),
            "SA" => Ok(Self::Saturday),
            _ => Err(anyhow!("Invalid weekday")) //TODO
        }
    }
}

impl ToString for Weekday {
    fn to_string(&self) -> String {
        match &self {
            Self::Sunday => "SU",
            Self::Monday => "MO",
            Self::Tuesday => "TU",
            Self::Wednesday => "WE",
            Self::Thursday => "TH",
            Self::Friday => "FR",
            Self::Saturday => "SA",
        }.to_string()
    }
}

impl DateOrDateTime {
    fn from_str(value: &str, params: &ICalParameterMap) -> anyhow::Result<Self> {
        match ICalDateTime::parse(value, params) {
            Ok(datetime) => Ok(Self::DateTime(datetime)),
            Err(_) => {
                let date = ICalDate::parse(value, params)
                    .context("DateOrDateTime has invalid date and datetime")?;
                Ok(Self::Date(date))
            },
        }
    }
}

impl ToString for DateOrDateTime {
    fn to_string(&self) -> String {
        match &self {
            Self::Date(d) => d.serialize(),
            Self::DateTime(dt) => dt.serialize(),
        }
    }
}

impl Default for Frequency {
    fn default() -> Self {
        Self::Daily
    }
}

impl FromStr for ByDay {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() <= 2 {
            Ok(Self {
                ordwk: None,
                weekday: Weekday::from_str(s)?
            })
        }
        else {
            let parts: Vec<&str> = s.splitn(2, char::is_alphabetic).collect();
            Ok(Self {
                ordwk: Some(parts[0].parse()?),
                weekday: Weekday::from_str(parts[1])?,
            })
        }
    }
}

impl ToString for ByDay {
    fn to_string(&self) -> String {
        if let Some(ordwk) = &self.ordwk {
            return ordwk.to_string() + &self.weekday.to_string()
        }
        return self.weekday.to_string()
    }
}

impl ByDay {
    pub fn wk(weekday: Weekday) -> Self {
        Self {
            ordwk: None,
            weekday,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::property::*;
    use crate::values::recur::*;

    #[test]
    fn test_recur() {
        let value = "FREQ=MONTHLY;BYDAY=MO,TU,WE,TH,FR;BYSETPOS=-1";
        let expected = ICalRecur {
            freq: Frequency::Monthly,
            byday: vec![
                ByDay::wk(Weekday::Monday),
                ByDay::wk(Weekday::Tuesday),
                ByDay::wk(Weekday::Wednesday),
                ByDay::wk(Weekday::Thursday),
                ByDay::wk(Weekday::Friday),
            ],
            bysetpos: vec![-1],
            ..Default::default()
        };
        let result = ICalRecur::parse(value, &HashMap::new()).expect("Failed to parse!");
        assert_eq!(result, expected);
        let s = ICalPropertyValueTrait::serialize(&result);
        assert_eq!(s, value);
    }
}
