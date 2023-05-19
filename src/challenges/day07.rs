use std::{collections::HashMap, str::FromStr};

use super::{Challenge, NotImplemented};

pub struct Day07 {
    instructions: Vec<Instruction>,
}

impl Challenge for Day07 {
    const DAY: u8 = 7;
    type Part1Solution = u16;
    type Part2Solution = NotImplemented;

    fn new(input: &str) -> Self {
        let mut instructions: Vec<_> = input
            .lines()
            .map(|line| line.parse().expect(&format!("invalid line: {}", line)))
            .collect();
        instructions.sort_topologically();
        Self { instructions }
    }
    fn solve_part1(&self) -> Self::Part1Solution {
        let wire_values = Emulator::new().execute_instructions(&self.instructions).unwrap();
        *wire_values.get("a").unwrap()
    }
    fn solve_part2(&self) -> Self::Part2Solution {
        NotImplemented {}
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    expression: Expression,
    output: String,
}

impl Instruction {
    fn incoming_wires(&self) -> Dependencies {
        Dependencies {
            operands: self.expression.operands(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Expression {
    Assignment(Operand),
    Not(Operand),
    And { lhs: Operand, rhs: Operand },
    Or { lhs: Operand, rhs: Operand },
    LShift { lhs: Operand, rhs: Operand },
    RShift { lhs: Operand, rhs: Operand },
}

impl Expression {
    fn operands(&self) -> Operands {
        match self {
            Expression::Assignment(operand) | Expression::Not(operand) => Operands {
                first_operand: operand,
                maybe_second_operand: None,
                cursor: 0,
            },
            Expression::And { lhs, rhs }
            | Expression::Or { lhs, rhs }
            | Expression::LShift { lhs, rhs }
            | Expression::RShift { lhs, rhs } => Operands {
                first_operand: lhs,
                maybe_second_operand: Some(rhs),
                cursor: 0,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operand {
    Constant(u16),
    Wire(String),
}

#[derive(Debug)]
struct Operands<'a> {
    first_operand: &'a Operand,
    maybe_second_operand: Option<&'a Operand>,
    cursor: u8,
}

impl<'a> Iterator for Operands<'a> {
    type Item = &'a Operand;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor {
            0 => {
                self.cursor += 1;
                Some(self.first_operand)
            }
            1 => {
                self.cursor += 1;
                self.maybe_second_operand
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Dependencies<'a> {
    operands: Operands<'a>,
}

impl<'a> Iterator for Dependencies<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let operand = self.operands.next()?;
            if let Operand::Wire(wire) = operand {
                return Some(&wire);
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Token {
    Not,
    And,
    Or,
    LShift,
    RShift,
    Arrow,
    Constant(u16),
    Wire(String),
}

type ParseError = String;

impl FromStr for Token {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NOT" => Ok(Token::Not),
            "AND" => Ok(Token::And),
            "OR" => Ok(Token::Or),
            "LSHIFT" => Ok(Token::LShift),
            "RSHIFT" => Ok(Token::RShift),
            "->" => Ok(Token::Arrow),
            _ => {
                if let Ok(constant) = s.parse::<u16>() {
                    Ok(Token::Constant(constant))
                } else if s.chars().all(|c| c.is_lowercase()) {
                    Ok(Token::Wire(s.to_owned()))
                } else {
                    Err(format!("invalid token: {:?}", s))
                }
            }
        }
    }
}

struct InstructionParser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl InstructionParser {
    fn new(input: &str) -> Result<Self, ParseError> {
        Ok(Self {
            tokens: input
                .split_whitespace()
                .map(|word| word.parse::<Token>())
                .collect::<Result<Vec<_>, ParseError>>()?,
            cursor: 0,
        })
    }

    fn parse(&mut self) -> Result<Instruction, ParseError> {
        let expression = self.parse_expression()?;
        self.parse_arrow()?;
        let output = self.parse_output()?;
        Ok(Instruction { expression, output })
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        match self.peek_token() {
            Some(Token::Not) => self.parse_not_expression(),
            Some(Token::Wire(_) | Token::Constant(_)) => {
                self.parse_binary_expression_or_assignment()
            }
            Some(token) => Err(format!(
                "unexpected token while parsing expression: {:?}",
                token
            )),
            None => Err("no more tokens wile parsing expression: {:?}".into()),
        }
    }

    fn parse_not_expression(&mut self) -> Result<Expression, ParseError> {
        match self.next_token() {
            Some(Token::Not) => Ok(Expression::Not(self.parse_operand()?)),
            Some(token) => Err(format!(
                "unexpected token while parsing unary expression operator: {:?}",
                token
            )),
            None => Err("no more tokens wile parsing unary expression operator".into()),
        }
    }

    fn parse_binary_expression_or_assignment(&mut self) -> Result<Expression, ParseError> {
        let lhs = self.parse_operand()?;

        if let Some(Token::Arrow) = self.peek_token() {
            return Ok(Expression::Assignment(lhs));
        }

        let operator_token = self.next_token().ok_or(String::from(
            "no more tokens wile parsing binary expression operator",
        ))?;
        let rhs = self.parse_operand()?;
        match operator_token {
            Token::And => Ok(Expression::And { lhs, rhs }),
            Token::Or => Ok(Expression::Or { lhs, rhs }),
            Token::RShift => Ok(Expression::RShift { lhs, rhs }),
            Token::LShift => Ok(Expression::LShift { lhs, rhs }),
            token @ _ => Err(format!(
                "unexpected token while parsing binary expression operator: {:?}",
                token
            )),
        }
    }

    fn parse_operand(&mut self) -> Result<Operand, ParseError> {
        match self.next_token() {
            Some(Token::Constant(value)) => Ok(Operand::Constant(value)),
            Some(Token::Wire(name)) => Ok(Operand::Wire(name)),
            Some(token) => Err(format!(
                "unexpected token while parsing operand: {:?}",
                token
            )),
            None => Err("no more tokens while parsing operand".into()),
        }
    }

    fn parse_arrow(&mut self) -> Result<(), ParseError> {
        match self.next_token() {
            Some(Token::Arrow) => Ok(()),
            Some(token) => Err(format!("unexpected token while parsing arrow: {:?}", token)),
            None => Err("no more tokens while parsing arrow".into()),
        }
    }

    fn parse_output(&mut self) -> Result<String, ParseError> {
        match self.next_token() {
            Some(Token::Wire(wire)) => Ok(wire),
            Some(token) => Err(format!(
                "unexpected token while parsing output: {:?}",
                token
            )),
            None => Err("no more tokens while parsing output".into()),
        }
    }

    fn peek_token(&mut self) -> Option<Token> {
        self.tokens.get(self.cursor).cloned()
    }

    fn next_token(&mut self) -> Option<Token> {
        let token = self.peek_token()?;
        self.cursor += 1;
        Some(token)
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InstructionParser::new(s)?.parse()
    }
}

trait TopologicalSort {
    fn sort_topologically(&mut self);
}

impl TopologicalSort for [Instruction] {
    fn sort_topologically(&mut self) {
        TopologicalSorter::new(self).sort();
    }
}

struct TopologicalSorter<'a> {
    instructions: &'a mut [Instruction],
    sorted_wire_indices: HashMap<String, usize>,
}

impl<'a> TopologicalSorter<'a> {
    fn new(instructions: &'a mut [Instruction]) -> Self {
        Self {
            instructions,
            sorted_wire_indices: HashMap::new(),
        }
    }

    fn sort(&mut self) {
        self.find_sorted_wire_indices();
        self.instructions
            .sort_by_key(|instruction| self.sorted_wire_indices.get(&instruction.output).unwrap())
    }
    fn find_sorted_wire_indices(&mut self) {
        let mut independent_indices: Vec<usize> = Vec::new();
        let mut dependent_indices_by_wire: HashMap<&str, Vec<usize>> = HashMap::new();

        for (i, instruction) in self.instructions.iter().enumerate() {
            if self.is_independent(instruction) {
                independent_indices.push(i)
            };
            for dependency in instruction.incoming_wires() {
                dependent_indices_by_wire
                    .entry(dependency)
                    .or_default()
                    .push(i);
            }
        }

        let mut next_sorted_index = 0usize;
        loop {
            match independent_indices.pop() {
                None => {
                    break;
                }
                Some(index) => {
                    let instruction = &self.instructions[index];
                    self.sorted_wire_indices
                        .insert(instruction.output.clone(), next_sorted_index);
                    next_sorted_index += 1;
                    for dependent_index in dependent_indices_by_wire
                        .entry(&instruction.output)
                        .or_default()
                        .iter()
                    {
                        if self.is_independent(&self.instructions[*dependent_index]) {
                            independent_indices.push(*dependent_index);
                        }
                    }
                }
            }
        }
    }

    fn is_independent(&self, instruction: &Instruction) -> bool {
        instruction
            .incoming_wires()
            .all(|wire| self.sorted_wire_indices.contains_key(wire))
    }
}

type WireValues = HashMap<String, u16>;
type ComputeError = String;
struct Emulator {
    wire_values: WireValues,
}

impl Emulator {
    fn new() -> Self {
        Self {
            wire_values: WireValues::new(),
        }
    }

    fn execute_instructions<'a>(
        &mut self,
        instructions: impl IntoIterator<Item = &'a Instruction>,
    ) -> Result<WireValues, ComputeError> {
        for instruction in instructions.into_iter() {
            self.execute_instruction(instruction)?;
        }

        Ok(std::mem::take(&mut self.wire_values))
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<u16, ComputeError> {
        let output_value = match &instruction.expression {
            Expression::Assignment(operand) => self.get_operand_value(&operand)?,
            Expression::Not(operand) => !self.get_operand_value(&operand)?,
            Expression::And { lhs, rhs } => 
                self.get_operand_value(&lhs)? & self.get_operand_value(&rhs)?,
            Expression::Or { lhs, rhs } => 
                self.get_operand_value(&lhs)? | self.get_operand_value(&rhs)?,
            Expression::LShift { lhs, rhs } => 
                self.get_operand_value(&lhs)? << self.get_operand_value(&rhs)?,
            Expression::RShift { lhs, rhs } => 
                self.get_operand_value(&lhs)? >> self.get_operand_value(&rhs)?,
        };

        self.wire_values
            .insert(instruction.output.clone(), output_value);
        Ok(output_value)
    }

    fn get_operand_value(&self, operand: &Operand) -> Result<u16, ComputeError> {
        match operand {
            Operand::Constant(value) => Ok(*value),
            Operand::Wire(wire) => self
                .wire_values
                .get(wire)
                .map(|value| *value)
                .ok_or(format!(
                    "tried to use value of {} before it was computed",
                    wire
                )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn instruction_parsing() {
        assert_eq!(
            "a AND b -> c".parse::<Instruction>().unwrap(),
            Instruction {
                expression: Expression::And {
                    lhs: Operand::Wire("a".into()),
                    rhs: Operand::Wire("b".into())
                },
                output: "c".into()
            }
        );
        assert_eq!(
            "12 LSHIFT asd -> out".parse::<Instruction>().unwrap(),
            Instruction {
                expression: Expression::LShift {
                    lhs: Operand::Constant(12),
                    rhs: Operand::Wire("asd".into())
                },
                output: "out".into()
            }
        );
        assert_eq!(
            "123 OR 111 -> a".parse::<Instruction>().unwrap(),
            Instruction {
                expression: Expression::Or {
                    lhs: Operand::Constant(123),
                    rhs: Operand::Constant(111)
                },
                output: "a".into()
            }
        );
        assert_eq!(
            "NOT 111 -> b".parse::<Instruction>().unwrap(),
            Instruction {
                expression: Expression::Not(Operand::Constant(111)),
                output: "b".into()
            }
        );
        assert_eq!(
            "NOT hi -> bye".parse::<Instruction>().unwrap(),
            Instruction {
                expression: Expression::Not(Operand::Wire("hi".into())),
                output: "bye".into()
            }
        );
        assert_eq!(
            "a -> b".parse::<Instruction>().unwrap(),
            Instruction {
                expression: Expression::Assignment(Operand::Wire("a".into())),
                output: "b".into()
            }
        );
        assert_eq!(
            "12 -> c".parse::<Instruction>().unwrap(),
            Instruction {
                expression: Expression::Assignment(Operand::Constant(12)),
                output: "c".into()
            }
        );
    }

    #[test]
    fn dependencies() {
        assert!("a AND b -> c"
            .parse::<Instruction>()
            .unwrap()
            .incoming_wires()
            .eq(["a", "b"].into_iter()));
        assert!("12 LSHIFT b -> c"
            .parse::<Instruction>()
            .unwrap()
            .incoming_wires()
            .eq(["b"].into_iter()));
        assert_eq!(
            "12 OR 1 -> c"
                .parse::<Instruction>()
                .unwrap()
                .incoming_wires()
                .count(),
            0
        );
        assert!("NOT a -> c"
            .parse::<Instruction>()
            .unwrap()
            .incoming_wires()
            .eq(["a"].into_iter()));
        assert_eq!(
            "12 -> c"
                .parse::<Instruction>()
                .unwrap()
                .incoming_wires()
                .count(),
            0
        );
    }

    fn assert_topologically_sorted(instructions: &[Instruction]) {
        let mut encountered = HashSet::<&str>::new();
        for instruction in instructions {
            for dependency in instruction.incoming_wires() {
                assert!(
                    encountered.contains(dependency),
                    "unsatisfied dependency {}",
                    dependency
                );
            }
            encountered.insert(&instruction.output);
        }
    }

    #[test]
    fn sorting() {
        for input in [
            ["1 -> a", "b -> c", "a -> b"],
            ["b AND a -> xyz", "a -> b", "NOT 3 -> a"],
        ] {
            let mut instructions: Vec<Instruction> =
                input.into_iter().map(|s| s.parse().unwrap()).collect();
            instructions.sort_topologically();
            assert_topologically_sorted(&instructions);
        }
    }

    #[test]
    fn emulator_execute() {
        let instructions: Vec<Instruction> = [
            "5 -> a",
            "3 -> b",
            "a AND b -> c",
            "1 -> d",
            "2 -> e",
            "d OR e -> f",
            "f LSHIFT c -> g",
        ].into_iter().map(|s| s.parse().unwrap()).collect();

        let values = Emulator::new().execute_instructions(&instructions).unwrap();

        assert_eq!(*values.get("g").unwrap(), 6);
    }
}
