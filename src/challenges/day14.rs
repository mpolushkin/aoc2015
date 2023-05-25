use std::{cmp::min, str::FromStr};

use super::Challenge;

pub struct Day14 {
    olympics: ReindeerOlympics,
}

impl Challenge for Day14 {
    const DAY: u8 = 14;

    type Part1Solution = u32;
    type Part2Solution = u32;

    fn new(input: &str) -> Self {
        Self {
            olympics: ReindeerOlympics {
                contestants: input
                    .lines()
                    .map(|line| line.parse::<ReindeerStats>().unwrap())
                    .collect(),
                race_duration: 2503,
            },
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        self.olympics.race1_leading_distance_traveled()
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        self.olympics.race2_leading_points()
    }
}

struct ReindeerOlympics {
    contestants: Vec<ReindeerStats>,
    race_duration: u32,
}

impl ReindeerOlympics {
    fn race1_leading_distance_traveled(&self) -> u32 {
        self.contestants
            .iter()
            .map(|stats| RacingReindeer::distance_traveled_at(stats, self.race_duration))
            .max()
            .expect("no contestants")
    }

    fn race2_leading_points(&self) -> u32 {
        let mut race2 = Race2::new(&self.contestants);
        race2.race_for(self.race_duration);
        race2.leading_points()
    }
}

struct Race2Contestant<'a> {
    reindeer: RacingReindeer<'a>,
    points: u32,
}

struct Race2<'a> {
    contestants: Vec<Race2Contestant<'a>>,
}

impl<'a> Race2<'a> {
    fn new(contestants: &'a Vec<ReindeerStats>) -> Self {
        Self {
            contestants: contestants
                .iter()
                .map(|stats| Race2Contestant {
                    reindeer: RacingReindeer::new(stats),
                    points: 0,
                })
                .collect(),
        }
    }

    fn race_for(&mut self, time: u32) {
        for _ in 0..time {
            for contestant in self.contestants.iter_mut() {
                contestant.reindeer.race_for(1);
            }

            let leading_distance = self
                .contestants
                .iter()
                .map(|contestant| contestant.reindeer.distance_traveled)
                .max()
                .expect("no contestants");

            for contestant in self.contestants.iter_mut() {
                if contestant.reindeer.distance_traveled == leading_distance {
                    contestant.points += 1;
                }
            }
        }
    }

    fn leading_points(&self) -> u32 {
        self.contestants
            .iter()
            .map(|contestant| contestant.points)
            .max()
            .expect("no contestants")
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ReindeerStats {
    name: String,
    speed: u32,
    flight_duration: u32,
    rest_duration: u32,
}

enum ReindeerState {
    Flying,
    Resting,
}

struct RacingReindeer<'a> {
    stats: &'a ReindeerStats,
    state: ReindeerState,
    time_to_state_change: u32,
    current_speed: u32,
    distance_traveled: u32,
}

impl<'a> RacingReindeer<'a> {
    fn new(stats: &'a ReindeerStats) -> Self {
        Self {
            stats,
            state: ReindeerState::Flying,
            time_to_state_change: stats.flight_duration,
            current_speed: stats.speed,
            distance_traveled: 0,
        }
    }

    fn distance_traveled_at(stats: &'a ReindeerStats, time: u32) -> u32 {
        let mut reindeer = Self::new(stats);
        reindeer.race_for(time);
        reindeer.distance_traveled
    }

    fn race_for(&mut self, mut time: u32) {
        while time > 0 {
            let iteration_time = min(time, self.time_to_state_change);
            time -= iteration_time;
            self.time_to_state_change -= iteration_time;
            self.distance_traveled += iteration_time * self.current_speed;

            if self.time_to_state_change == 0 {
                self.toggle_state();
            }
        }
    }

    fn toggle_state(&mut self) {
        match self.state {
            ReindeerState::Flying => {
                self.state = ReindeerState::Resting;
                self.time_to_state_change = self.stats.rest_duration;
                self.current_speed = 0;
            }
            ReindeerState::Resting => {
                self.state = ReindeerState::Flying;
                self.time_to_state_change = self.stats.flight_duration;
                self.current_speed = self.stats.speed;
            }
        }
    }
}

type MyError = String;
type MyResult<T> = Result<T, MyError>;

impl FromStr for ReindeerStats {
    type Err = MyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ReindeerStatsParser::new(s).parse()
    }
}

struct ReindeerStatsParser<'a> {
    input: &'a str,
    cursor: usize,
}

impl<'a> ReindeerStatsParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, cursor: 0 }
    }

    pub fn parse(mut self) -> MyResult<ReindeerStats> {
        let name = self.parse_name()?;
        self.expect_literal(" can fly ")?;
        let speed = self.parse_u32()?;
        self.expect_literal(" km/s for ")?;
        let flight_duration = self.parse_u32()?;
        self.expect_literal(" seconds, but then must rest for ")?;
        let rest_duration = self.parse_u32()?;
        self.expect_literal(" seconds.")?;
        self.expect_end()?;
        Ok(ReindeerStats {
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
            Ok(ReindeerStats {
                name: "Dancer".to_owned(),
                speed: 27,
                flight_duration: 5,
                rest_duration: 132,
            })
        );
    }

    #[test]
    fn test_distance_traveled() {
        let comet_stats =
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."
                .parse::<ReindeerStats>()
                .unwrap();
        let dancer_stats =
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
                .parse::<ReindeerStats>()
                .unwrap();

        assert_eq!(RacingReindeer::distance_traveled_at(&comet_stats, 1), 14);
        assert_eq!(RacingReindeer::distance_traveled_at(&dancer_stats, 1), 16);

        assert_eq!(RacingReindeer::distance_traveled_at(&comet_stats, 10), 140);
        assert_eq!(RacingReindeer::distance_traveled_at(&dancer_stats, 10), 160);

        assert_eq!(RacingReindeer::distance_traveled_at(&comet_stats, 11), 140);
        assert_eq!(RacingReindeer::distance_traveled_at(&dancer_stats, 11), 176);

        assert_eq!(
            RacingReindeer::distance_traveled_at(&comet_stats, 1000),
            1120
        );
        assert_eq!(
            RacingReindeer::distance_traveled_at(&dancer_stats, 1000),
            1056
        );
    }

    #[test]
    fn test_race2() {
        let contestants: Vec<_> = [
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        ]
        .into_iter()
        .map(|line| line.parse::<ReindeerStats>().unwrap())
        .collect();

        let comet_index = 0;
        let dancer_index = 1;

        let mut race2 = Race2::new(&contestants);

        race2.race_for(1000);
        assert_eq!(race2.contestants[dancer_index].points, 689);
        assert_eq!(race2.contestants[comet_index].points, 312);
        assert_eq!(race2.leading_points(), 689);
    }
}
