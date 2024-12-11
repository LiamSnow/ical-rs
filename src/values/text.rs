use crate::property::{ICalParameterMap, ICalProperty};
use super::{ICalValue, ICalValueTrait};

pub type ICalText = String;

impl ICalValueTrait for ICalText {
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(value.to_string())
    }

    fn serialize(&self) -> String {
        self.to_string()
    }
}

pub type ICalTextList = Vec<String>;

impl ICalValueTrait for ICalTextList {
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(value.split(',').map(|s| s.to_owned()).collect())
    }

    fn serialize(&self) -> String {
        self.join(",")
    }
}

impl From<&str> for ICalProperty {
    fn from(value: &str) -> Self {
        Self::from_value(ICalValue::Text(value.to_string()))
    }
}
