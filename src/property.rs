use std::collections::HashMap;

use either::Either;

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

gen_prop_value_enum!(
    /// NOTE: CAL-ADDRESS, URI, and UTC-OFFSET are represented as TEXT
    Binary,
    Boolean,
    Date,
    DateList,
    DateTime,
    DateTimeList,
    Time,
    Duration,
    Float,
    Integer,
    Period,
    PeriodList,
    Recur,
    Text,
    TextList,
    Geo,
);

impl ICalProperty {
    pub fn new(value: ICalPropertyValue, params: ICalParameterMap) -> Self {
        ICalProperty { value, params }
    }

    //creates a property with value and no parameters
    pub fn from_value(value: ICalPropertyValue) -> Self {
        Self::new(value, HashMap::new())
    }

    pub fn set_param(&mut self, name: &str, value: &str) -> &mut Self {
        self.params.insert(name.to_string(), value.to_string());
        self
    }

    pub fn get_param(&self, name: &str) -> Option<&String> {
        self.params.get(name)
    }

    pub(crate) fn from_content_line(cl: ContentLine) -> anyhow::Result<Self> {
        let value = match cl.params.get("VALUE") {
            Some(v) => ICalPropertyValue::from_value_param(v, &cl.value, &cl.params)?,
            None => ICalPropertyValue::from_default(&cl.name, &cl.value, &cl.params)?,
        };
        Ok(Self::new(value, cl.params))
    }

    pub fn get_as<T: GetFromICalPropertyValue>(&self) -> Option<&T> {
        T::from_value(&self.value)
    }

    pub fn get_as_either<'a, T>(&'a self) -> Option<T>
    where
        T: GetEitherFromICalPropertyValue<'a>
    {
        T::from_value(&self.value)
    }
}

pub trait GetFromICalPropertyValue {
    fn from_value(value: &ICalPropertyValue) -> Option<&Self>;
}

pub trait GetEitherFromICalPropertyValue<'a>: Sized {
    fn from_value(value: &'a ICalPropertyValue) -> Option<Self>;
}

gen_get_either_from_ical_prop_value!(
    Text Binary,
    DateTime Date,
    DateTimeList DateList,
    Duration DateTime,
);

impl ICalPropertyValue {
    fn from_value_param(value_param: &str, value: &str, params: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(match value_param {
            "DATE" => match value.contains(',') {
                true => Self::DateList(ICalDateList::parse(value, params)?),
                false => Self::Date(ICalDate::parse(value, params)?),
            },
            "DATE-TIME" => match value.contains(',') {
                true => Self::DateTimeList(ICalDateTimeList::parse(value, params)?),
                false => Self::DateTime(ICalDateTime::parse(value, params)?),
            },
            "PERIOD" => match value.contains(',') {
                true => Self::PeriodList(ICalPeriodList::parse(value, params)?),
                false => Self::Period(ICalPeriod::parse(value, params)?),
            },
            "INTEGER" => Self::Integer(ICalInteger::parse(value, params)?),
            "DURATION" => Self::Duration(ICalDuration::parse(value, params)?),
            "FLOAT" => Self::Float(ICalFloat::parse(value, params)?),
            "TIME" => Self::Time(ICalTime::parse(value, params)?),
            "BOOLEAN" => Self::Boolean(ICalBoolean::parse(value, params)?),
            "RECUR" => Self::Recur(ICalRecur::parse(value, params)?),
            "BINARY" => Self::Binary(ICalBinary::parse(value, params)?),
            "GEO" => Self::Geo(ICalGeo::parse(value, params)?),
            _ => Self::Text(ICalText::parse(value, params)?)
        })
    }

