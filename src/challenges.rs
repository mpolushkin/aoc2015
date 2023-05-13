pub mod day01;
pub mod day02;

use std::collections::HashMap;

trait Challenge {
    type Input;

    fn day() -> u8;
    fn get_input() -> Self::Input;
    fn solve_part1(_input: &Self::Input) -> String {
        "<not implemented>".into()
    }
    fn solve_part2(_input: &Self::Input) -> String {
        "<not implemented>".into()
    }

    fn print_solutions() {
        let input = Self::get_input();
        println!("Part 1: {}", Self::solve_part1(&input));
        println!("Part 2: {}", Self::solve_part2(&input));
    }
}

pub struct Challenges {
    challenges_by_day: HashMap<u8, fn () -> ()>
}

impl Challenges {
    pub fn new() -> Challenges {
        let mut challenges = Challenges {challenges_by_day: HashMap::new()};
        challenges.register(day01::Day01{});
        challenges.register(day02::Day02{});
        challenges
    }

    pub fn print_solutions(&self, day: u8) {
        self.challenges_by_day.get(&day).unwrap()();
    }

    fn register<T: Challenge>(&mut self, _challenge: T) {
        self.challenges_by_day.insert(T::day(), T::print_solutions);
    }
}
