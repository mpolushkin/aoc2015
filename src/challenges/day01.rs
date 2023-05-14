use super::Challenge;

pub struct Day01 {
    input: String,
}

impl Challenge for Day01 {
    const DAY: u8 = 1;
    type Part1Solution = i32;
    type Part2Solution = usize;

    fn new(input: &str) -> Self {
        Self {
            input: input.trim().to_owned(),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        let mut floor = 0;
        for c in self.input.chars() {
            match c {
                '(' => {
                    floor += 1;
                }
                ')' => {
                    floor -= 1;
                }
                _ => panic!("invalid character"),
            }
        }
        floor
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        let mut floor = 0;
        for (c, i) in self.input.chars().zip(1..) {
            match c {
                '(' => {
                    floor += 1;
                }
                ')' => {
                    floor -= 1;
                }
                _ => {
                    panic!("invalid character");
                }
            }
            if floor < 0 {
                return i;
            }
        }
        panic!("never entered basement");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day01::new("(())").solve_part1(), 0);
        assert_eq!(Day01::new("()()").solve_part1(), 0);
        assert_eq!(Day01::new("(()(()(").solve_part1(), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day01::new(")").solve_part2(), 1);
        assert_eq!(Day01::new("()())").solve_part2(), 5);
    }
}
