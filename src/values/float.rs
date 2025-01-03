use crate::property::ICalParameterMap;
use super::ICalValueTrait;
use anyhow::Context;

pub type ICalFloat = f64;

impl ICalValueTrait for ICalFloat {
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(value.parse().context("Parsing ICalFloat")?)
    }

    fn serialize(&self) -> String {
        self.to_string()
    }
}
