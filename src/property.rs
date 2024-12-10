use std::{collections::HashMap, fmt::Display};

use crate::{parser::ContentLine, values::{binary::ICalBinary, boolean::ICalBoolean, date::{ICalDate, ICalDateList}, datetime::{ICalDateTime, ICalDateTimeList}, duration::ICalDuration, float::ICalFloat, geo::ICalGeo, integer::ICalInteger, period::{ICalPeriod, ICalPeriodList}, recur::ICalRecur, text::{ICalText, ICalTextList}, time::ICalTime}};

#[derive(Clone)]

pub struct ICalProperty {
    pub value: ICalPropertyValue,
    pub params: ICalParameterMap,
}

pub type ICalParameterMap = HashMap<String, String>;

pub trait ICalPropertyValueTrait: Sized {
    fn parse(value: &str, params: &ICalParameterMap) -> anyhow::Result<Self>;
    fn serialize(&self) -> String;
}

#[derive(Clone)]
/// NOTE: CAL-ADDRESS, URI, and UTC-OFFSET are represented as TEXT
pub enum ICalPropertyValue {
    Binary(ICalBinary),
    Boolean(ICalBoolean),
    Date(ICalDate),
    DateList(ICalDateList),
    DateTime(ICalDateTime),
    DateTimeList(ICalDateTimeList),
    Time(ICalTime),
    Duration(ICalDuration),
    Float(ICalFloat),
    Integer(ICalInteger),
    Period(ICalPeriod),
    PeriodList(ICalPeriodList),
    Recur(ICalRecur),
    Text(ICalText),
    TextList(ICalTextList),
    Geo(ICalGeo)
}

impl ICalProperty {
    pub fn new(value: ICalPropertyValue, params: ICalParameterMap) -> Self {
        ICalProperty { value, params }
    }

    pub fn from_value(value: ICalPropertyValue) -> Self {
        Self::new(value, HashMap::new())
    }

    pub fn set_param(&mut self, name: String, value: String) -> &mut Self {
        self.params.insert(name, value);
        self
    }

    pub(crate) fn from_content_line(cl: ContentLine) -> anyhow::Result<Self> {
        let value: ICalPropertyValue;

        //specified value type
        if cl.params.contains_key("VALUE") {
            value = match cl.params.get("VALUE").unwrap().as_str() {
                "BINARY" => ICalPropertyValue::Binary(ICalBinary::parse(&cl.value, &cl.params)?),
                "BOOLEAN" => ICalPropertyValue::Boolean(ICalBoolean::parse(&cl.value, &cl.params)?),
                "DATE" => match cl.value.contains(',') {
                    true => ICalPropertyValue::DateList(ICalDateList::parse(&cl.value, &cl.params)?),
                    false => ICalPropertyValue::Date(ICalDate::parse(&cl.value, &cl.params)?),
                },
                "DATE-TIME" => match cl.value.contains(',') {
                    true => ICalPropertyValue::DateTimeList(ICalDateTimeList::parse(&cl.value, &cl.params)?),
                    false => ICalPropertyValue::DateTime(ICalDateTime::parse(&cl.value, &cl.params)?),
                },
                "TIME" => ICalPropertyValue::Time(ICalTime::parse(&cl.value, &cl.params)?),
                "DURATION" => ICalPropertyValue::Duration(ICalDuration::parse(&cl.value, &cl.params)?),
                "FLOAT" => ICalPropertyValue::Float(ICalFloat::parse(&cl.value, &cl.params)?),
                "INTEGER" => ICalPropertyValue::Integer(ICalInteger::parse(&cl.value, &cl.params)?),
                "PERIOD" => match cl.value.contains(',') {
                    true => ICalPropertyValue::PeriodList(ICalPeriodList::parse(&cl.value, &cl.params)?),
                    false => ICalPropertyValue::Period(ICalPeriod::parse(&cl.value, &cl.params)?),
                },
                "RECUR" => ICalPropertyValue::Recur(ICalRecur::parse(&cl.value, &cl.params)?),
                _ => ICalPropertyValue::Text(ICalText::parse(&cl.value, &cl.params)?)
            }
        }

        else {
            value = match cl.name.as_str() {
                "COMPLETED" | "CREATED" | "DTEND" | "DTSTAMP" |
                "DTSTART" | "DUE" | "LAST-MODIFIED" | "RECURRENCE-ID"
                    => ICalPropertyValue::DateTime(ICalDateTime::parse(&cl.value, &cl.params)?),
                "EXDATE" | "RDATE"
                    => ICalPropertyValue::DateTimeList(ICalDateTimeList::parse(&cl.value, &cl.params)?),
                "DURATION" | "TRIGGER"
                    => ICalPropertyValue::Duration(ICalDuration::parse(&cl.value, &cl.params)?),
                "PERCENT-COMPLETE" | "PRIORITY" | "SEQUENCE" | "REPEAT"
                    => ICalPropertyValue::Integer(ICalInteger::parse(&cl.value, &cl.params)?),
                "GEO" => ICalPropertyValue::Geo(ICalGeo::parse(&cl.value, &cl.params)?),
                "FREEBUSY" => ICalPropertyValue::Period(ICalPeriod::parse(&cl.value, &cl.params)?),
                "RRULE" => ICalPropertyValue::Recur(ICalRecur::parse(&cl.value, &cl.params)?),
                "CATEGORIES" | "RESOURCES"
                    => ICalPropertyValue::TextList(ICalTextList::parse(&cl.value, &cl.params)?),
                _ => ICalPropertyValue::Text(ICalText::parse(&cl.value, &cl.params)?)
            }
        }

        Ok(Self::new(value, cl.params))
    }

