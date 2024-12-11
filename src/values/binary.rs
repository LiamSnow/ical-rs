use anyhow::anyhow;
use base64::Engine;

use crate::property::ICalParameterMap;
use super::ICalValueTrait;

///RFC5545 3.1.1
pub type ICalBinary = Vec<u8>;

impl ICalValueTrait for ICalBinary {
    /// A "BASE64" encoded character string, as defined by [RFC4648]
    /// binary = *(4b-char) [b-end]
    /// b-end      = (2b-char "==") / (3b-char "=")
    /// b-char = ALPHA / DIGIT / "+" / "/"
    fn parse(value: &str, params: &ICalParameterMap) -> anyhow::Result<Self> {
        let encoding = params.get("ENCODING")
            .ok_or(anyhow!("Binary value must have ENCODING parameter"))?;

        if encoding != "BASE64" {
            return Err(anyhow!("Binary value must have ENCODING=BASE64 parameter"));
        }

        Ok(base64::engine::general_purpose::STANDARD.decode(value)?)
    }

    fn serialize(&self) -> String {
        base64::engine::general_purpose::STANDARD.encode(self)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::values::binary::*;

    #[test]
    fn test_binary() {
        let value = "AAABAAEAEBAQAAEABAAoAQAAFgAAACgAAAAQAAAAIAAAAAEABAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAgIAAAICAgADAwMAA////AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAMwAAAAAAABNEMQAAAAAAAkQgAAAAAAJEREQgAAACECQ0QgEgAAQxQzM0E0AABERCRCREQAADRDJEJEQwAAAhA0QwEQAAAAAEREAAAAAAAAREQAAAAAAAAkQgAAAAAAAAMgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let mut params = HashMap::new();
        params.insert("ENCODING".to_string(), "BASE64".to_string());
        let bin = ICalBinary::parse(value, &params).expect("Failed to parse!");
        // assert_eq!(bin...?);
        let s = ICalValueTrait::serialize(&bin);
        assert_eq!(s, value);
    }
}
