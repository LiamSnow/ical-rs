use std::{collections::HashMap, fmt::Display};

use crate::{parser::ContentLine, values::{address::ICalAddress, binary::ICalBinary, boolean::ICalBoolean, date::ICalDate, datetime::ICalDateTime, duration::ICalDuration, float::ICalFloat, integer::ICalInteger, period::ICalPeriod, recur::ICalRecur, text::{ICalText, ICalTextList}, time::ICalTime}};

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
pub enum ICalPropertyValue {
    Binary(ICalBinary),
    Boolean(ICalBoolean),
    CalAddress(ICalAddress),
    Date(ICalDate),
    DateTime(ICalDateTime),
    Time(ICalTime),
    Duration(ICalDuration),
    Float(ICalFloat),
    Integer(ICalInteger),
    Period(ICalPeriod),
    Recur(ICalRecur),
    Text(ICalText),
    TextList(ICalTextList),
    // Uri(ICalUri),
    // UtcOffset(ICalUtcOffset),
}

impl ICalProperty {
    pub fn new(value: ICalPropertyValue, params: ICalParameterMap) -> Self {
        ICalProperty { value, params }
    }

    pub(crate) fn from_content_line(cl: ContentLine) -> anyhow::Result<Self> {
        let value: ICalPropertyValue;

        //specified value type
        if cl.params.contains_key("VALUE") {
            value = match cl.params.get("VALUE").unwrap().as_str() {
                "BINARY" => ICalPropertyValue::Binary(ICalBinary::parse(&cl.value, &cl.params)?),
                "BOOLEAN" => ICalPropertyValue::Boolean(ICalBoolean::parse(&cl.value, &cl.params)?),
                "CAL-ADDRESS" => ICalPropertyValue::CalAddress(ICalAddress::parse(&cl.value, &cl.params)?),
                "DATE" => ICalPropertyValue::Date(ICalDate::parse(&cl.value, &cl.params)?),
                "DATE-TIME" => ICalPropertyValue::DateTime(ICalDateTime::parse(&cl.value, &cl.params)?),
                "DURATION" => ICalPropertyValue::Duration(ICalDuration::parse(&cl.value, &cl.params)?),
                "FLOAT" => ICalPropertyValue::Float(ICalFloat::parse(&cl.value, &cl.params)?),
                "INTEGER" => ICalPropertyValue::Integer(ICalInteger::parse(&cl.value, &cl.params)?),
                "PERIOD" => ICalPropertyValue::Period(ICalPeriod::parse(&cl.value, &cl.params)?),
                "RECUR" => ICalPropertyValue::Recur(ICalRecur::parse(&cl.value, &cl.params)?),
                "TIME" => ICalPropertyValue::Time(ICalTime::parse(&cl.value, &cl.params)?),
                _ => ICalPropertyValue::Text(ICalText::parse(&cl.value, &cl.params)?)
            }
        }

        //use default value type
        else {
            value = match cl.name.as_str() {
                "DTSTAMP" | "DTSTART" | "DTEND" | "COMPLETED" | "CREATED" |
                "LAST-MODIFIED" | "DUE" | "EXDATE" | "RDATE"
                    => ICalPropertyValue::DateTime(ICalDateTime::parse(&cl.value, &cl.params)?),
                "DURATION" | "TRIGGER"
                    => ICalPropertyValue::Duration(ICalDuration::parse(&cl.value, &cl.params)?),
                "PERCENT-COMPLETE" | "PRIORITY" | "SEQUENCE" | "REPEAT"
                    => ICalPropertyValue::Integer(ICalInteger::parse(&cl.value, &cl.params)?),
                "CATEGORIES" => ICalPropertyValue::TextList(ICalTextList::parse(&cl.value, &cl.params)?),
                _ => ICalPropertyValue::Text(ICalText::parse(&cl.value, &cl.params)?)
            }
        }

        Ok(Self::new(value, cl.params))
    }

    pub(crate) fn serialize_value(&self) -> String {
        match &self.value {
            ICalPropertyValue::Binary(b) => b.serialize(),
            ICalPropertyValue::Boolean(b) => b.serialize(),
            ICalPropertyValue::CalAddress(b) => b.serialize(),
            ICalPropertyValue::Date(b) => b.serialize(),
            ICalPropertyValue::DateTime(b) => b.serialize(),
            ICalPropertyValue::Time(b) => b.serialize(),
            ICalPropertyValue::Duration(b) => b.serialize(),
            ICalPropertyValue::Float(b) => b.serialize(),
            ICalPropertyValue::Integer(b) => b.serialize(),
            ICalPropertyValue::Period(b) => b.serialize(),
            ICalPropertyValue::Recur(b) => b.serialize(),
            ICalPropertyValue::Text(b) => b.serialize(),
            ICalPropertyValue::TextList(b) => b.serialize(),
        }
    }

    pub fn expect_text(&self) -> Option<&str> {
        match &self.value {
            ICalPropertyValue::Text(t) => Some(t),
            _ => None
        }
    }
}

impl From<i32> for ICalProperty {
    fn from(value: i32) -> Self {
        Self {
            value: ICalPropertyValue::Integer(value),
            params: HashMap::new()
        }
    }
}

impl From<String> for ICalProperty {
    fn from(value: String) -> Self {
        Self {
            value: ICalPropertyValue::Text(value),
            params: HashMap::new()
        }
    }
}

impl From<&str> for ICalProperty {
    fn from(value: &str) -> Self {
        Self {
            value: ICalPropertyValue::Text(value.to_string()),
            params: HashMap::new()
        }
    }
}

impl Display for ICalPropertyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ICalPropertyValue::Binary(_) => write!(f, "Binary"),
            ICalPropertyValue::Boolean(b) => write!(f, "{}(Boolean)", b),
            ICalPropertyValue::CalAddress(a) => write!(f, "{}(CalAddress)", a.email),
            ICalPropertyValue::Date(d) => write!(f, "{}(Date)", d),
            ICalPropertyValue::DateTime(d) => write!(f, "{}", d),
            ICalPropertyValue::Time(t) => write!(f, "{}(Time)", t),
            ICalPropertyValue::Duration(d) => write!(f, "{}(Duration)", d),
            ICalPropertyValue::Float(n) => write!(f, "{}(Float)", n),
            ICalPropertyValue::Integer(n) => write!(f, "{}(Integer)", n),
            ICalPropertyValue::Period(p) => write!(f, "{}(Period)", p),
            ICalPropertyValue::Recur(_) => write!(f, "Recur"),
            ICalPropertyValue::Text(t) => write!(f, "{}(Text)", t),
            ICalPropertyValue::TextList(v) => write!(f, "{}(TextList)", v.join(",")),
        }
    }
}
