use std::collections::HashMap;

use bon::builder;

use crate::{component::ICalComponent, constructors::base::make_vcalendar, property::ICalProperty};




#[builder]
pub fn make(
uid: ICalProperty,
dtstamp: ICalProperty,
class: Option<ICalProperty>,
completed: Option<ICalProperty>,
created: Option<ICalProperty>,
description: Option<ICalProperty>,
dtstart: Option<ICalProperty>,
geo: Option<ICalProperty>,
last_modified: Option<ICalProperty>,
location: Option<ICalProperty>,
organizer: Option<ICalProperty>,
percent_complete: Option<ICalProperty>,
priority: Option<ICalProperty>,
recurrence_id: Option<ICalProperty>,
sequence: Option<ICalProperty>,
status: Option<ICalProperty>,
summary: Option<ICalProperty>,
url: Option<ICalProperty>,
due: Option<ICalProperty>,
duration: Option<ICalProperty>,
// attach: Vec<ICalProperty>,
// attendee: Vec<ICalProperty>,
// categories: Vec<ICalProperty>,
// comment: Vec<ICalProperty>,
// contact: Vec<ICalProperty>,
// exdate: Vec<ICalProperty>,
// request_status: Vec<ICalProperty>,
// related_to: Vec<ICalProperty>,
// resources: Vec<ICalProperty>,
// rdate: Vec<ICalProperty>,
// rrule: Vec<ICalProperty>,
// alarms: Vec<ICalComponent>,
    ) -> ICalComponent {
    let mut props: HashMap<String, ICalProperty> = HashMap::new();

    props.insert("UID".into(), uid);
    props.insert("DTSTAMP".into(), dtstamp);

    if let Some(class) = class {
        props.insert("CLASS".into(), class);
    }

    if let Some(summary) = summary {
        props.insert("SUMMARY".into(), summary);
    }

    let vtodo = ICalComponent {
        props,
        comps: HashMap::new()
    };

    let mut comps: HashMap<String, ICalComponent> = HashMap::new();
    comps.insert("VTODO".into(), vtodo);
    make_vcalendar(None, comps)
}
