use crate::property::ICalParameterMap;
use either::Either;
use crate::values::{binary::ICalBinary, boolean::ICalBoolean, date::{ICalDate, ICalDateList}, datetime::{ICalDateTime, ICalDateTimeList}, duration::ICalDuration, float::ICalFloat, geo::ICalGeo, integer::ICalInteger, period::{ICalPeriod, ICalPeriodList}, recur::ICalRecur, text::{ICalText, ICalTextList}, time::ICalTime};

pub mod date;
pub mod datetime;
pub mod integer;
pub mod text;
pub mod duration;
pub mod binary;
pub mod boolean;
pub mod float;
pub mod period;
pub mod recur;
pub mod time;
pub mod geo;

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

pub trait ICalValueTrait: Sized {
    fn parse(value: &str, params: &ICalParameterMap) -> anyhow::Result<Self>;
    fn serialize(&self) -> String;
}

impl ICalValue {
    //TODO move to generator
    pub(crate) fn from_default(name: &str, value: &str, params: &ICalParameterMap) -> anyhow::Result<Self> {
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

    pub (crate) fn from_value_param(value_param: &str, value: &str, params: &ICalParameterMap) -> anyhow::Result<Self> {
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
}

pub trait GetFromICalValue {
    fn from_value(value: &ICalValue) -> Option<&Self>;
}

pub trait GetEitherFromICalValue<'a>: Sized {
    fn from_value(value: &'a ICalValue) -> Option<Self>;
}

gen_get_either_from_ical_prop_value!(
    Text Binary,
    DateTime Date,
    DateTimeList DateList,
    Duration DateTime,
);

macro_rules! gen_prop_value_enum {
    (
        $(#[$enum_meta:meta])*
        $($typ:ident,)+
    ) => {
        paste::paste! {
            $(#[$enum_meta])*
            #[derive(Clone)]
            pub enum ICalValue {
                $(
                    $typ([<ICal $typ>]),
                )+
            }

            impl ICalValue {
                pub fn serialize(&self) -> String {
                    match self {
                        $(
                            Self::$typ(v) => v.serialize(),
                        )+
                    }
                }
            }

            $(
                impl GetFromICalValue for [<ICal $typ>] {
                    fn from_value(value: &ICalValue) -> Option<&Self> {
                        match value {
                            ICalValue::$typ(v) => Some(v),
                            _ => None,
                        }
                    }
                }

                impl From<[<ICal $typ>]> for ICalValue {
                    fn from(value: [<ICal $typ>]) -> Self {
                        ICalValue::$typ(value.into())
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
                impl<'a> GetEitherFromICalValue<'a> for Either<&'a [<ICal $type1>], &'a [<ICal $type2>]> {
                    fn from_value(value: &'a ICalValue) -> Option<Self> {
                        match value {
                            ICalValue::$type1(t) => Some(Either::Left(t)),
                            ICalValue::$type2(b) => Some(Either::Right(b)),
                            _ => None
                        }
                    }
                }
            )+
        }
    };
}

pub(crate) use gen_get_either_from_ical_prop_value;


