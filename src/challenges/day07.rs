use std::str::FromStr;

use super::{Challenge, NotImplemented};

pub struct Day07 {
    instructions: Vec<Instruction>,
}

impl Challenge for Day07 {
    const DAY: u8 = 7;
    type Part1Solution = NotImplemented;
    type Part2Solution = NotImplemented;

    fn new(input: &str) -> Self {
        Day07 {
            instructions: input
                .lines()
                .map(|line| line.parse().expect(&format!("invalid line: {}", line)))
                .collect(),
        }
    }
    fn solve_part1(&self) -> Self::Part1Solution {
        NotImplemented {}
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

#[derive(Debug, PartialEq, Eq)]
enum Expression {
    Assignment(Operand),
    Not(Operand),
    And { lhs: Operand, rhs: Operand },
    Or { lhs: Operand, rhs: Operand },
    LShift { lhs: Operand, rhs: Operand },
    RShift { lhs: Operand, rhs: Operand },
}

#[derive(Debug, PartialEq, Eq)]
enum Operand {
    Constant(u16),
    Wire(String),
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
            return Ok(Expression::Assignment(lhs))
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

#[cfg(test)]
mod tests {
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
}
