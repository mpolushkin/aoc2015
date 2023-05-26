use std::{collections::HashMap, str::FromStr};

use super::{Challenge, NotImplemented};

const SUPPLEMENTARY_INPUT: &'static str = "\
    children: 3, \
    cats: 7, \
    samoyeds: 2, \
    pomeranians: 3, \
    akitas: 0, \
    vizslas: 0, \
    goldfish: 5, \
    trees: 3, \
    cars: 2, \
    perfumes: 1 \
";

pub struct Day16 {
    reference: Attributes,
    sues: Vec<Sue>,
}

impl Challenge for Day16 {
    const DAY: u8 = 16;

    type Part1Solution = u32;
    type Part2Solution = NotImplemented;

    fn new(input: &str) -> Self {
        Self {
            reference: SUPPLEMENTARY_INPUT.parse::<Attributes>().unwrap(),
            sues: input
                .lines()
                .map(|line| line.parse::<Sue>().unwrap())
                .collect(),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        let potential_matches = self
            .sues
            .iter()
            .filter_map(|sue| {
                if sue.attributes.might_match(&self.reference) {
                    Some(sue)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if potential_matches.len() != 1 {
            panic!("expected exactly one match")
        }
        potential_matches[0].id
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        NotImplemented
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sue {
    id: u32,
    attributes: Attributes,
}

#[derive(Debug, PartialEq, Eq)]
struct Attributes {
    inner: HashMap<String, u32>,
}

impl Attributes {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add_attribute(&mut self, name: String, value: u32) {
        self.inner.insert(name, value);
    }

    fn with_attributes(attributes: impl IntoIterator<Item = (String, u32)>) -> Self {
        let mut self_ = Self::new();
        for (name, value) in attributes.into_iter() {
            self_.add_attribute(name, value);
        }
        self_
    }

    fn might_match(&self, other: &Attributes) -> bool {
        self.inner
            .iter()
            .filter_map(|(key, self_value)| {
                if let Some(other_value) = other.inner.get(key) {
                    Some((self_value, other_value))
                } else {
                    None
                }
            })
            .all(|(self_value, other_value)| self_value == other_value)
    }
}

type Error = String;
type Result<T> = std::result::Result<T, Error>;

impl FromStr for Sue {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (sue_id, attributes) = s
            .split_once(':')
            .ok_or_else(|| "expected sue id and attributes delimited by ':'")?;
        Ok(Sue {
            id: parse_sue_id(sue_id)?,
            attributes: attributes.parse::<Attributes>()?,
        })
    }
}

fn parse_sue_id(s: &str) -> Result<u32> {
    let prefix = "Sue ";
    if !s.starts_with(prefix) {
        return Err(format!("expected prefix: \"{}\"", prefix));
    }
    s[prefix.len()..]
        .parse::<u32>()
        .map_err(|_| "could not parse sue id".to_owned())
}

impl FromStr for Attributes {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self::with_attributes(
            s.split(',')
                .map(|attribute| parse_attribute(attribute))
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

fn parse_attribute(s: &str) -> Result<(String, u32)> {
    let (name, value) = s.split_once(':').ok_or_else(|| "expected ':'")?;
    Ok((
        name.trim().to_owned(),
        value
            .trim()
            .parse::<u32>()
            .map_err(|_| "failed to parse value")?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            "perfumes: 6, goldfish: 10, children: 7".parse(),
            Ok(Attributes::with_attributes([
                ("perfumes".to_owned(), 6),
                ("goldfish".to_owned(), 10),
                ("children".to_owned(), 7)
            ]))
        );
        assert!(SUPPLEMENTARY_INPUT.parse::<Attributes>().is_ok());

        assert_eq!(
            "Sue 12: children: 10, cars: 6, perfumes: 5".parse(),
            Ok(Sue {
                id: 12,
                attributes: Attributes::with_attributes([
                    ("children".to_owned(), 10),
                    ("cars".to_owned(), 6),
                    ("perfumes".to_owned(), 5),
                ])
            })
        )
    }

    #[test]
    fn test_matching() {
        let reference = Attributes::with_attributes([
            ("perfumes".to_owned(), 6),
            ("goldfish".to_owned(), 10),
            ("children".to_owned(), 7),
        ]);

        assert!(reference.might_match(&Attributes::new()));
        assert!(reference.might_match(&Attributes::with_attributes([
            ("goldfish".to_owned(), 10),
            ("children".to_owned(), 7),
        ])));

        assert!(!reference.might_match(&Attributes::with_attributes([
            ("goldfish".to_owned(), 10),
            ("children".to_owned(), 8),
        ])));
    }
}
