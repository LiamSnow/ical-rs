use std::{collections::HashMap, fmt::Display};

use crate::property::ICalProp;

pub struct ICalComponent {
    //TODO multi-map
    pub properties: HashMap<String, ICalProp>,
    pub components: HashMap<String, ICalComponent>
}

impl ICalComponent {
    pub fn get_vtodo(&self) -> Option<&ICalComponent> {
        self.components.get("VTODO")
    }
}

impl ICalComponent {
    fn fmt(&self, name: &str, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [\n", name)?;
        for (prop_name, prop) in &self.properties {
            write!(f, "  {}={},\n", prop_name, prop.value)?;
        }
        write!(f, "]")?;

        for (comp_name, comp) in &self.components {
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
