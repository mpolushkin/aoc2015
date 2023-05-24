use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

use super::Challenge;

pub struct Day13 {
    opinion_registry: OpinionRegistry,
}

impl Challenge for Day13 {
    const DAY: u8 = 13;

    type Part1Solution = i32;
    type Part2Solution = i32;

    fn new(input: &str) -> Self {
        Self {
            opinion_registry: OpinionRegistry::from_opinions(
                input.lines().map(|line| line.parse().unwrap()),
            ),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        self.opinion_registry
            .find_happiness_change_of_best_arrangement()
            .unwrap()
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        let mut opinion_registry = self.opinion_registry.clone();
        for guest in opinion_registry
            .guests()
            .map(|guest| guest.to_owned())
            .collect_vec()
        {
            opinion_registry.register_opinion(InterpersonalOpinion {
                subject: "me".to_owned(),
                other: guest.to_owned(),
                happiness_change: 0,
            });
            opinion_registry.register_opinion(InterpersonalOpinion {
                subject: guest.to_owned(),
                other: "me".to_owned(),
                happiness_change: 0,
            });
        }
        opinion_registry
            .find_happiness_change_of_best_arrangement()
            .unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct InterpersonalOpinion {
    subject: String,
    other: String,
    happiness_change: i32,
}

type ParseError = String;

impl FromStr for InterpersonalOpinion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InterpersonalOpinionParser::new(s).parse()
    }
}

struct InterpersonalOpinionParser<'a> {
    input: &'a str,
    cursor: usize,
}

impl<'a> InterpersonalOpinionParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, cursor: 0 }
    }

    fn parse(mut self) -> Result<InterpersonalOpinion, ParseError> {
        let subject = self.parse_name()?;
        self.expect_literal(" would ")?;
        let happiness_change = self.parse_happiness_change()?;
        self.expect_literal(" happiness units by sitting next to ")?;
        let other = self.parse_name()?;
        self.expect_literal(".")?;
        self.expect_end()?;
        Ok(InterpersonalOpinion {
            subject,
            other,
            happiness_change,
        })
    }

    fn remaining_input(&self) -> &str {
        &self.input[self.cursor..]
    }

    fn take_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> &str {
        let last_cursor = self.cursor;
        self.cursor = match self.input[last_cursor..].find(|c| !predicate(c)) {
            Some(i) => self.cursor + i,
            None => self.input.len(),
        };
        &self.input[last_cursor..self.cursor]
    }

    fn parse_name(&mut self) -> Result<String, ParseError> {
        Ok(self.take_while(char::is_alphabetic).into())
    }

    fn parse_happiness_change(&mut self) -> Result<i32, ParseError> {
        let sign = self.parse_sign()?;
        self.expect_literal(" ")?;
        let magnitude = self.parse_number()?;
        Ok(sign * magnitude)
    }

    fn parse_sign(&mut self) -> Result<i32, ParseError> {
        match self.take_while(char::is_alphabetic) {
            "gain" => Ok(1),
            "lose" => Ok(-1),
            _ => Err("invalid sign".to_owned()),
        }
    }

    fn parse_number(&mut self) -> Result<i32, ParseError> {
        Ok(self
            .take_while(char::is_numeric)
            .parse::<i32>()
            .map_err(|_| "invalid number".to_owned())?)
    }

    fn expect_literal(&mut self, expected: &str) -> Result<(), ParseError> {
        if !self.remaining_input().starts_with(expected) {
            Err(format!("expected literal \"{}\"", expected))
        } else {
            self.cursor += expected.len();
            Ok(())
        }
    }

    fn expect_end(&self) -> Result<(), ParseError> {
        if self.cursor == self.input.len() {
            Ok(())
        } else {
            Err("input didn't end after valid parse".to_owned())
        }
    }
}

#[derive(Clone)]
struct OpinionRegistry {
    happiness_change_by_subject_other: HashMap<String, HashMap<String, i32>>,
}

impl OpinionRegistry {
    fn new() -> Self {
        Self {
            happiness_change_by_subject_other: HashMap::new(),
        }
    }

    fn from_opinions(opinions: impl IntoIterator<Item = InterpersonalOpinion>) -> Self {
        let mut opinion_registry = OpinionRegistry::new();
        for opinion in opinions.into_iter() {
            opinion_registry.register_opinion(opinion);
        }
        opinion_registry
    }

    fn register_opinion(&mut self, opinion: InterpersonalOpinion) {
        self.happiness_change_by_subject_other
            .entry(opinion.subject)
            .or_default()
            .insert(opinion.other, opinion.happiness_change);
    }

    fn happiness_change(&self, subject: &str, other: &str) -> Option<i32> {
        Some(
            *self
                .happiness_change_by_subject_other
                .get(subject)?
                .get(other)?,
        )
    }

    fn happiness_change_when_seated(&self, subject: &str, neighbors: (&str, &str)) -> Option<i32> {
        Some(
            self.happiness_change(subject, neighbors.0)?
                + self.happiness_change(subject, neighbors.1)?,
        )
    }

