use crate::ical::values::{base::*, integer::*, string::*, duration::*};
use super::{generics::*, macros::*};
use crate::ical::serializer::{self, ICSAble, ICSAbleWithName};
use std::vec::IntoIter;
use crate::ical::parser::{Parsable, ContentLine};
use anyhow::anyhow;
use bon::Builder;

make_ical_comp_struct! {
    /// RFC5545 3.6.6
    VAlarm {
        action Opt String,
        description Opt String,
        trigger Opt Duration,
        summary Opt String,
        duration Opt Duration,
        repeat Opt Integer,
        attach Opt String,
        attendee Mul String,
    }
}

pub enum VAlarmAction {
    Audio,
    Display,
    Email
}

impl VAlarm {
    pub const NAME: &'static str = "VALARM";

    pub fn get_action(&self) -> VAlarmAction {
        match self.action.get_value().unwrap().as_str() {
            "AUDIO" => VAlarmAction::Audio,
            "DISPLAY" => VAlarmAction::Display,
            "EMAIL" => VAlarmAction::Email,
            _ => panic!("Alarm should have been validated!")
        }
    }
}

impl Validadable for VAlarm {
    fn validate(&self) -> anyhow::Result<()> {
        self.action.get().ok_or(anyhow!("ACTION is required"))?;
        self.trigger.get().ok_or(anyhow!("TRIGGER is required"))?;

        if self.repeat.get().is_some() && self.duration.get().is_some() {
            return Err(anyhow!("REPEAT and DURATION are mutually exclusive"));
        }

        let action_str = self.action.get_value().unwrap();
        match action_str.as_str() {
            "AUDIO" | "DISPLAY" | "EMAIL" => {},
            _ => return Err(anyhow!("Invalid ACTION: {}", action_str)),
        }

        let action = self.get_action();
        if !matches!(action, VAlarmAction::Audio) {
            self.description.get().ok_or(anyhow!("DESCRIPTION is required for ACTION=DISPLAY/EMAIL"))?;
        }
        if matches!(action, VAlarmAction::Email) && self.attendee.get().len() < 1 {
            return Err(anyhow!("ATTENDEE is required for ACTION=EMAIL"));
        }

        Ok(())
    }
}

impl ICSAbleWithName for Vec<VAlarm> {
    fn to_ics_with_name(&self, _: &str, ics: &mut String) {
        for child in self {
            child.to_ics(ics);
        }
    }
}

