use crate::ical::objects::generics::ICalParameterMap;
use super::base::*;

pub type ICalString = String;

impl ICalPropType for ICalString {
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(value.to_string())
    }

    fn serialize(&self) -> String {
        self.to_string()
    }
}

pub type ICalStringList = Vec<String>;

impl ICalPropType for ICalStringList {
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(value.split(',').map(|s| s.to_owned()).collect())
    }

    fn serialize(&self) -> String {
        self.join(",")
    }
}
