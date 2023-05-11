pub static INPUT: &str = include_str!("../input/day01.txt");

pub fn part1(input: &str) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
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

pub fn part2(input: &str) -> usize {
    let mut floor = 0;
    for (c, i) in input.chars().zip(1..) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("(()(()("), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(")"), 1);
        assert_eq!(part2("()())"), 5);
    }
}
