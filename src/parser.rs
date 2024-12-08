use std::{collections::HashMap, iter::Peekable, str::Lines};
use anyhow::anyhow;

use crate::{component::ICalComponent, property::ICalProperty};

pub fn from_ics(ics: &str) -> anyhow::Result<ICalComponent> {
    let mut lines = ics.lines().peekable();
    let begin_line = lines.next().ok_or(anyhow!("ICal string is empty!"))?.to_uppercase();
    if begin_line != "BEGIN:VCALENDAR" {
        return Err(anyhow!("ICal started with {begin_line} not BEGIN:VCALENDAR!").into());
    }
    Ok(ICalComponent::from_ics("VCALENDAR".into(), &mut lines)?)
}

impl ICalComponent {
    fn from_ics(component_name: &str, lines: &mut Peekable<Lines>) -> anyhow::Result<ICalComponent> {
        let mut properties: HashMap<String, ICalProperty> = HashMap::new();
        let mut components: HashMap<String, ICalComponent> = HashMap::new();

        while let Some(line) = lines.next() {
            let line = unfold_line(line, lines);
            let cl = ContentLine::parse(&line)?;
            let name = cl.name.clone();

            match name.as_str() {
                "BEGIN" => {
                    let comp = ICalComponent::from_ics(&cl.value, lines)?;
                    components.insert(cl.value, comp);
                },
                "END" => {
                    if cl.value != component_name {
                        return Err(anyhow!("Unexpected END in component!"));
                    }
                    break
                },
                _ => {
                    let prop = ICalProperty::from_content_line(cl)?;
                    properties.insert(name, prop);
                }
            }
        }

        Ok(ICalComponent {
            props: properties, comps: components
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
        let mut params: HashMap<String, String> = HashMap::new();

        let (name, delim, rest) = Self::split_once_at_delim(line, vec![';', ':'])?;

        let value = if delim == ';' {
            Self::parse_params(rest, &mut params)?
        } else {
            rest.to_string()
        };

        Ok(ContentLine { name: name.to_uppercase(), params, value })
    }

    /// RFC 5545 3.2
    /// Recursively read paramters
    /// Example: DIR="text";
    /// Example: DIR=asdfasdf;
    fn parse_params(rest: &str, params: &mut HashMap<String, String>) -> anyhow::Result<String> {
        let (name, mut rest) = rest.split_once('=')
            .ok_or(anyhow!("ContentLine Parameter missing =!"))?;

        let value: &str;
        let delim: char;
        if let Some(r) = rest.strip_prefix('"') {
            let (v, _, r) = Self::split_once_at_delim(r, vec!['"'])?;
            value = &v[0..v.len()];
            rest = &r[1..];
            delim = r.chars().next().unwrap();
        }
        else {
            (value, delim, rest) = Self::split_once_at_delim(rest, vec![';', ':'])?;
        }

        params.insert(name.to_string(), value.to_string());

        match delim {
            ';' => Self::parse_params(rest, params),
            ':' => Ok(rest.to_string()),
            _ => Err(anyhow!("ContentLine Parameter had unexpected char after value"))
        }
    }

    fn split_once_at_delim(line: &str, delims: Vec<char>) -> anyhow::Result<(&str, char, &str)> {
        let delim_pos = line.find(|c| delims.contains(&c))
            .ok_or(anyhow!("ContentLine missing delimiter!"))?;
        let delim = line.chars().nth(delim_pos).unwrap();
        let (left, right) = line.split_at(delim_pos);
        Ok((left, delim, &right[1..]))
    }
}



