use std::{collections::HashMap, fmt::Display};

use crate::property::ICalProperty;

pub struct ICalComponent {
    //TODO multi-map
    pub props: HashMap<String, ICalProperty>,
    pub comps: HashMap<String, ICalComponent>
}

impl ICalComponent {
    pub fn get_vtodo(&mut self) -> Option<&mut ICalComponent> {
        self.comps.get_mut("VTODO")
    }

    pub fn expect_vtodo(&mut self) -> &mut ICalComponent {
        self.get_vtodo().unwrap()
    }

    pub fn expect_prop(&mut self, name: &str) -> &mut ICalProperty {
        self.props.get_mut(name).unwrap()
    }
}

impl ICalComponent {
    fn fmt(&self, name: &str, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [\n", name)?;
        for (prop_name, prop) in &self.props {
            write!(f, "  {}={},\n", prop_name, prop.value)?;
        }
        write!(f, "]")?;

        for (comp_name, comp) in &self.comps {
            write!(f, "\n")?;
            comp.fmt(comp_name, f)?;
        }

        Ok(())
    }
}

impl Display for ICalComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt("VCALENDAR", f)
    }
}
