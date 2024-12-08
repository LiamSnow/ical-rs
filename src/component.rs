use std::{fmt::Display, mem};

use chrono::DateTime;
use chrono_tz::Tz;
use multimap::MultiMap;

use crate::property::ICalProperty;

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

    /// gets the first propert under the given name
    pub fn get_prop(&mut self, name: &str) -> Option<&mut ICalProperty> {
        self.props.get_mut(name)
    }

    /// gets the first component under the given name
    pub fn get_comp(&mut self, name: &str) -> Option<&mut ICalComponent> {
        self.comps.get_mut(name)
    }

    /// gets all the properties under the given name
    pub fn get_props(&mut self, name: &str) -> Option<&mut Vec<ICalProperty>> {
        self.props.get_vec_mut(name)
    }

    /// gets all the components under the given name
    pub fn get_comps(&mut self, name: &str) -> Option<&mut Vec<ICalComponent>> {
        self.comps.get_vec_mut(name)
    }

    /// force gets the first property under given name
    pub fn expect_prop(&mut self, name: &str) -> &mut ICalProperty {
        self.get_prop(name).unwrap()
    }

    /// force gets the first component under given name
    pub fn expect_comp(&mut self, name: &str) -> &mut ICalComponent {
        self.get_comp(name).unwrap()
    }

    /// adds another value under the given name
    pub fn insert_prop(&mut self, name: &str, value: ICalProperty) {
        self.props.insert(name.to_string(), value);
    }

    /// adds another value under the given name
    pub fn insert_comp(&mut self, name: &str, value: ICalComponent) {
        self.comps.insert(name.to_string(), value);
    }

    /// replaces the value at the first property under the given name
    pub fn set_prop(&mut self, name: &str, new_value: ICalProperty) -> &mut Self {
        if let Some(prop_value) = self.get_prop(name) {
            *prop_value = new_value;
        }
        else {
            self.insert_prop(name, new_value);
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


    // -------------------------------------------------------- \\


    /// creates a default VCALENDAR
    pub fn vcalendar() -> Self {
        let mut vcal = Self::empty();
        vcal.set_prop("VERSION", "2.0".into());
        vcal.set_prop("CALSCALE", "GREGORIAN".into());
        vcal.set_prop("PRODID", "-//Liam Snow//ical-rs//EN".into());
        vcal
    }


    pub fn vevent(&mut self, vevent: Self) -> &mut Self {
        self.set_comp("VEVENT", vevent)
    }
    pub fn get_vevent(&mut self) -> Option<&mut ICalComponent> {
        self.get_comp("VEVENT")
    }
    pub fn expect_vevent(&mut self) -> &mut ICalComponent {
        self.get_vevent().unwrap()
    }


    pub fn vtodo(&mut self, vtodo: Self) -> &mut Self {
        self.set_comp("VTODO", vtodo)
    }
    pub fn get_vtodo(&mut self) -> Option<&mut ICalComponent> {
        self.get_comp("VTODO")
    }
    pub fn expect_vtodo(&mut self) -> &mut ICalComponent {
        self.get_vtodo().unwrap()
    }


    pub fn valarm(&mut self, valarm: Self) -> &mut Self {
        self.set_comp("VALARM", valarm)
    }
    pub fn get_valarm(&mut self) -> Option<&mut ICalComponent> {
        self.get_comp("VALARM")
    }
    pub fn expect_valarm(&mut self) -> &mut ICalComponent {
        self.get_valarm().unwrap()
    }


    pub fn vjournal(&mut self, vjournal: Self) -> &mut Self {
        self.set_comp("VJOURNAL", vjournal)
    }
    pub fn get_vjournal(&mut self) -> Option<&mut ICalComponent> {
        self.get_comp("VJOURNAL")
    }
    pub fn expect_vjournal(&mut self) -> &mut ICalComponent {
        self.get_vjournal().unwrap()
    }


    pub fn uid(&mut self, value: &str) -> &mut Self {
        self.set_prop("UID", value.into())
    }

    pub fn dtstamp_zoned(&mut self, value: DateTime<Tz>) -> &mut Self {
        self.set_prop("DTSTAMP", value.into())
    }

    // pub fn uid_text(&mut self) -> Option<&str) {
    //
    // }
}

impl ICalComponent {
    fn fmt(&self, name: &str, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [\n", name)?;
        for (prop_name, props) in &self.props {
            for prop in props {
                write!(f, "  {}={},\n", prop_name, prop.value)?;
            }
        }
        write!(f, "]")?;

        for (comp_name, comps) in &self.comps {
            for comp in comps {
                write!(f, "\n")?;
                comp.fmt(comp_name, f)?;
            }
        }

        Ok(())
    }
}

impl Display for ICalComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt("VCALENDAR", f)
    }
}
