use std::cmp::{max, min};
use std::error::Error;
use std::str::FromStr;

use super::Challenge;

pub struct Day06 {
    instructions: Vec<Instruction>,
}

impl Challenge for Day06 {
    const DAY: u8 = 6;
    type Part1Solution = usize;
    type Part2Solution = u32;

    fn new(input: &str) -> Self {
        Self {
            instructions: input.lines().map(|line| line.parse().unwrap()).collect(),
        }
    }
    fn solve_part1(&self) -> Self::Part1Solution {
        let mut lights = Lights::new();
        for instruction in &self.instructions {
            lights.execute_instruction(*instruction);
        }
        lights.count_on()
    }
    fn solve_part2(&self) -> Self::Part2Solution {
        let mut lights = DimmableLights::new();
        for instruction in &self.instructions {
            lights.execute_instruction(*instruction);
        }
        lights.total_brightness()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    action: Action,
    coordinate1: Coordinate,
    coordinate2: Coordinate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Toggle,
    TurnOn,
    TurnOff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
enum LightState {
    Off = 0,
    On,
}

const NUM_LIGHTS_X: usize = 1000;
const NUM_LIGHTS_Y: usize = 1000;
const BOTTOM_LEFT: Coordinate = Coordinate { x: 0, y: 0 };
const TOP_RIGHT: Coordinate = Coordinate {
    x: NUM_LIGHTS_X - 1,
    y: NUM_LIGHTS_Y - 1,
};

fn coordinates_in_range(
    coordinate1: Coordinate,
    coordinate2: Coordinate,
) -> impl Iterator<Item = Coordinate> {
    let min_x = min(coordinate1.x, coordinate2.x);
    let max_x = max(coordinate1.x, coordinate2.x);
    let min_y = min(coordinate1.y, coordinate2.y);
    let max_y = max(coordinate1.y, coordinate2.y);
    (min_x..=max_x)
        .into_iter()
        .flat_map(move |x| std::iter::repeat(x).zip(min_y..=max_y))
        .map(|(x, y)| Coordinate { x, y })
}

struct Lights {
    grid: [[LightState; NUM_LIGHTS_Y]; NUM_LIGHTS_X],
}

impl Lights {
    fn new() -> Self {
        Lights {
            grid: [[LightState::Off; NUM_LIGHTS_Y]; NUM_LIGHTS_X],
        }
    }

    fn turn_on(&mut self, coordinate1: Coordinate, coordinate2: Coordinate) {
        for coordinate in coordinates_in_range(coordinate1, coordinate2) {
            self.grid[coordinate.x][coordinate.y] = LightState::On;
        }
    }

    fn turn_off(&mut self, coordinate1: Coordinate, coordinate2: Coordinate) {
        for coordinate in coordinates_in_range(coordinate1, coordinate2) {
            self.grid[coordinate.x][coordinate.y] = LightState::Off;
        }
    }

    fn toggle(&mut self, coordinate1: Coordinate, coordinate2: Coordinate) {
        for coordinate in coordinates_in_range(coordinate1, coordinate2) {
            let light_state = &mut self.grid[coordinate.x][coordinate.y];
            *light_state = match light_state {
                LightState::On => LightState::Off,
                LightState::Off => LightState::On,
            }
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        let method = match instruction.action {
            Action::TurnOn => Self::turn_on,
            Action::TurnOff => Self::turn_off,
            Action::Toggle => Self::toggle,
        };
        method(self, instruction.coordinate1, instruction.coordinate2);
    }

    fn count_on(&self) -> usize {
        let mut count = 0usize;
        for coordinate in coordinates_in_range(BOTTOM_LEFT, TOP_RIGHT) {
            if let LightState::On = self.grid[coordinate.x][coordinate.y] {
                count += 1;
            }
        }
        count
    }
}

struct DimmableLights {
    grid: [[u32; NUM_LIGHTS_Y]; NUM_LIGHTS_X],
}

impl DimmableLights {
    fn new() -> Self {
        Self {
            grid: [[0; NUM_LIGHTS_Y]; NUM_LIGHTS_X],
        }
    }

    fn increase_brightness(
        &mut self,
        increment: u32,
        coordinate1: Coordinate,
        coordinate2: Coordinate,
    ) {
        for coordinate in coordinates_in_range(coordinate1, coordinate2) {
            self.grid[coordinate.x][coordinate.y] += increment;
        }
    }

    fn decrease_brightness(
        &mut self,
        decrement: u32,
        coordinate1: Coordinate,
        coordinate2: Coordinate,
    ) {
        for coordinate in coordinates_in_range(coordinate1, coordinate2) {
            let brightness = &mut self.grid[coordinate.x][coordinate.y];
            *brightness = brightness.saturating_sub(decrement);
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        let (method, amount): (fn(&mut DimmableLights, u32, Coordinate, Coordinate), u32) =
            match instruction.action {
                Action::TurnOn => (Self::increase_brightness, 1),
                Action::TurnOff => (Self::decrease_brightness, 1),
                Action::Toggle => (Self::increase_brightness, 2),
            };
        method(
            self,
            amount,
            instruction.coordinate1,
            instruction.coordinate2,
        );
    }

    fn total_brightness(&self) -> u32 {
        coordinates_in_range(BOTTOM_LEFT, TOP_RIGHT)
            .into_iter()
            .map(|coordinate| self.grid[coordinate.x][coordinate.y])
            .sum()
    }
}

struct InstructionParser<'a> {
    input: &'a str,
    cursor: usize,
}

type ParseError = Box<dyn Error>;

impl<'a> InstructionParser<'a> {
    fn new(input: &str) -> InstructionParser {
        InstructionParser { input, cursor: 0 }
    }

    fn parse(&mut self) -> Result<Instruction, ParseError> {
        let action = self.parse_action()?;
        self.parse_space()?;
        let coordinate1 = self.parse_coordinate()?;
        self.parse_space()?;
        self.parse_literal("through")?;
        self.parse_space()?;
        let coordinate2 = self.parse_coordinate()?;
        Ok(Instruction {
            action,
            coordinate1,
            coordinate2,
        })
    }

    fn remaining_input(&self) -> &str {
        &self.input[self.cursor..]
    }

    fn parse_literal(&mut self, literal: &str) -> Result<(), ParseError> {
        if self.remaining_input().starts_with(literal) {
            self.cursor += literal.len();
            Ok(())
        } else {
            Err(format!("expected literal: {}", literal).into())
        }
    }

    fn parse_number(&mut self) -> Result<u32, ParseError> {
        let mut len = self.remaining_input().len();
        for (i, c) in self.remaining_input().char_indices() {
            if !c.is_numeric() {
                len = i;
                break;
            }
        }

        // println!(
        //     "parsing: {}, remaining_input: {}",
        //     &self.remaining_input()[..len],
        //     &self.remaining_input()
        // );
        let number = self.remaining_input()[..len].parse()?;
        self.cursor += len;
        Ok(number)
    }

    fn parse_space(&mut self) -> Result<(), ParseError> {
        self.parse_literal(" ")
    }

    fn parse_comma(&mut self) -> Result<(), ParseError> {
        self.parse_literal(",")
    }

    fn parse_action(&mut self) -> Result<Action, ParseError> {
        if self.parse_literal("turn on").is_ok() {
            Ok(Action::TurnOn)
        } else if self.parse_literal("turn off").is_ok() {
            Ok(Action::TurnOff)
        } else if self.parse_literal("toggle").is_ok() {
            Ok(Action::Toggle)
        } else {
            Err("expected action".into())
        }
    }

    fn parse_coordinate(&mut self) -> Result<Coordinate, ParseError> {
        let x = self.parse_number()?;
        self.parse_comma()?;
        let y = self.parse_number()?;
        Ok(Coordinate {
            x: x as usize,
            y: y as usize,
        })
    }
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InstructionParser::new(s).parse()
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Coordinate {
            x: value.0,
            y: value.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction() {
        assert_eq!(
            "toggle 0,1 through 123,456".parse::<Instruction>().unwrap(),
            Instruction {
                action: Action::Toggle,
                coordinate1: Coordinate { x: 0, y: 1 },
                coordinate2: Coordinate { x: 123, y: 456 }
            }
        );
        assert_eq!(
            "turn on 12,34 through 999,999"
                .parse::<Instruction>()
                .unwrap(),
            Instruction {
                action: Action::TurnOn,
                coordinate1: Coordinate { x: 12, y: 34 },
                coordinate2: Coordinate { x: 999, y: 999 }
            }
        );
        assert_eq!(
            "turn off 1234567,1 through 23,45"
                .parse::<Instruction>()
                .unwrap(),
            Instruction {
                action: Action::TurnOff,
                coordinate1: Coordinate { x: 1234567, y: 1 },
                coordinate2: Coordinate { x: 23, y: 45 }
            }
        );
    }

    #[test]
    fn parse_number() {
        assert_eq!(InstructionParser::new("456").parse_number().unwrap(), 456);
        assert_eq!(
            InstructionParser::new("12 and more")
                .parse_number()
                .unwrap(),
            12
        );
        assert!(InstructionParser::new("").parse_number().is_err());
        assert!(InstructionParser::new(" abc ").parse_number().is_err());
    }

    #[test]
    fn turn_on() {
        let mut lights = Lights::new();
        assert_eq!(lights.count_on(), 0);
        lights.turn_on((0, 0).into(), (999, 999).into());
        assert_eq!(lights.count_on(), 1_000_000);
        lights.turn_on((0, 0).into(), (999, 999).into());
        assert_eq!(lights.count_on(), 1_000_000);
    }

    #[test]
    fn turn_off() {
        let mut lights = Lights::new();
        lights.turn_on((0, 0).into(), (999, 999).into());
        assert_eq!(lights.count_on(), 1_000_000);
        lights.turn_off((499, 499).into(), (500, 500).into());
        assert_eq!(lights.count_on(), 999_996);
        lights.turn_off((499, 499).into(), (500, 500).into());
        assert_eq!(lights.count_on(), 999_996);
    }

    #[test]
    fn toggle() {
        let mut lights = Lights::new();
        lights.turn_on((0, 200).into(), (0, 499).into());
        assert_eq!(lights.count_on(), 300);
        lights.toggle((0, 0).into(), (0, 999).into());
        assert_eq!(lights.count_on(), 700);
        lights.toggle((0, 0).into(), (0, 999).into());
        assert_eq!(lights.count_on(), 300);
    }
}