    pub fn to_value_param(&self) -> &str {
        match self {
            Self::Binary(_) => "BINARY",
            Self::Boolean(_) => "BOOLEAN",
            Self::Date(_) => "DATE",
            Self::DateList(_) => "DATE",
            Self::DateTime(_) => "DATETIME",
            Self::DateTimeList(_) => "DATETIME",
            Self::Time(_) => "TIME",
            Self::Duration(_) => "DURATION",
            Self::Float(_) => "FLOAT",
            Self::Integer(_) => "INTEGER",
            Self::Period(_) => "PERIOD",
            Self::PeriodList(_) => "PERIOD",
            Self::Recur(_) => "RECUR",
            Self::Text(_) => "TEXT",
            Self::TextList(_) => "TEXT",
            Self::Geo(_) => "GEO",
        }
    }

    //TODO move to generator
    fn from_default(name: &str, value: &str, params: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(match name {
            "COMPLETED" | "CREATED" | "DTEND" | "DTSTAMP" |
            "DTSTART" | "DUE" | "LAST-MODIFIED" | "RECURRENCE-ID"
                => Self::DateTime(ICalDateTime::parse(value, params)?),
            "EXDATE" | "RDATE"
                => Self::DateTimeList(ICalDateTimeList::parse(value, params)?),
            "DURATION" | "TRIGGER"
                => Self::Duration(ICalDuration::parse(value, params)?),
            "PERCENT-COMPLETE" | "PRIORITY" | "SEQUENCE" | "REPEAT"
                => Self::Integer(ICalInteger::parse(value, params)?),
            "GEO" => Self::Geo(ICalGeo::parse(value, params)?),
            "FREEBUSY" => Self::Period(ICalPeriod::parse(value, params)?),
            "RRULE" => Self::Recur(ICalRecur::parse(value, params)?),
            "CATEGORIES" | "RESOURCES"
                => Self::TextList(ICalTextList::parse(value, params)?),
            _ => Self::Text(ICalText::parse(value, params)?)
        })
    }
}

macro_rules! gen_prop_value_enum {
    (
        $(#[$enum_meta:meta])*
        $($typ:ident,)+
    ) => {
        paste::paste! {
            $(#[$enum_meta])*
            #[derive(Clone)]
            pub enum ICalPropertyValue {
                $(
                    $typ([<ICal $typ>]),
                )+
            }

            impl ICalPropertyValue {
                pub fn serialize(&self) -> String {
                    match self {
                        $(
                            Self::$typ(v) => v.serialize(),
                        )+
                    }
                }
            }

            $(
                impl GetFromICalPropertyValue for [<ICal $typ>] {
                    fn from_value(value: &ICalPropertyValue) -> Option<&Self> {
                        match value {
                            ICalPropertyValue::$typ(v) => Some(v),
                            _ => None,
                        }
                    }
                }

                impl From<[<ICal $typ>]> for ICalPropertyValue {
                    fn from(value: [<ICal $typ>]) -> Self {
                        ICalPropertyValue::$typ(value.into())
                    }
                }
            )+
        }
    };
}

pub(crate) use gen_prop_value_enum;

macro_rules! gen_get_either_from_ical_prop_value {
    ($($type1:ident $type2:ident,)+) => {
        paste::paste! {
            $(
                impl<'a> GetEitherFromICalPropertyValue<'a> for Either<&'a [<ICal $type1>], &'a [<ICal $type2>]> {
                    fn from_value(value: &'a ICalPropertyValue) -> Option<Self> {
                        match value {
                            ICalPropertyValue::$type1(t) => Some(Either::Left(t)),
                            ICalPropertyValue::$type2(b) => Some(Either::Right(b)),
                            _ => None
                        }
                    }
                }
            )+
        }
    };
}

pub(crate) use gen_get_either_from_ical_prop_value;

// macro_rules! params {
//     () => {
//         {
//             let map: ICalParameterMap = HashMap::new();
//             map
//         }
//     };
//     ($($key:ident = $value:expr),+ $(,)?) => {
//         {
//             let mut map: ICalParameterMap = HashMap::new();
//             $(
//                 map.insert(stringify!($key).to_string(), stringify!($value).to_string());
//             )+
//             map
//         }
//     };
// }
//
// pub use params;
