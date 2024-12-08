use crate::property::*;

#[derive(Clone)]
pub struct ICalAddress {
    pub email: String
}

const MAILTO_PREFIX: &'static str = "mailto:";

impl ICalPropValueTrait for ICalAddress {
    fn parse(value: &str, _: &ICalParameterMap) -> anyhow::Result<Self> {
        if !value.starts_with(MAILTO_PREFIX) {
            return Err(anyhow::anyhow!("ICalAddress must start with 'mailto:'"));
        }

        let email = value[MAILTO_PREFIX.len()..].to_string();
        if email.is_empty() {
            return Err(anyhow::anyhow!("Email address cannot be empty"));
        }

        if !is_valid_email(&email) {
            return Err(anyhow::anyhow!("Invalid email address format"));
        }

        Ok(ICalAddress { email })
    }

    fn serialize(&self) -> String {
        format!("mailto:{}", self.email)
    }
}

fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return false;
    }
    parts[1].contains('.')
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::property::*;
    use crate::values::address::*;

    #[test]
    fn test_address() {
        let value = "mailto:jane_doe@example.com";
        let addr = ICalAddress::parse(value, &HashMap::new()).expect("Failed to parse!");
        assert_eq!(addr.email, "jane_doe@example.com");
        let s = ICalPropValueTrait::serialize(&addr);
        assert_eq!(s, value);
    }
}
