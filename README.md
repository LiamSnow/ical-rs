# ical-rs

WORK IN PROGRESS

ICalendar (RFC 5545) serializer and parser.

## Usage

### Modify Existing
```rust
let ics_str = "BEGIN:VCALENDAR...";
let mut vcal = parser::from_ics(ics_str).unwrap();
let vtodo = vcal.expect_vtodo();
vtodo.summary("New Summary".to_string());
let new_ics_str = vcal.to_ics();
```

### Make New
```rust
let dtstamp = Tz::America__New_York.with_ymd_and_hms(1992, 12, 17, 12, 34, 56).unwrap();
let vcal = ICalComponent::vcalendar()
    .vtodo(
        ICalComponent::empty()
            .uid_random()
            .dtstamp(dtstamp.into())
            .percent_complete(10)
            .summary("Example Todo")
            .build()
    )
    .build();
let ics_str = vcal.to_ics();
```
