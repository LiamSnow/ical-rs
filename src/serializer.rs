use super::{
    objects::generics::*,
    values::base::{ICalProp, ICalPropType},
};
use unicode_segmentation::UnicodeSegmentation;

const CRLF: &str = "\r\n";

impl VCalendar {
    pub fn to_ics(&self) -> String {
        let mut ics = String::new();
        ics.push_str("BEGIN:");
        ics.push_str(VCalendar::NAME);
        self.children.to_ics(&mut ics);
        end(&mut ics, VCalendar::NAME);
        ics
    }
}

pub trait ICSAble {
    fn to_ics(&self, ics: &mut String);
}

pub trait ICSAbleWithName {
    fn to_ics_with_name(&self, name: &str, ics: &mut String);
}

impl<T: ICalPropType> ICalProp<T> {
    /// RFC5545 3.1: Lines of text SHOULD NOT be longer than 75 octets,
    /// excluding the line break. Long content lines SHOULD be split into a
    /// multiple line representations using a line "folding" technique. That
    /// is, a long line can be split between any two characters by inserting
    /// a CRLF immediately followed by a single linear white-space character
    fn fold_push(&self, line: &str, ics: &mut String) {
        let graphemes = line.graphemes(true); //properly handle unicode
        let end = graphemes.clone().count() - 1;
        let (mut start, mut cur_size) = (0, 0);
        for (i, g) in graphemes.enumerate() {
            let num_bytes = g.len();
            let at_start = start == 0;
            let at_end = i == end;
            let max_bytes = if at_start { 75 } else { 74 }; //account for space
            if at_end || cur_size + num_bytes > max_bytes {
                ics.push_str(CRLF);
                if !at_start {
                    ics.push_str(" ");
                }
                ics.push_str(if at_end {
                    &line[start..]
                } else {
                    &line[start..i]
                });
                (start, cur_size) = (i, 0);
            }
            cur_size += num_bytes;
        }
    }

    /// RFC 5545 3.1: "name *(";" param ) ":" value CRLF"
    fn make_line(&self, name: &str) -> String {
        let mut line = name.to_string();
        for (key, value) in &self.params {
            line += ";";
            line += &key;
            line += "=";
            line += &value;
        }
        line += ":";
        line += &self.value.serialize();
        line
    }
}

pub fn begin(ics: &mut String, name: &str) {
    ics.push_str(CRLF);
    ics.push_str("BEGIN:");
    ics.push_str(name);
}

pub fn end(ics: &mut String, name: &str) {
    ics.push_str(CRLF);
    ics.push_str("END:");
    ics.push_str(name);
}

impl ICSAble for ICalObject {
    fn to_ics(&self, ics: &mut String) {
        match self {
            ICalObject::UnknownComponent(c) => c.to_ics(ics),
            ICalObject::UnknownProperty(c) => c.to_ics(ics),
            ICalObject::VTodo(c) => c.to_ics(ics),
            ICalObject::VAlarm(c) => c.to_ics(ics),
            ICalObject::VEvent(c) => c.to_ics(ics),
            ICalObject::VJournal(c) => c.to_ics(ics),
        }
    }
}

impl ICSAble for Vec<ICalObject> {
    fn to_ics(&self, ics: &mut String) {
        for child in self {
            child.to_ics(ics);
        }
    }
}

impl ICSAble for UnknownComponent {
    fn to_ics(&self, ics: &mut String) {
        begin(ics, &self.name);
        self.children.to_ics(ics);
        end(ics, &self.name);
    }
}

impl ICSAble for UnknownProperty {
    fn to_ics(&self, ics: &mut String) {
        self.value.to_ics_with_name(&self.name, ics)
    }
}

impl<T: ICSAbleWithName> ICSAbleWithName for Option<T> {
    fn to_ics_with_name(&self, name: &str, ics: &mut String) {
        if let Some(value) = &self {
            value.to_ics_with_name(name, ics)
        }
    }
}

impl<T: ICSAbleWithName> ICSAbleWithName for Vec<T> {
    fn to_ics_with_name(&self, name: &str, ics: &mut String) {
        for value in self {
            value.to_ics_with_name(name, ics)
        }
    }
}

impl<T: ICalPropType> ICSAbleWithName for ICalProp<T> {
    fn to_ics_with_name(&self, name: &str, ics: &mut String) {
        let line = self.make_line(name);
        self.fold_push(&line, ics)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_serialize_property_line_fold() {
        let name = "EXAMPLE".to_string();
        let str = "This is a really long line. ".repeat(20);
        let value: ICalProp<String> = ICalProp {
            value: str,
            params: HashMap::new(),
        };
        let prop = UnknownProperty { name, value };
        let mut ics = String::new();
        prop.to_ics(&mut ics);
        for line in ics.lines() {
            let len = line.len();
            if len > 75 {
                panic!("Line is too long ({} > 75). Line: \"{}\"", len, line);
            }
        }
    }
}
