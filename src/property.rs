use std::collections::HashMap;
use anyhow::anyhow;

use crate::{parser::ContentLine, values::{text::ICalText, GetEitherFromICalValue, GetFromICalValue, ICalValue, ICalValueTrait}};

#[derive(Clone)]

pub struct ICalProperty {
    pub value: ICalValue,
    pub params: ICalParameterMap,
}

pub type ICalParameterMap = HashMap<String, String>;


impl ICalProperty {
    pub fn new(value: ICalValue, params: ICalParameterMap) -> Self {
        ICalProperty { value, params }
    }

    /// creates a property with value and no parameters
    pub fn from_value(value: ICalValue) -> Self {
        Self::new(value, HashMap::new())
    }

    pub fn set_param(&mut self, name: &str, value: &str) -> &mut Self {
        self.params.insert(name.to_string(), value.to_string());
        self
    }

    pub fn get_param(&self, name: &str) -> Option<&String> {
        self.params.get(name)
    }

    pub(crate) fn from_content_line(cl: ContentLine) -> anyhow::Result<Self> {
        let value = match cl.params.get("VALUE") {
            Some(v) => ICalValue::from_value_param(v, &cl.value, &cl.params)?,
            None => ICalValue::from_default(&cl.name, &cl.value, &cl.params)?,
        };
        Ok(Self::new(value, cl.params))
    }

    /// Try to convert property value to a specified type
    /// NOTE: starting value must be of type ICalText
    /// (X and IANA props will be if not already converted)
    pub fn convert_value<T>(&mut self) -> anyhow::Result<&mut Self>
    where
        T: ICalValueTrait,
        ICalValue: From<T>,
    {
        let value: &ICalText = self.get_as().ok_or(anyhow!("Value must be ICalText to convert!"))?;
        let new_value = T::parse(value, &self.params)?;
        self.value = new_value.into();
        Ok(self)
    }

    pub fn get_as<T: GetFromICalValue>(&self) -> Option<&T> {
        T::from_value(&self.value)
    }

    pub fn get_as_either<'a, T>(&'a self) -> Option<T>
    where
        T: GetEitherFromICalValue<'a>
    {
        T::from_value(&self.value)
    }
}


// macro_rules! params {
//     () => {
//         {
//             let map: ICalParameterMap = HashMap::new();
//             map
//         }
//     };
//     ($($key:ident = $value:expr),+ $(,)?) => {
//         {
//             let mut map: ICalParameterMap = HashMap::new();
//             $(
//                 map.insert(stringify!($key).to_string(), stringify!($value).to_string());
//             )+
//             map
//         }
//     };
// }
//
// pub use params;
