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

    pub fn get_text(&self) -> Option<&str> {
        match &self.value {
            ICalPropertyValue::Text(t) => Some(t),
            _ => None
        }
    }

    pub fn expect_text(&self) -> &str {
        self.get_text().unwrap()
    }
}

impl ICalPropertyValue {
    pub fn serialize(&self) -> String {
        match self {
            Self::Binary(b) => b.serialize(),
            Self::Boolean(b) => b.serialize(),
            Self::CalAddress(b) => b.serialize(),
            Self::Date(b) => b.serialize(),
            Self::DateTime(b) => b.serialize(),
            Self::Time(b) => b.serialize(),
            Self::Duration(b) => b.serialize(),
            Self::Float(b) => b.serialize(),
            Self::Integer(b) => b.serialize(),
            Self::Period(b) => b.serialize(),
            Self::Recur(b) => b.serialize(),
            Self::Text(b) => b.serialize(),
            Self::TextList(b) => b.serialize(),
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
