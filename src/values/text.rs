use crate::property::*;

pub type ICalText = String;

impl ICalPropertyValueTrait for ICalText {
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(value.to_string())
    }

    fn serialize(&self) -> String {
        self.to_string()
    }
}

pub type ICalTextList = Vec<String>;

impl ICalPropertyValueTrait for ICalTextList {
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(value.split(',').map(|s| s.to_owned()).collect())
    }

    fn serialize(&self) -> String {
        self.join(",")
    }
}

impl From<&str> for ICalProperty {
    fn from(value: &str) -> Self {
        Self::from_value(ICalPropertyValue::Text(value.to_string()))
    }
}
