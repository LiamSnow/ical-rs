use chrono::{DateTime, NaiveDateTime};
use chrono_tz::Tz;
use either::Either;

use crate::{component::ICalComponent, property::ICalPropertyValue, values::{binary::ICalBinary, datetime::ICalDateTime, text::ICalText}};

#[derive(Debug, PartialEq)]
pub enum GetPropError {
    PropertyMissing,
    WrongValueType,
}

gen_prop_methods!(
    calscale Text,
    method Text,

    ///EXAMPLE 2
    uid Text,
);

impl ICalComponent {
    pub fn dtstamp_zoned(&mut self, value: DateTime<Tz>) -> &mut Self {
        self.set_prop("DTSTAMP", value.into())
    }
    pub fn dtstamp_local(&mut self, value: NaiveDateTime) -> &mut Self {
        self.set_prop("DTSTAMP", value.into())
    }
    pub fn get_dtstamp(&self) -> Result<&ICalDateTime, GetPropError> {
        Ok(self.props.get("DTSTAMP")
            .ok_or(GetPropError::PropertyMissing)?
            .get_datetime()
            .ok_or(GetPropError::WrongValueType)?)
    }
}

macro_rules! gen_prop_methods {
    (
        $(
            $(#[$field_meta:meta])*
            $prop:ident $type:ident,
        )+
    ) => {
        impl ICalComponent {
            $(
                gen_prop_methods!(@prop_methods $(#[$field_meta])* $prop $type);
            )+
        }
    };

    (@prop_methods $(#[$field_meta:meta])* $prop:ident Text) => {
        paste::paste! {
            //Sets the value to a new Text value without any parameters
            $(#[$field_meta])*
            pub fn $prop(&mut self, value: &str) -> &mut Self {
                self.set_prop(gen_prop_methods!(@prop_name $prop), value.into())
            }

            //Gets an immutable reference to the Text value
            $(#[$field_meta])*
            pub fn [<get_ $prop>](&self) -> Result<&str, GetPropError> {
                Ok(self.props.get(gen_prop_methods!(@prop_name $prop))
                    .ok_or(GetPropError::PropertyMissing)?
                    .get_text()
                    .ok_or(GetPropError::WrongValueType)?)
            }
        }
    };

    (@prop_name $prop:ident) => {
        paste::paste! {
            &stringify!([<$prop:upper>]).replace("_", "-")
        }
    };
}

pub(crate) use gen_prop_methods;

impl ICalComponent {
    pub fn attach(&mut self, value: &str) -> &mut Self {
        self.set_prop("ATTACH", value.into())
    }
    pub fn get_attachments(&self) -> Result<Vec<Either<&ICalText, &ICalBinary>>, GetPropError> {
        Ok(self.props.get_vec("ATTACH")
            .ok_or(GetPropError::PropertyMissing)?
            .iter()
            .try_fold(Vec::new(), |mut acc, prop| {
                acc.push(match &prop.value {
                    ICalPropertyValue::Binary(b) => Either::Right(b),
                    ICalPropertyValue::Text(t) => Either::Left(t),
                    _ => return Err(GetPropError::WrongValueType),
                });
                Ok(acc)
            })?)
    }
}
