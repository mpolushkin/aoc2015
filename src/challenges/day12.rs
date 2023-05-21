use serde_json::Value;

use super::Challenge;

pub struct Day12 {
    input: Value,
}

impl Challenge for Day12 {
    const DAY: u8 = 12;

    type Part1Solution = i64;
    type Part2Solution = i64;

    fn new(input: &str) -> Self {
        Self {
            input: serde_json::from_str(input).unwrap(),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        sum_all_numbers(&self.input)
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        sum_all_numbers_ignoring_red(&self.input)
    }
}

fn sum_all_numbers(value: &Value) -> i64 {
    match value {
        Value::Number(number) => number.as_i64().expect("expected signed integer"),
        Value::Array(array) => array.iter().map(|value| sum_all_numbers(value)).sum(),
        Value::Object(object) => object.values().map(|value| sum_all_numbers(value)).sum(),
        _ => 0,
    }
}

fn sum_all_numbers_ignoring_red(value: &Value) -> i64 {
    match value {
        Value::Number(number) => number.as_i64().expect("expected signed integer"),
        Value::Array(array) => array
            .iter()
            .map(|value| sum_all_numbers_ignoring_red(value))
            .sum(),
        Value::Object(object) => {
            if object.values().any(|value| match value {
                Value::String(string) => string == "red",
                _ => false,
            }) {
                0
            } else {
                object
                    .values()
                    .map(|value| sum_all_numbers_ignoring_red(value))
                    .sum()
            }
        }
        _ => 0,
    }
}
