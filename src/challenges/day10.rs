use std::{iter::Peekable, str::Chars};

use super::Challenge;

pub struct Day10 {
    input: String,
}

impl Challenge for Day10 {
    const DAY: u8 = 10;

    type Part1Solution = usize;
    type Part2Solution = usize;

    fn new(input: &str) -> Self {
        Self {
            input: input.trim().to_owned(),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        look_and_say_n_times(&self.input, 40).len()
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        look_and_say_n_times(&self.input, 50).len()
    }
}

fn look_and_say(input: &str) -> String {
    Scanner::new(input).scan().unwrap_or(String::new())
}

fn look_and_say_n_times(input: &str, n: usize) -> String {
    let mut current = input.to_owned();
    for _ in 0..n {
        current = look_and_say(&current);
    }
    current
}

struct Scanner<'a> {
    input: Peekable<Chars<'a>>,
    output: String,
}

type ScanError = String;

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            output: String::new(),
        }
    }

    pub fn scan(mut self) -> Result<String, ScanError> {
        while self.input.peek().is_some() {
            self.scan_digit_run()?
        }
        Ok(self.output)
    }

    fn scan_digit_run(&mut self) -> Result<(), ScanError> {
        let digit = self
            .input
            .next()
            .ok_or("unexpected end of input".to_owned())?;
        let mut count = 1;

        while let Some(next_digit) = self.input.peek() {
            if *next_digit != digit {
                break;
            }
            self.input.next();
            count += 1;
        }

        self.output.push_str(&format!("{}{}", count, digit));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say_n_times() {
        assert_eq!(look_and_say_n_times("1", 1), "11");
        assert_eq!(look_and_say_n_times("1", 2), "21");
        assert_eq!(look_and_say_n_times("1", 3), "1211");
        assert_eq!(look_and_say_n_times("1", 4), "111221");
        assert_eq!(look_and_say_n_times("1", 5), "312211");
    }
}
