#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    fn test_full() {
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
summary:Example
UID:F87D9736-8ADE-47E4-AC46-638B5C86E7D0
X-TEST-PROP;VALUE=INTEGER:10
X-APPLE-SORT-ORDER:740793996
END:VTODO
END:VCALENDAR"#.replace("\n", "\r\n");

        let vcal = parser::parse(&in_ics).unwrap();
    }

    #[test]
    fn test_failure() {
        let in_ics = r#"BeGiN:VcALEnDAR
VERsION:2.0
CALScALE:GREGORIAN
PRoDID:-//Apple Inc.//iOS 18.0.1//EN
BEGiN:VTODO
COMPlETED:20241016T020342Z
CREATED:20241014T211812Z
DTSTAMP:20241027T214435Z
LASt-MODIFIED:20241016T020342Z
PERCENT-COMPLETE:100
STATuS:COMPLETED
DESCRiPTION:example description?:::::: 🦀 🦀 🦀 🦀 🦀 🦀 🦀
  🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀 🦀
SUMMaRY:Lorem ipsum dolor sit amet, consectetur adipiscin
 g elit. Vestibulum ligula ipsum, varius vel interdum ut,
 euismod varius nisi. Nulla iaculis efficitur sollicitudi
 n. Maecenas tempor sem consectetur leo feugiat luctus.
organizer;DIR="ldap://example.com:6666/o=ABC%20Industries,
 c=US???(cn=Jim%20Dolittle)":mailto:jimdo@example.com
UID:F87D9736-8ADE-47E4-AC46-638B5C86E7D0
X-TEST-PROP;VALUE=INTEGER:10
X-APPLE-SORT-ORDER:740793996
END:VTODO
END:VCALENDAR"#.replace("\n", "\r\n");

        let vcal = parser::parse(&in_ics).unwrap();
        let vtodo = vcal.get_vtodo().unwrap();
        let org = vtodo.properties.get("ORGANIZER").unwrap();
        assert_eq!(org.expect_text().unwrap(), "mailto:jimdo@example.com");
        assert_eq!(org.params.get("DIR").unwrap(), "ldap://example.com:6666/o=ABC%20Industries,c=US???(cn=Jim%20Dolittle)");
    }
}
