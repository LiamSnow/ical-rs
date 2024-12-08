use crate::{component::ICalComponent, property::ICalProperty};
use unicode_segmentation::UnicodeSegmentation;

const CRLF: &str = "\r\n";

pub fn to_ics(vcal: ICalComponent) -> String {
    let mut ics = String::new();
    vcal.to_ics(&mut ics, "VCALENDAR", true);
    ics
}

impl ICalComponent {
    fn to_ics(&self, ics: &mut String, comp_name: &str, init: bool) {
        if !init {
            ics.push_str(CRLF);
        }
        ics.push_str("BEGIN:");
        ics.push_str(comp_name);
        for (prop_name, prop) in &self.properties {
            prop.to_ics(ics, prop_name);
        }
        for (comp_name, comp) in &self.components {
            comp.to_ics(ics, comp_name, false);
        }
        ics.push_str(CRLF);
        ics.push_str("END:");
        ics.push_str(comp_name);
    }
}

impl ICalProperty {
    fn to_ics(&self, ics: &mut String, name: &str) {
        let line = self.make_line(name);
        self.fold_push(ics, &line);
    }

    /// RFC5545 3.1: Lines of text SHOULD NOT be longer than 75 octets,
    /// excluding the line break. Long content lines SHOULD be split into a
    /// multiple line representations using a line "folding" technique. That
    /// is, a long line can be split between any two characters by inserting
    /// a CRLF immediately followed by a single linear white-space character
    fn fold_push(&self, ics: &mut String, line: &str) {
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
                    ics.push(' ');
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
        for (name, value) in &self.params {
            line.push(';');
            line.push_str(name);
            line.push('=');
            //TODO quotes
            line.push_str(value);
        }
        line.push(':');
        line.push_str(&self.serialize_value());
        line
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::property::{ICalProperty, ICalPropertyValue};

    #[test]
    fn test_serialize_property_line_fold() {
        let name = "EXAMPLE";
        let str = "This is a really long line. ".repeat(20);
        let prop = ICalProperty {
            value: ICalPropertyValue::Text(str),
            params: HashMap::new(),
        };
        let mut ics = String::new();
        prop.to_ics(&mut ics, name);
        for line in ics.lines() {
            let len = line.len();
            if len > 75 {
                panic!("Line is too long ({} > 75). Line: \"{}\"", len, line);
            }
        }
    }
}
