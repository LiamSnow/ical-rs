use std::{collections::HashMap, iter::Peekable, str::{Chars, Lines}};
use anyhow::anyhow;

use crate::{component::ICalComponent, property::ICalProp};

pub fn parse(ical_str: &str) -> anyhow::Result<ICalComponent> {
    let mut lines = ical_str.lines().peekable();

    let begin_line = lines.next().ok_or(anyhow!("ICal string is empty!"))?.to_uppercase();
    if begin_line != "BEGIN:VCALENDAR" {
        return Err(anyhow!("ICal started with {begin_line} not BEGIN:VCALENDAR!").into());
    }

    Ok(ICalComponent::parse("VCALENDAR".into(), &mut lines)?)
}

impl ICalComponent {
    fn parse(component_name: &str, lines: &mut Peekable<Lines>) -> anyhow::Result<ICalComponent> {
        let mut properties: HashMap<String, ICalProp> = HashMap::new();
        let mut components: HashMap<String, ICalComponent> = HashMap::new();

        while let Some(line) = lines.next() {
            let line = unfold_line(line, lines);
            let cl = ContentLine::parse(&line)?;
            let name = cl.name.clone();

            match name.as_str() {
                "BEGIN" => {
                    let comp = ICalComponent::parse(&cl.value, lines)?;
                    components.insert(cl.value, comp);
                },
                "END" => {
                    if cl.value != component_name {
                        return Err(anyhow!("Unexpected END in component!"));
                    }
                    break
                },
                _ => {
                    let prop = ICalProp::parse(cl)?;
                    properties.insert(name, prop);
                }
            }
        }

        Ok(ICalComponent {
            properties, components
        })
    }
}

fn unfold_line(line: &str, lines: &mut Peekable<Lines>) -> String {
    let mut line = line.to_string();
    while next_line_folded(lines) {
        let mut next_line = lines.next().unwrap().to_string();
        next_line.remove(0);
        line.push_str(&next_line);
    }
    line
}

fn next_line_folded(lines: &mut Peekable<Lines>) -> bool {
    if let Some(next_line) = lines.peek() {
        return next_line.starts_with(' ') || next_line.starts_with('\t')
    }
    false
}

#[derive(Debug, Clone)]
pub struct ContentLine {
    pub name: String,
    pub params: HashMap<String, String>,
    pub value: String,
}

impl ContentLine {
    ///RFC5545 3.1: "name *(";" param ) ":" value CRLF"
    pub fn parse(line: &str) -> anyhow::Result<Self> {
        let mut chars = line.chars().peekable();
        let mut params: HashMap<String, String> = HashMap::new();

        let (name, delim) = Self::read_until_delim(&mut chars, vec![';', ':'])?;

        if delim == ';' {
            Self::parse_params(&mut chars, &mut params)?;
        }

        let mut value = String::new();
        while let Some(c) = chars.next() {
            value.push(c);
        }

        Ok(ContentLine { name, params, value })
    }

    /// RFC 5545 3.2
    /// Recursively read paramters
    /// Example: DIR="text";
    /// Example: DIR=asdfasdf;
    fn parse_params(chars: &mut Peekable<Chars>, params: &mut HashMap<String, String>) -> anyhow::Result<()> {
        let (name, _) = Self::read_until_delim(chars, vec!['='])?;

        let val_start = chars.peek().expect("ContentLine Parameter had unexpected end after =".into());

        let value: String;
        let delim: char;

        if *val_start == '"' {
            value = Self::read_string(chars)?;
            delim = chars.next().expect("ContentLine Parameter had unexpected end after quoted value!");
        }
        else {
            (value, delim) = Self::read_until_delim(chars, vec![';', ':'])?;
        }

        params.insert(name, value);

        match delim {
            ';' => Self::parse_params(chars, params),
            ':' => Ok(()),
            _ => Err(anyhow!("ContentLine Parameter had unexpected char after value"))
        }
    }

    fn read_until_delim(chars: &mut Peekable<Chars>, delims: Vec<char>) -> anyhow::Result<(String, char)> {
        let mut name = String::new();
        while let Some(c) = chars.next() {
            if delims.contains(&c) {
                return Ok((name, c))
            }
            name.push(c.to_ascii_uppercase())
        }
        Err(anyhow!("ContentLine missing delimeter!"))
    }

    /// RFC: "Property parameter values MUST NOT contain the DQUOTE character."
    /// Reads from the first quote to the next quote
    fn read_string(chars: &mut Peekable<Chars>) -> anyhow::Result<String> {
        let mut s = String::new();
        chars.next().expect("ContentLine Parameter quote value is empty!");
        while let Some(c) = chars.next() {
            if c == '"' {
                return Ok(s)
            }
            s.push(c)
        }
        Err(anyhow!("ContentLine Parameter quote value unexpectedly ended!"))
    }
}



