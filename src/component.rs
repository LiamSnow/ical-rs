use std::mem;
use multimap::MultiMap;

use crate::property::ICalProperty;
use crate::values::ICalValue;

pub struct ICalComponent {
    pub props: ICalPropertyMap,
    pub comps: ICalComponentMap
}

pub type ICalPropertyMap = MultiMap<String, ICalProperty>;
pub type ICalComponentMap = MultiMap<String, ICalComponent>;

impl ICalComponent {
    pub fn new(props: ICalPropertyMap, comps: ICalComponentMap) -> Self {
        Self { props, comps }
    }

    pub fn empty() -> Self {
        Self::new(ICalPropertyMap::new(), ICalComponentMap::new())
    }

    pub fn build(&mut self) -> Self {
        ICalComponent {
            props: mem::take(&mut self.props),
            comps: mem::take(&mut self.comps)
        }
    }

    /// returns mut ref to first property for name
    pub fn get_prop(&mut self, name: &str) -> Option<&mut ICalProperty> {
        self.props.get_mut(name)
    }

    /// returns mut ref to first component for name
    pub fn get_comp(&mut self, name: &str) -> Option<&mut ICalComponent> {
        self.comps.get_mut(name)
    }

    /// returns mut ref to all properties for name
    pub fn get_props(&mut self, name: &str) -> Option<&mut Vec<ICalProperty>> {
        self.props.get_vec_mut(name)
    }

    /// returns mut ref to all components for name
    pub fn get_comps(&mut self, name: &str) -> Option<&mut Vec<ICalComponent>> {
        self.comps.get_vec_mut(name)
    }

    /// adds another value under the given name
    pub fn insert_prop(&mut self, name: &str, value: ICalProperty) -> &mut Self {
        self.props.insert(name.to_string(), value);
        self
    }

    /// adds another value under the given name
    pub fn insert_comp(&mut self, name: &str, value: ICalComponent) -> &mut Self {
        self.comps.insert(name.to_string(), value);
        self
    }

    /// replaces or creates the property under the given name
    pub fn set_prop(&mut self, name: &str, new_prop: ICalProperty) -> &mut Self {
        if let Some(prop) = self.get_prop(name) {
            *prop = new_prop;
        }
        else {
            self.insert_prop(name, new_prop);
        }
        self
    }

    /// replaces the value at the first component under the given name
    pub fn set_comp(&mut self, name: &str, new_comp: ICalComponent) -> &mut Self {
        if let Some(comp) = self.get_comp(name) {
            *comp = new_comp;
        }
        else {
            self.insert_comp(name, new_comp);
        }
        self
    }

    /// sets the value of the property if it exists
    /// or creates a new property with value & no parameters
    pub fn set_prop_value(&mut self, name: &str, new_value: ICalValue) -> &mut Self {
        if let Some(prop) = self.get_prop(name) {
            prop.value = new_value;
        }
        else {
            self.insert_prop(name, ICalProperty::from_value(new_value));
        }
        self
    }

    /// sets the value of property's parameter
    /// fails if property does not exist
    pub fn set_prop_param(&mut self, prop_name: &str, param_name: &str, new_value: String) -> &mut Self {
        if let Some(prop_ref) = self.get_prop(prop_name) {
            prop_ref.params.insert(param_name.to_string(), new_value);
        }
        self
    }

    /// returns the value prop's param if it exists
    pub fn get_prop_param(&self, prop_name: &str, param_name: &str) -> Option<&String> {
        self.props.get(prop_name)?.params.get(param_name)
    }
}

pub const VEVENT: &str = "VEVENT";
pub const VTODO: &str = "VTODO";
pub const VALARM: &str = "VALARM";
pub const VJOURNAL: &str = "VJOURNAL";
pub const VFREEBUSY: &str = "VFREEBUSY";
pub const VTIMEZONE: &str = "VTIMEZONE";

impl ICalComponent {
    /// creates a default VCALENDAR
    pub fn vcalendar() -> Self {
        let mut vcal = Self::empty();
        vcal.version("2.0".to_string())
            .calscale("GREGORIAN".to_string())
            .prodid("-//Liam Snow//ical-rs//EN".to_string());
        vcal
    }

    pub fn vevent(&mut self, vevent: Self) -> &mut Self {
        self.set_comp(VEVENT, vevent)
    }
    pub fn get_vevent(&mut self) -> Option<&mut Self> {
        self.get_comp(VEVENT)
    }
    pub fn expect_vevent(&mut self) -> &mut Self {
        self.get_vevent().unwrap()
    }


    pub fn vtodo(&mut self, vtodo: Self) -> &mut Self {
        self.set_comp(VTODO, vtodo)
    }
    pub fn get_vtodo(&mut self) -> Option<&mut Self> {
        self.get_comp(VTODO)
    }
    pub fn expect_vtodo(&mut self) -> &mut Self {
        self.get_vtodo().unwrap()
    }


    pub fn valarm(&mut self, valarm: Self) -> &mut Self {
        self.insert_comp(VALARM, valarm)
    }
    pub fn get_valarms(&mut self) -> Option<&mut Vec<Self>> {
        self.get_comps(VALARM)
    }
    pub fn expect_valarms(&mut self) -> &mut Vec<Self> {
        self.get_valarms().unwrap()
    }


    pub fn vjournal(&mut self, vjournal: Self) -> &mut Self {
        self.set_comp(VJOURNAL, vjournal)
    }
    pub fn get_vjournal(&mut self) -> Option<&mut Self> {
        self.get_comp(VJOURNAL)
    }
    pub fn expect_vjournal(&mut self) -> &mut Self {
        self.get_vjournal().unwrap()
    }


    pub fn vfreebusy(&mut self, vfreebusy: Self) -> &mut Self {
        self.set_comp(VFREEBUSY, vfreebusy)
    }
    pub fn get_vfreebusy(&mut self) -> Option<&mut Self> {
        self.get_comp(VFREEBUSY)
    }
    pub fn expect_vfreebusy(&mut self) -> &mut Self {
        self.get_vfreebusy().unwrap()
    }


    pub fn vtimezone(&mut self, vtimezone: Self) -> &mut Self {
        self.set_comp(VTIMEZONE, vtimezone)
    }
    pub fn get_vtimezone(&mut self) -> Option<&mut Self> {
        self.get_comp(VTIMEZONE)
    }
    pub fn expect_vtimezone(&mut self) -> &mut Self {
        self.get_vtimezone().unwrap()
    }
}
