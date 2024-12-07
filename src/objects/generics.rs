#![allow(dead_code)]

use std::collections::HashMap;

use crate::ical::values::{base::ICalProp, string::ICalString};

use super::{valarm::VAlarm, vevent::VEvent, vjournal::VJournal, vtodo::VTodo};
use bon::Builder;

pub enum ICalObject {
    UnknownComponent(UnknownComponent),
    UnknownProperty(UnknownProperty),
    VTodo(VTodo),
    VAlarm(VAlarm),
    VEvent(VEvent),
    VJournal(VJournal),
}

pub type ICalParameterMap = HashMap<String, String>;

pub struct UnknownComponent {
    pub name: String,
    pub params: ICalParameterMap,
    pub children: Vec<ICalObject>,
}

pub struct UnknownProperty {
    pub name: String,
    pub value: ICalProp<ICalString>,
}

pub struct VCalendar {
    pub children: Vec<ICalObject>,
}

impl VCalendar {
    pub const NAME: &'static str = "VCALENDAR";
}

impl ICalObject {
    pub fn get_name(&self) -> &str {
        match self {
            ICalObject::UnknownComponent(comp) => &comp.name,
            ICalObject::UnknownProperty(prop) => &prop.name,
            ICalObject::VTodo(_) => VTodo::NAME,
            ICalObject::VAlarm(_) => VAlarm::NAME,
            ICalObject::VEvent(_) => VEvent::NAME,
            ICalObject::VJournal(_) => VJournal::NAME,
        }
    }
}

pub trait Validadable {
    fn validate(&self) -> anyhow::Result<()>;
}
