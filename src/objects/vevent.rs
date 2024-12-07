use crate::ical::values::{base::*, integer::*, string::*, duration::*, datetime::*};

use super::{generics::*, macros::*};
use crate::ical::serializer::{self, ICSAble, ICSAbleWithName};
use std::vec::IntoIter;
use crate::ical::parser::{Parsable, ContentLine};
use anyhow::anyhow;
use bon::Builder;

make_ical_comp_struct! {
    /// RFC5545 3.6.1
    VEvent {
        uid Opt String,
        dtstamp Opt DateTime,

        dtstart Opt DateTime,

        class Opt String,
        created Opt DateTime,
        description Opt String,
        geo Opt String,
        last_modified Opt DateTime,
        location Opt String,
        organizer Opt String,
        priority Opt Integer,
        sequence Opt Integer,
        status Opt String,
        transp Opt String,
        summary Opt String,
        url Opt String,
        recurrence_id Opt String,

        rrule Mul String,

        dtend Opt DateTime,
        duration Opt Duration,

        attach Mul String,
        attendee Mul String,
        categories Mul String,
        comment Mul String,
        contact Mul String,
        exdate Mul DateTime,
        request_status Mul String,
        related_to Mul String,
        resources Mul String,
        rdate Mul DateTime,
    }
}

impl VEvent {
    pub const NAME: &'static str = "VEVENT";
}

impl Validadable for VEvent {
    fn validate(&self) -> anyhow::Result<()> {
        self.uid.get().ok_or(anyhow!("UID is required"))?;
        self.dtstamp.get().ok_or(anyhow!("DTSTAMP is required"))?;

        if self.dtend.get().is_some() && self.duration.get().is_some() {
            return Err(anyhow!("DTEND and DURATION are mutually exclusive"));
        }

        Ok(())
    }
}
