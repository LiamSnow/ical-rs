use crate::ical::objects::generics::ICalParameterMap;

use super::base::*;

pub type ICalFloat = f32;

impl ICalPropType for ICalFloat {
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(value.parse()?)
    }

    fn serialize(&self) -> String {
        self.to_string()
    }
}
