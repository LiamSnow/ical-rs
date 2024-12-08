use std::{collections::HashMap, fmt::Display};

use crate::{parser::ContentLine, values::{address::ICalAddress, binary::ICalBinary, boolean::ICalBoolean, date::ICalDate, datetime::ICalDateTime, duration::ICalDuration, float::ICalFloat, integer::ICalInteger, period::ICalPeriod, recur::ICalRecur, text::{ICalText, ICalTextList}, time::ICalTime}};

#[derive(Clone)]
pub struct ICalProp {
    pub value: ICalPropValue,
    pub params: ICalParameterMap,
}

pub type ICalParameterMap = HashMap<String, String>;

pub trait ICalPropValueTrait: Sized {
    fn parse(value: &str, params: &ICalParameterMap) -> anyhow::Result<Self>;
    fn serialize(&self) -> String;
}

#[derive(Clone)]
pub enum ICalPropValue {
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

impl ICalProp {
    pub fn new(value: ICalPropValue, params: ICalParameterMap) -> Self {
        ICalProp { value, params }
    }

    pub fn parse(cl: ContentLine) -> anyhow::Result<Self> {
        let value: ICalPropValue;

        //specified value type
        if cl.params.contains_key("VALUE") {
            value = match cl.params.get("VALUE").unwrap().as_str() {
                "BINARY" => ICalPropValue::Binary(ICalBinary::parse(&cl.value, &cl.params)?),
                "BOOLEAN" => ICalPropValue::Boolean(ICalBoolean::parse(&cl.value, &cl.params)?),
                "CAL-ADDRESS" => ICalPropValue::CalAddress(ICalAddress::parse(&cl.value, &cl.params)?),
                "DATE" => ICalPropValue::Date(ICalDate::parse(&cl.value, &cl.params)?),
                "DATE-TIME" => ICalPropValue::DateTime(ICalDateTime::parse(&cl.value, &cl.params)?),
                "DURATION" => ICalPropValue::Duration(ICalDuration::parse(&cl.value, &cl.params)?),
                "FLOAT" => ICalPropValue::Float(ICalFloat::parse(&cl.value, &cl.params)?),
                "INTEGER" => ICalPropValue::Integer(ICalInteger::parse(&cl.value, &cl.params)?),
                "PERIOD" => ICalPropValue::Period(ICalPeriod::parse(&cl.value, &cl.params)?),
                "RECUR" => ICalPropValue::Recur(ICalRecur::parse(&cl.value, &cl.params)?),
                "TIME" => ICalPropValue::Time(ICalTime::parse(&cl.value, &cl.params)?),
                _ => ICalPropValue::Text(ICalText::parse(&cl.value, &cl.params)?)
            }
        }

        //use default value type
        else {
            value = match cl.name.as_str() {
                "DTSTAMP" | "DTSTART" | "DTEND" | "COMPLETED" | "CREATED" |
                "LAST-MODIFIED" | "DUE" | "EXDATE" | "RDATE"
                    => ICalPropValue::DateTime(ICalDateTime::parse(&cl.value, &cl.params)?),
                "DURATION" | "TRIGGER"
                    => ICalPropValue::Duration(ICalDuration::parse(&cl.value, &cl.params)?),
                "PERCENT-COMPLETE" | "PRIORITY" | "SEQUENCE" | "REPEAT"
                    => ICalPropValue::Integer(ICalInteger::parse(&cl.value, &cl.params)?),
                "CATEGORIES" => ICalPropValue::TextList(ICalTextList::parse(&cl.value, &cl.params)?),
                _ => ICalPropValue::Text(ICalText::parse(&cl.value, &cl.params)?)
            }
        }

        Ok(Self::new(value, cl.params))
    }

    pub fn serialize(&self) -> ContentLine {
        todo!()
    }

    pub fn expect_text(&self) -> Option<&str> {
        match &self.value {
            ICalPropValue::Text(t) => Some(t),
            _ => None
        }
    }
}

impl Display for ICalPropValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ICalPropValue::Binary(_) => write!(f, "Binary"),
            ICalPropValue::Boolean(b) => write!(f, "{}(Boolean)", b),
            ICalPropValue::CalAddress(a) => write!(f, "{}(CalAddress)", a.email),
            ICalPropValue::Date(d) => write!(f, "{}(Date)", d),
            ICalPropValue::DateTime(d) => write!(f, "{}", d),
            ICalPropValue::Time(t) => write!(f, "{}(Time)", t),
            ICalPropValue::Duration(d) => write!(f, "{}(Duration)", d),
            ICalPropValue::Float(n) => write!(f, "{}(Float)", n),
            ICalPropValue::Integer(n) => write!(f, "{}(Integer)", n),
            ICalPropValue::Period(p) => write!(f, "{}(Period)", p),
            ICalPropValue::Recur(_) => write!(f, "Recur"),
            ICalPropValue::Text(t) => write!(f, "{}(Text)", t),
            ICalPropValue::TextList(v) => write!(f, "{}(TextList)", v.join(",")),
        }
    }
}
