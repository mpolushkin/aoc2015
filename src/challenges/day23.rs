use std::str::FromStr;

use super::Challenge;

pub struct Day23 {
    instructions: Vec<Instruction>,
}

impl Challenge for Day23 {
    const DAY: u8 = 23;

    type Part1Solution = u32;
    type Part2Solution = u32;

    fn new(input: &str) -> Self {
        Self {
            instructions: input
                .lines()
                .map(|line| line.parse::<Instruction>().unwrap())
                .collect(),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        let mut computer = Computer::with_instructions(self.instructions.clone());
        computer.run();
        computer.b
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        let mut computer = Computer::with_instructions(self.instructions.clone());
        computer.a = 1;
        computer.run();
        computer.b
    }
}

struct Computer {
    a: u32,
    b: u32,
    pc: u32,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn new() -> Self {
        Computer {
            a: 0,
            b: 0,
            pc: 0,
            instructions: Vec::new(),
        }
    }

    fn with_instructions(instructions: impl IntoIterator<Item = Instruction>) -> Self {
        let mut self_ = Self::new();
        self_.instructions = instructions.into_iter().collect();
        self_
    }

    fn run(&mut self) {
        loop {
            if self.step().is_none() {
                break
            }
        }
    }

    fn step(&mut self) -> Option<Instruction> {
        let instruction = self.current_instruction()?;
        match instruction {
            Instruction::Half(register) => {
                *self.register_mut(register) /= 2;
                self.pc += 1;
            }
            Instruction::Triple(register) => {
                *self.register_mut(register) *= 3;
                self.pc += 1;
            }
            Instruction::Increment(register) => {
                *self.register_mut(register) += 1;
                self.pc += 1;
            }
            Instruction::Jump(offset) => {
                self.jump(offset);
            }
            Instruction::JumpIfEven(register, offset) => {
                if self.register(register) % 2 == 0 {
                    self.jump(offset);
                } else {
                    self.pc += 1;
                }
            }
            Instruction::JumpIfOne(register, offset) => {
                if self.register(register) == 1 {
                    self.jump(offset);
                } else {
                    self.pc += 1;
                }
            }
        }
        Some(instruction)
    }

    fn current_instruction(&self) -> Option<Instruction> {
        self.instructions.get(self.pc as usize).copied()
    }

    fn register(&self, register: Register) -> u32 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
        }
    }

    fn register_mut(&mut self, register: Register) -> &mut u32 {
        match register {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }

    fn jump(&mut self, offset: i32) {
        self.pc = self.pc.checked_add_signed(offset).unwrap_or(u32::MAX);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Register {
    A,
    B,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, args) = s.split_once(' ').ok_or(ParseError)?;
        Ok(match instruction {
            "hlf" => Instruction::Half(parse_register(args)?),
            "tpl" => Instruction::Triple(parse_register(args)?),
            "inc" => Instruction::Increment(parse_register(args)?),
            "jmp" => Instruction::Jump(parse_offset(args)?),
            "jio" => {
                let args = args.split_once(", ").ok_or(ParseError)?;
                Instruction::JumpIfOne(parse_register(args.0)?, parse_offset(args.1)?)
            }
            "jie" => {
                let args = args.split_once(", ").ok_or(ParseError)?;
                Instruction::JumpIfEven(parse_register(args.0)?, parse_offset(args.1)?)
            }
            _ => return Err(ParseError),
        })
    }
}

fn parse_register(s: &str) -> Result<Register, ParseError> {
    match s {
        "a" => Ok(Register::A),
        "b" => Ok(Register::B),
        _ => Err(ParseError),
    }
}

fn parse_offset(mut s: &str) -> Result<i32, ParseError> {
    if s.starts_with('+') {
        s = &s[1..];
    }
    s.parse::<i32>().map_err(|_| ParseError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!("hlf a".parse(), Ok(Instruction::Half(Register::A)));
        assert_eq!("tpl b".parse(), Ok(Instruction::Triple(Register::B)));
        assert_eq!("inc a".parse(), Ok(Instruction::Increment(Register::A)));
        assert_eq!("jmp +12".parse(), Ok(Instruction::Jump(12)));
        assert_eq!(
            "jio b, -1".parse(),
            Ok(Instruction::JumpIfOne(Register::B, -1))
        );
        assert_eq!(
            "jie a, +1".parse(),
            Ok(Instruction::JumpIfEven(Register::A, 1))
        );
    }

    #[test]
    fn test_compute() {
        let mut c = Computer::with_instructions(
            [
                "inc a",
                "tpl a",
                "hlf a",
                "inc b",
                "jio b, -1",
                "jie a, +3",
                "inc a",
                "jmp -2",
                "jmp -100",
            ]
            .map(|s| s.parse::<Instruction>().unwrap()),
        );
        assert_eq!((c.a, c.b, c.pc), (0, 0, 0));

        assert_eq!(c.step(), Some(Instruction::Increment(Register::A)));
        assert_eq!((c.a, c.b, c.pc), (1, 0, 1));

        assert_eq!(c.step(), Some(Instruction::Triple(Register::A)));
        assert_eq!((c.a, c.b, c.pc), (3, 0, 2));

        assert_eq!(c.step(), Some(Instruction::Half(Register::A)));
        assert_eq!((c.a, c.b, c.pc), (1, 0, 3));

        assert_eq!(c.step(), Some(Instruction::Increment(Register::B)));
        assert_eq!((c.a, c.b, c.pc), (1, 1, 4));
        assert_eq!(c.step(), Some(Instruction::JumpIfOne(Register::B, -1)));
        assert_eq!((c.a, c.b, c.pc), (1, 1, 3));
        assert_eq!(c.step(), Some(Instruction::Increment(Register::B)));
        assert_eq!((c.a, c.b, c.pc), (1, 2, 4));
        assert_eq!(c.step(), Some(Instruction::JumpIfOne(Register::B, -1)));
        assert_eq!((c.a, c.b, c.pc), (1, 2, 5));

        assert_eq!(c.step(), Some(Instruction::JumpIfEven(Register::A, 3)));
        assert_eq!((c.a, c.b, c.pc), (1, 2, 6));
        assert_eq!(c.step(), Some(Instruction::Increment(Register::A)));
        assert_eq!((c.a, c.b, c.pc), (2, 2, 7));
        assert_eq!(c.step(), Some(Instruction::Jump(-2)));
        assert_eq!((c.a, c.b, c.pc), (2, 2, 5));
        assert_eq!(c.step(), Some(Instruction::JumpIfEven(Register::A, 3)));
        assert_eq!((c.a, c.b, c.pc), (2, 2, 8));

        assert_eq!(c.step(), Some(Instruction::Jump(-100)));
        assert_eq!(c.step(), None)
    }
}
