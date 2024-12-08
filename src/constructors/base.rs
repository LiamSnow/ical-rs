use std::collections::HashMap;

use crate::{component::ICalComponent, property::ICalProperty};

pub fn make_vcalendar(extra_props: Option<HashMap<String, ICalProperty>>, comps: HashMap<String, ICalComponent>) -> ICalComponent {
    let mut props = extra_props.unwrap_or(HashMap::new());
    props.insert("VERSION".into(), "2.0".into());
    props.insert("CALSCALE".into(), "GREGORIAN".into());
    props.insert("PRODID".into(), "LIAMSNOW".into());
    ICalComponent {
        props,
        comps
    }
}
