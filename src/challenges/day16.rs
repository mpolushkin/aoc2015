use std::{collections::HashMap, str::FromStr};

use super::Challenge;

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

impl Day16 {
    fn find_matching_sue<T: Fn(&str, u32, u32) -> bool + Copy>(&self, strategy: T) -> &Sue {
        let potential_matches = self
            .sues
            .iter()
            .filter_map(|sue| {
                if sue.attributes.matches_reference(&self.reference, strategy) {
                    Some(sue)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let strategy_name = std::any::type_name::<T>();
        match potential_matches.len() {
            0 => {
                panic!("no Sues match using strategy {}", strategy_name)
            }
            1 => potential_matches[0],
            _ => {
                panic!(
                    "expected exactly one match using strategy {}, got: {:?}",
                    strategy_name, potential_matches
                )
            }
        }
    }
}

impl Challenge for Day16 {
    const DAY: u8 = 16;

    type Part1Solution = u32;
    type Part2Solution = u32;

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
        self.find_matching_sue(part1_attribute_matches).id
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        self.find_matching_sue(part2_attribute_matches).id
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

    fn matches_reference(
        &self,
        reference: &Attributes,
        strategy: impl Fn(&str, u32, u32) -> bool,
    ) -> bool {
        self.inner
            .iter()
            .filter_map(|(key, &self_value)| {
                if let Some(&reference_value) = reference.inner.get(key) {
                    Some((key, self_value, reference_value))
                } else {
                    None
                }
            })
            .all(|(key, self_value, other_value)| strategy(key, self_value, other_value))
    }
}

fn part1_attribute_matches(_name: &str, tested_value: u32, reference_value: u32) -> bool {
    tested_value == reference_value
}

fn part2_attribute_matches(name: &str, tested_value: u32, reference_value: u32) -> bool {
    match name {
        "cats" | "trees" => tested_value > reference_value,
        "pomeranians" | "goldfish" => tested_value < reference_value,
        _ => tested_value == reference_value,
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

        assert!(reference.matches_reference(&Attributes::new(), part1_attribute_matches));
        assert!(reference.matches_reference(
            &Attributes::with_attributes(
                [("goldfish".to_owned(), 10), ("children".to_owned(), 7),]
            ),
            part1_attribute_matches
        ));

        assert!(!reference.matches_reference(
            &Attributes::with_attributes(
                [("goldfish".to_owned(), 10), ("children".to_owned(), 8),]
            ),
            part1_attribute_matches
        ));
    }

    #[test]
    fn test_part2_matching() {
        assert!(!part2_attribute_matches("some attribute", 2, 3));
        assert!(part2_attribute_matches("some attribute", 2, 2));
        assert!(!part2_attribute_matches("some attribute", 2, 1));

        assert!(!part2_attribute_matches("cats", 2, 3));
        assert!(!part2_attribute_matches("cats", 2, 2));
        assert!(part2_attribute_matches("cats", 2, 1));

        assert!(!part2_attribute_matches("trees", 2, 3));
        assert!(!part2_attribute_matches("trees", 2, 2));
        assert!(part2_attribute_matches("trees", 2, 1));

        assert!(part2_attribute_matches("pomeranians", 2, 3));
        assert!(!part2_attribute_matches("pomeranians", 2, 2));
        assert!(!part2_attribute_matches("pomeranians", 2, 1));

        assert!(part2_attribute_matches("goldfish", 2, 3));
        assert!(!part2_attribute_matches("goldfish", 2, 2));
        assert!(!part2_attribute_matches("goldfish", 2, 1));
    }
}
