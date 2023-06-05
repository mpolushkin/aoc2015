mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::fs;

pub trait Challenge {
    const DAY: u8;
    type Part1Solution: Display;
    type Part2Solution: Display;

    fn new(input: &str) -> Self;
    fn solve_part1(&self) -> Self::Part1Solution;
    fn solve_part2(&self) -> Self::Part2Solution;
}

pub struct NotImplemented;
impl Display for NotImplemented {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<not implemented>")
    }
}

struct FormattedSolutions {
    part1: String,
    part2: String,
}
type FormatSolutionsFn = fn(input: &str) -> FormattedSolutions;

pub struct Challenges {
    challenges_by_day: HashMap<u8, FormatSolutionsFn>,
}

fn solve_challenge_and_format_solutions<T: Challenge>(input: &str) -> FormattedSolutions {
    let challenge = T::new(input);
    FormattedSolutions {
        part1: challenge.solve_part1().to_string(),
        part2: challenge.solve_part2().to_string(),
    }
}

impl Challenges {
    pub fn new() -> Challenges {
        let mut challenges = Challenges {
            challenges_by_day: HashMap::new(),
        };
        challenges.register::<day01::Day01>();
        challenges.register::<day02::Day02>();
        challenges.register::<day03::Day03>();
        challenges.register::<day04::Day04>();
        challenges.register::<day05::Day05>();
        challenges.register::<day06::Day06>();
        challenges.register::<day07::Day07>();
        challenges.register::<day08::Day08>();
        challenges.register::<day09::Day09>();
        challenges.register::<day10::Day10>();
        challenges.register::<day11::Day11>();
        challenges.register::<day12::Day12>();
        challenges.register::<day13::Day13>();
        challenges.register::<day14::Day14>();
        challenges.register::<day15::Day15>();
        challenges.register::<day16::Day16>();
        challenges.register::<day17::Day17>();
        challenges.register::<day18::Day18>();
        challenges.register::<day19::Day19>();
        challenges.register::<day20::Day20>();
        challenges.register::<day21::Day21>();
        challenges
    }

    fn register<T: Challenge>(&mut self) {
        self.challenges_by_day
            .insert(T::DAY, solve_challenge_and_format_solutions::<T>);
    }

    pub fn print_solutions(&self, day: u8) {
        let input = fs::read_to_string(format!("./input/day{:02}.txt", day)).unwrap();
        let solutions = self
            .challenges_by_day
            .get(&day)
            .expect(&format!("no solution for day {}", day))(&input);

        println!("Solutions for day {}:", day);
        println!("  part 1: {} ", solutions.part1);
        println!("  part 2: {} ", solutions.part2);
    }
}
