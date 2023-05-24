use std::{cmp::min, str::FromStr};

use super::{Challenge, NotImplemented};

pub struct Day14 {
    contestants: Vec<Reindeer>,
}

impl Challenge for Day14 {
    const DAY: u8 = 14;

    type Part1Solution = u32;
    type Part2Solution = NotImplemented;

    fn new(input: &str) -> Self {
        Self {
            contestants: input
                .lines()
                .map(|line| line.parse::<Reindeer>().unwrap())
                .collect(),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        self.contestants
            .iter()
            .map(|reindeer| reindeer.distance_traveled_at(2503))
            .max().expect("no contestants")
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        NotImplemented
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Reindeer {
    name: String,
    speed: u32,
    flight_duration: u32,
    rest_duration: u32,
}

enum ReindeerState {
    Flying,
    Resting,
}

impl Reindeer {
    fn distance_traveled_at(&self, mut time: u32) -> u32 {
        let mut distance = 0;
        let mut state = ReindeerState::Flying;
        while time > 0 {
            let (max_state_duration, speed, next_state) = match state {
                ReindeerState::Flying => (self.flight_duration, self.speed, ReindeerState::Resting),
                ReindeerState::Resting => (self.rest_duration, 0, ReindeerState::Flying),
            };
            let time_in_this_state = min(time, max_state_duration);

            time -= time_in_this_state;
            state = next_state;
            distance += time_in_this_state * speed;
        }
        distance
    }
}

type MyError = String;
type MyResult<T> = Result<T, MyError>;

impl FromStr for Reindeer {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ReindeerInfoParser::new(s).parse()
    }
}

struct ReindeerInfoParser<'a> {
    input: &'a str,
    cursor: usize,
}

impl<'a> ReindeerInfoParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, cursor: 0 }
    }

    pub fn parse(mut self) -> MyResult<Reindeer> {
        let name = self.parse_name()?;
        self.expect_literal(" can fly ")?;
        let speed = self.parse_u32()?;
        self.expect_literal(" km/s for ")?;
        let flight_duration = self.parse_u32()?;
        self.expect_literal(" seconds, but then must rest for ")?;
        let rest_duration = self.parse_u32()?;
        self.expect_literal(" seconds.")?;
        self.expect_end()?;
        Ok(Reindeer {
            name,
            speed,
            flight_duration,
            rest_duration,
        })
    }

    fn parse_name(&mut self) -> MyResult<String> {
        Ok(self.take_while(char::is_alphabetic).to_owned())
    }

    fn parse_u32(&mut self) -> MyResult<u32> {
        self.take_while(char::is_numeric)
            .parse()
            .map_err(|_| "could not parse u32".to_owned())
    }

    fn take_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> &str {
        let last_cursor = self.cursor;
        self.cursor = match self.input[last_cursor..].find(|c| !predicate(c)) {
            Some(i) => self.cursor + i,
            None => self.input.len(),
        };
        &self.input[last_cursor..self.cursor]
    }

    fn expect_literal(&mut self, expected: &str) -> MyResult<()> {
        if !self.input[self.cursor..].starts_with(expected) {
            Err(format!("expected literal \"{}\"", expected))
        } else {
            self.cursor += expected.len();
            Ok(())
        }
    }

    fn expect_end(&self) -> MyResult<()> {
        if self.cursor == self.input.len() {
            Ok(())
        } else {
            Err("input didn't end after valid parse".to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            "Dancer can fly 27 km/s for 5 seconds, but then must rest for 132 seconds.".parse(),
            Ok(Reindeer {
                name: "Dancer".to_owned(),
                speed: 27,
                flight_duration: 5,
                rest_duration: 132,
            })
        );
    }

    #[test]
    fn test_distance_traveled() {
        let comet = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."
            .parse::<Reindeer>()
            .unwrap();
        let dancer = "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
            .parse::<Reindeer>()
            .unwrap();

        assert_eq!(comet.distance_traveled_at(1), 14);
        assert_eq!(dancer.distance_traveled_at(1), 16);

        assert_eq!(comet.distance_traveled_at(10), 140);
        assert_eq!(dancer.distance_traveled_at(10), 160);

        assert_eq!(comet.distance_traveled_at(11), 140);
        assert_eq!(dancer.distance_traveled_at(11), 176);

        assert_eq!(comet.distance_traveled_at(1000), 1120);
        assert_eq!(dancer.distance_traveled_at(1000), 1056);
    }
}
