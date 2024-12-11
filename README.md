# ical-rs

A flexbile and typed ICalendar (RFC 5545) library.

## Features

 - Full implemenation of every ICalendar type (see [src/values](src/values)).
 - Generated methods for every ICalendar property with all allowed types
 - Support for X & IANA properties and parameters

## Usage

### Modify Existing
```rust
let ics_str = "BEGIN:VCALENDAR...";
let mut vcal = ICalComponent::from_ics(ics_str)?;
let vtodo = vcal.expect_vtodo();
vtodo.summary("New Summary".to_string());
let new_ics_str = vcal.to_ics();
```

### Make New
```rust
let dtstamp = Tz::America__New_York.with_ymd_and_hms(1992, 12, 17, 12, 34, 56)?;
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

### X & IANA Properties
```rust
let in_ics = r#"BEGIN:VCALENDAR
X-EXAMPLE:19921217T123456
END:VCALENDAR"#;
let mut vcal = ICalComponent::from_ics(&in_ics)?;
let x_example = vcal.get_prop("X-EXAMPLE")?
    .convert_value::<ICalDateTime>()?
    .get_as::<ICalDateTime>()?;
println!("{}", x_example); // 1992-12-17 12:34:56
```
