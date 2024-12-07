use crate::ical::objects::generics::ICalParameterMap;

//Base Value
#[derive(Clone)]
pub struct ICalProp<T: ICalPropType> {
    pub value: T,
    pub params: ICalParameterMap,
}

pub trait ICalPropType: Sized {
    fn parse(value: &str, params: &ICalParameterMap) -> anyhow::Result<Self>;
    fn serialize(&self) -> String;
}

impl<T: ICalPropType> ICalProp<T> {
    fn new(value: &str, params: ICalParameterMap) -> anyhow::Result<Self> {
        Ok(ICalProp {
            value: T::parse(value, &params)?,
            params,
        })
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn get_params(&self) -> &ICalParameterMap {
        &self.params
    }
}
