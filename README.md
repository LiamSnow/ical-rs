# ical-rs

WORK IN PROGRESS

ICalendar (RFC 5545) serializer and parser.

## Usage

### Modify Existing
```rust
let ics_str = "BEGIN:VCALENDAR...";
let mut vcal = parser::from_ics(ics_str).unwrap();
let vtodo = vcal.expect_vtodo();
let summary = vtodo.expect_prop("SUMMARY");
*summary = "Example Two".into();
let new_ics_str = vcal.to_ics();
```

### Make New
```rust
let dtstamp = Tz::America__New_York.with_ymd_and_hms(1992, 12, 17, 12, 34, 56).unwrap();
let vcal = ICalComponent::vcalendar()
    .vtodo(
        ICalComponent::empty()
            .uid("128397129837129837")
            .dtstamp_zoned(dtstamp)
            .build()
    )
    .build();
let ics_str = vcal.to_ics();
```
