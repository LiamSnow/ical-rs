use crate::ical::objects::generics::ICalParameterMap;

use super::base::*;

pub type ICalBoolean = bool;

impl ICalPropType for ICalBoolean {
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        Ok(value.to_lowercase().parse()?)
    }

    fn serialize(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::ical::values::base::*;
    use crate::ical::values::boolean::*;

    #[test]
    fn test_boolean_false() {
        assert_boolean("false", false);
    }

    #[test]
    fn test_boolean_true() {
        assert_boolean("true", true);
    }

    fn assert_boolean(value: &str, expected: bool) {
        let result = ICalBoolean::parse(value, &HashMap::new()).expect("Failed to parse!");
        assert_eq!(result, expected);
        let s = ICalPropType::serialize(&result);
        assert_eq!(s, value);
    }
}
