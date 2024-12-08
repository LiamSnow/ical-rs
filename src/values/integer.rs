use crate::property::*;
use anyhow::Context;

pub type ICalInteger = i32;

impl ICalPropertyValueTrait for ICalInteger {
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(value.parse().context("Parsing ICalInteger")?)
    }

    fn serialize(&self) -> String {
        self.to_string()
    }
}
