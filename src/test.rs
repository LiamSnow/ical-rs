#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
    use chrono_tz::Tz;

    use crate::{component::ICalComponent, values::datetime::ICalDateTime};

    #[test]
    fn test_convert_x_prop() {
        let in_ics = r#"BEGIN:VCALENDAR
X-EXAMPLE:19921217T123456
END:VCALENDAR"#;
        let mut vcal = ICalComponent::from_ics(&in_ics).unwrap();

        let x_example = vcal.get_prop("X-EXAMPLE").unwrap()
            .convert_value::<ICalDateTime>().unwrap()
            .get_as::<ICalDateTime>().unwrap();

        let expected: ICalDateTime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(1992, 12, 17).unwrap(),
            NaiveTime::from_hms_opt(12, 34, 56).unwrap()
        ).into();
        assert_eq!(x_example, &expected);
    }

    #[test]
    fn test_modify() {
        let in_ics = r#"BEGIN:VCALENDAR
BEGIN:VTODO
SUMMARY:Old Summary
END:VTODO
END:VCALENDAR"#;

        let mut vcal = ICalComponent::from_ics(&in_ics).unwrap();
        let vtodo = vcal.expect_vtodo();
        vtodo.summary("New Summary".to_string());
        let out_ics = vcal.to_ics();
        let out_lines: Vec<&str> = out_ics.lines().collect();
        assert!(out_lines.contains(&"SUMMARY:New Summary"));
    }

    #[test]
    fn test_builder() {
        let dtstamp = Tz::America__New_York.with_ymd_and_hms(1992, 12, 17, 12, 34, 56).unwrap();

        let vcal = ICalComponent::vcalendar_with_vtodo(
            ICalComponent::empty()
                .uid("128397129837129837".to_string())
                .dtstamp(dtstamp.into())
                .percent_complete(10)
                .build()
        );

        let expected_ics = r#"BEGIN:VCALENDAR
VERSION:2.0
CALSCALE:GREGORIAN
PRODID:-//Liam Snow//ical-rs//EN
BEGIN:VTODO
UID:128397129837129837
DTSTAMP:19921217T123456
PERCENT-COMPLETE:10
END:VTODO
END:VCALENDAR"#;

        assert_lines_match(&vcal.to_ics(), expected_ics);
    }

    #[test]
    fn test_lines_lost() {
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
DESCRIPTION:example description
SUMMARY:Example
UID:F87D9736-8ADE-47E4-AC46-638B5C86E7D0
ORGANIZER;DIR="ldap://example.com:6666/o=ABC%20Industries,c=US???(cn=Jim%20
 Dolittle)":mailto:jimdo@example.com
X-TEST-PROP;VALUE=INTEGER:10
X-APPLE-SORT-ORDER:740793996
END:VTODO
END:VCALENDAR"#;

        let vcal = ICalComponent::from_ics(&in_ics).unwrap();
        assert_lines_match(&vcal.to_ics(), in_ics);
    }

    /// compare ignoring line order
    fn assert_lines_match(in_ics: &str, out_ics: &str) {
        let in_lines: Vec<&str> = in_ics.lines().collect();
        let out_lines: Vec<&str> = out_ics.lines().collect();
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

    #[test]
    fn test_quoted_params() {
        let in_ics = r#"BEGIN:VCALENDAR
BEGIN:VTODO
ORGANIZER;DIR="ldap://example.com:6666/o=ABC%20Industries,
 c=US???(cn=Jim%20Dolittle)":mailto:jimdo@example.com
END:VTODO
END:VCALENDAR"#;

        let mut vcal = ICalComponent::from_ics(&in_ics).unwrap();
        let vtodo = vcal.expect_vtodo();
        assert_eq!(vtodo.get_organizer_value().unwrap(), "mailto:jimdo@example.com");
        assert_eq!(
            vtodo.get_organizer_prop().unwrap().get_param("DIR").unwrap(),
            "ldap://example.com:6666/o=ABC%20Industries,c=US???(cn=Jim%20Dolittle)"
        );
    }
}