    fn guests(&self) -> impl Iterator<Item = &str> {
        self.happiness_change_by_subject_other
            .keys()
            .map(|guest| guest as &str)
    }

    fn num_guests(&self) -> usize {
        self.happiness_change_by_subject_other.len()
    }

    fn possible_seating_arrangements(&self) -> Result<impl Iterator<Item = Vec<&str>>, String> {
        let num_guests = self.num_guests();
        if num_guests < 3 {
            return Err("not enough guests".to_owned());
        }
        let mut guests = self.guests();
        let first_guest = guests.next().unwrap();
        Ok(guests.permutations(num_guests - 1).map(|mut other_guests| {
            other_guests.insert(0, first_guest);
            other_guests
        }))
    }

    fn happiness_change_of_arrangement(
        &self,
        seating_arrangement: &Vec<&str>,
    ) -> Result<i32, String> {
        CircularWindowsOf3::new(seating_arrangement)
            .ok_or("invalid seating_arrangement".to_owned())?
            .map(|[neighbor1, subject, neighbor2]| {
                self.happiness_change_when_seated(subject, (neighbor1, neighbor2))
            })
            .sum::<Option<i32>>()
            .ok_or("incomplete opinion data".to_owned())
    }

    fn find_happiness_change_of_best_arrangement(&self) -> Result<i32, String> {
        self.possible_seating_arrangements()?
            .map(|arrangement| self.happiness_change_of_arrangement(&arrangement))
            .max()
            .expect("no possible seating arrangements")
    }
}

#[derive(Debug)]
struct CircularWindowsOf3<'a, T: Copy> {
    source: &'a Vec<T>,
    cursor: usize,
}

impl<'a, T: Copy> CircularWindowsOf3<'a, T> {
    fn new(source: &'a Vec<T>) -> Option<Self> {
        if source.len() >= 3 {
            Some(Self { source, cursor: 0 })
        } else {
            None
        }
    }
}

impl<'a, T: Copy> Iterator for CircularWindowsOf3<'a, T> {
    type Item = [T; 3];

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.source.len();
        let cursor = self.cursor;
        if cursor >= len {
            None
        } else {
            let item = [
                self.source[cursor],
                self.source[(cursor + 1) % len],
                self.source[(cursor + 2) % len],
            ];
            self.cursor += 1;
            Some(item)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            "Alice would gain 54 happiness units by sitting next to Bob."
                .parse::<InterpersonalOpinion>()
                .unwrap(),
            InterpersonalOpinion {
                subject: "Alice".into(),
                other: "Bob".into(),
                happiness_change: 54,
            }
        );
        assert_eq!(
            "Steve would lose 1 happiness units by sitting next to Claire."
                .parse::<InterpersonalOpinion>()
                .unwrap(),
            InterpersonalOpinion {
                subject: "Steve".into(),
                other: "Claire".into(),
                happiness_change: -1,
            }
        );
    }

    #[test]
    fn test_circular_windows_of_3() {
        assert!(CircularWindowsOf3::new(&vec![1, 2, 3, 4]).unwrap().eq([
            [1, 2, 3],
            [2, 3, 4],
            [3, 4, 1],
            [4, 1, 2],
        ]
        .into_iter()),);
    }

    #[test]
    fn test_opinion_registry() {
        let opinions = [
            "Alice would gain 54 happiness units by sitting next to Bob.",
            "Alice would lose 79 happiness units by sitting next to Carol.",
            "Alice would lose 2 happiness units by sitting next to David.",
            "Bob would gain 83 happiness units by sitting next to Alice.",
            "Bob would lose 7 happiness units by sitting next to Carol.",
            "Bob would lose 63 happiness units by sitting next to David.",
            "Carol would lose 62 happiness units by sitting next to Alice.",
            "Carol would gain 60 happiness units by sitting next to Bob.",
            "Carol would gain 55 happiness units by sitting next to David.",
            "David would gain 46 happiness units by sitting next to Alice.",
            "David would lose 7 happiness units by sitting next to Bob.",
            "David would gain 41 happiness units by sitting next to Carol.",
        ]
        .into_iter()
        .map(|line| line.parse::<InterpersonalOpinion>().unwrap());

        let opinion_registry = OpinionRegistry::from_opinions(opinions);

        assert_eq!(opinion_registry.happiness_change("Alice", "Unknown"), None);
        assert_eq!(
            opinion_registry.happiness_change("Alice", "Carol"),
            Some(-79)
        );

        assert_eq!(
            opinion_registry.happiness_change_when_seated("Alice", ("David", "Bob")),
            Some(-2 + 54)
        );

        // Since only position relative to neighbors counts, not absolute position at the table, we
        // can take one guest as the reference (index 0), and arrange the others
        assert_eq!(
            opinion_registry
                .possible_seating_arrangements()
                .unwrap()
                .count(),
            3 * 2 * 1
        );

        assert_eq!(
            opinion_registry
                .find_happiness_change_of_best_arrangement()
                .unwrap(),
            330
        );
    }
}
