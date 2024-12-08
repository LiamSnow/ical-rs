use chrono::NaiveDate;

use crate::property::*;

/// RFC 5545 3.3.4: 19970714 -> July 14, 1997
pub type ICalDate = NaiveDate;

const FORMAT: &str = "%Y%m%d";

impl ICalPropValueTrait for ICalDate {
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(NaiveDate::parse_from_str(value, FORMAT)?)
    }

    fn serialize(&self) -> String {
        self.format(FORMAT).to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::property::*;
    use crate::values::date::*;

    #[test]
    fn test_date() {
        let value = "20140517";
        let expected = NaiveDate::from_ymd_opt(2014, 5, 17).unwrap();
        let date = ICalDate::parse(value, &HashMap::new()).expect("Failed to parse!");
        assert_eq!(date, expected);
        let s = ICalPropValueTrait::serialize(&date);
        assert_eq!(s, value);
    }
}
