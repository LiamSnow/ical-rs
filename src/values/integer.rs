use crate::ical::objects::generics::ICalParameterMap;

use super::base::*;

pub type ICalInteger = i32;

impl ICalPropType for ICalInteger {
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(value.parse()?)
    }

    fn serialize(&self) -> String {
        self.to_string()
    }
}
