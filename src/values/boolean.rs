use crate::property::*;

pub type ICalBoolean = bool;

impl ICalPropertyValueTrait for ICalBoolean {
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

    use crate::property::*;
    use crate::values::boolean::*;

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
        let s = ICalPropertyValueTrait::serialize(&result);
        assert_eq!(s, value);
    }
}
