use crate::property::ICalParameterMap;
use super::ICalValueTrait;
use anyhow::{anyhow, Context};

pub type ICalGeo = (f64, f64);

impl ICalValueTrait for ICalGeo {
    fn parse(values: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        let values: Vec<&str> = values.split(',').collect();
        if values.len() != 2 {
            return Err(anyhow!("Wrong amount of values in Geo value type"));
        }
        Ok((
            values[0].parse().context("Parsing Geo value type")?,
            values[1].parse().context("Parsing Geo value type")?
        ))
    }

    fn serialize(&self) -> String {
        format!("{},{}", self.0, self.1)
    }
}