    pub fn get_binary(&self) -> Option<&ICalBinary> {
        match &self.value {
            ICalPropertyValue::Binary(b) => Some(b),
            _ => None
        }
    }

    pub fn get_boolean(&self) -> Option<&ICalBoolean> {
        match &self.value {
            ICalPropertyValue::Boolean(b) => Some(b),
            _ => None
        }
    }

    pub fn get_date(&self) -> Option<&ICalDate> {
        match &self.value {
            ICalPropertyValue::Date(d) => Some(d),
            _ => None
        }
    }

    pub fn get_date_list(&self) -> Option<&ICalDateList> {
        match &self.value {
            ICalPropertyValue::DateList(p) => Some(p),
            _ => None
        }
    }

    pub fn get_datetime(&self) -> Option<&ICalDateTime> {
        match &self.value {
            ICalPropertyValue::DateTime(dt) => Some(dt),
            _ => None
        }
    }

    pub fn get_datetime_list(&self) -> Option<&ICalDateTimeList> {
        match &self.value {
            ICalPropertyValue::DateTimeList(p) => Some(p),
            _ => None
        }
    }

    pub fn get_time(&self) -> Option<&ICalTime> {
        match &self.value {
            ICalPropertyValue::Time(t) => Some(t),
            _ => None
        }
    }

    pub fn get_duration(&self) -> Option<&ICalDuration> {
        match &self.value {
            ICalPropertyValue::Duration(d) => Some(d),
            _ => None
        }
    }

    pub fn get_float(&self) -> Option<&ICalFloat> {
        match &self.value {
            ICalPropertyValue::Float(f) => Some(f),
            _ => None
        }
    }

    pub fn get_integer(&self) -> Option<&ICalInteger> {
        match &self.value {
            ICalPropertyValue::Integer(i) => Some(i),
            _ => None
        }
    }

    pub fn get_period(&self) -> Option<&ICalPeriod> {
        match &self.value {
            ICalPropertyValue::Period(p) => Some(p),
            _ => None
        }
    }

    pub fn get_period_list(&self) -> Option<&ICalPeriodList> {
        match &self.value {
            ICalPropertyValue::PeriodList(p) => Some(p),
            _ => None
        }
    }

    pub fn get_recur(&self) -> Option<&ICalRecur> {
        match &self.value {
            ICalPropertyValue::Recur(r) => Some(r),
            _ => None
        }
    }

    pub fn get_text_list(&self) -> Option<&ICalTextList> {
        match &self.value {
            ICalPropertyValue::TextList(tl) => Some(tl),
            _ => None
        }
    }

    pub fn get_text(&self) -> Option<&ICalText> {
        match &self.value {
            ICalPropertyValue::Text(t) => Some(t),
            _ => None
        }
    }

    pub fn get_geo(&self) -> Option<&ICalGeo> {
        match &self.value {
            ICalPropertyValue::Geo(g) => Some(g),
            _ => None
        }
    }
}

impl ICalPropertyValue {
    pub fn serialize(&self) -> String {
        match self {
            Self::Binary(b) => b.serialize(),
            Self::Boolean(b) => b.serialize(),
            Self::Date(b) => b.serialize(),
            Self::DateList(b) => b.serialize(),
            Self::DateTime(b) => b.serialize(),
            Self::DateTimeList(b) => b.serialize(),
            Self::Time(b) => b.serialize(),
            Self::Duration(b) => b.serialize(),
            Self::Float(b) => b.serialize(),
            Self::Integer(b) => b.serialize(),
            Self::Period(b) => b.serialize(),
            Self::PeriodList(b) => b.serialize(),
            Self::Recur(b) => b.serialize(),
            Self::Text(b) => b.serialize(),
            Self::TextList(b) => b.serialize(),
            Self::Geo(b) => b.serialize(),
        }
    }
}
