#[cfg(test)]
mod tests {
    use crate::ical::objects::{generics::{ICalObject, VCalendar}, vtodo::VTodo};

    #[test]
    fn test_info_loss() {
        let in_ics = r#"BEGIN:VCALENDAR
VERSION:2.0
CALSCALE:GREGORIAN
PRODID:-//Apple Inc.//iOS 18.0.1//EN
BEGIN:VTODO
COMPLETED:20241016T020342Z
CREATED:20241014T211812Z
DTSTAMP:20241027T214435Z
LAST-MODIFIED:20241016T020342Z
PERCENT-COMPLETE:100
STATUS:COMPLETED
DESCRIPTION:
SUMMARY:Chop Saw
UID:F87D9736-8ADE-47E4-AC46-638B5C86E7D0
X-APPLE-SORT-ORDER:740793996
END:VTODO
END:VCALENDAR"#.replace("\n", "\r\n");

        let vtodo_expected = VTodo::builder()
            .uid("".into())
            .priority(10.into());

        let vcal_expected = VCalendar {
            children: vec![

            ]
        };

        let vcal = VCalendar::parse(&in_ics).expect("Failed to parse ical");
        let out_ics = vcal.to_ics();

        let in_lines: Vec<&str> = in_ics.split("\r\n").collect();
        let out_lines: Vec<&str> = out_ics.split("\r\n").collect();
        // println!("IN:\n{}\n", in_ics);
        // println!("OUT:\n{}\n", out_ics);

        let in_lines_len = in_lines.len();
        let out_lines_len = out_lines.len();
        if in_lines_len != out_lines_len {
            panic!("Lines lost! Expected {} got {}", in_lines_len, out_lines_len);
        }

        for in_line in in_lines {
            assert!(
                out_lines.contains(&in_line),
                "Output does not contain {}", in_line
            );
        }
    }
}
