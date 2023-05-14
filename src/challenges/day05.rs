use super::Challenge;

pub struct Day05 {
    lines: Vec<String>,
}

impl Day05 {
    fn count_lines_satisfying(&self, predicate: fn (&str) -> bool) -> usize {
        self.lines
            .iter()
            .map(|line| predicate(line))
            .filter(|is_nice| *is_nice)
            .count()
    }
}

impl Challenge for Day05 {
    const DAY: u8 = 5;
    type Part1Solution = usize;
    type Part2Solution = usize;

    fn new(input: &str) -> Self {
        let lines: Vec<String> = input.lines().map(|line| line.to_owned()).collect();
        Self { lines }
    }
    fn solve_part1(&self) -> Self::Part1Solution {
        self.count_lines_satisfying(is_nice_part1)
    }
    fn solve_part2(&self) -> Self::Part2Solution {
        self.count_lines_satisfying(is_nice_part2)
    }
}

fn is_nice_part1(input: &str) -> bool {
    has_double_letter(input) && has_at_least_three_vowels(input) && !has_naughty_string(input)
}

fn is_nice_part2(input: &str) -> bool {
    has_double_pair(input) && has_double_letter_separated_by_one(input)
}

fn has_double_letter(input: &str) -> bool {
    let window_of_2_iter = input.chars().zip(input.chars().skip(1));
    for (first, second) in window_of_2_iter {
        if first == second {
            return true;
        }
    }
    false
}

fn has_at_least_three_vowels(input: &str) -> bool {
    let mut num_vowels = 0usize;
    for c in input.chars() {
        if "aeiou".contains(c) {
            num_vowels += 1;
        }
    }
    num_vowels >= 3
}

fn has_naughty_string(input: &str) -> bool {
    for naughty_string in ["ab", "cd", "pq", "xy"] {
        if input.contains(naughty_string) {
            return true;
        }
    }
    false
}

fn has_double_letter_separated_by_one(input: &str) -> bool {
    let chars: Vec<_> = input.chars().collect();
    if chars.len() < 3 {
        return false;
    }

    for window in chars.windows(3) {
        if window[0] == window[2] {
            return true;
        }
    }
    false
}

fn has_double_pair(input: &str) -> bool {
    let chars: Vec<_> = input.chars().collect();
    if chars.len() < 4 {
        return false;
    }
    for i in 0..chars.len() - 3 {
        for j in i + 2..chars.len() - 1 {
            if chars[i..=i + 1] == chars[j..=j + 1] {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice_part1() {
        assert!(is_nice_part1("ugknbfddgicrmopn"));
        assert!(is_nice_part1("aaa"));
        assert!(!is_nice_part1("jchzalrnumimnmhp"));
        assert!(!is_nice_part1("haegwjzuvuyypxyu"));
        assert!(!is_nice_part1("dvszwmarrgswjxmb"))
    }

    #[test]
    fn test_is_nice_part2() {
        assert!(is_nice_part2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_part2("xxyxx"));
        assert!(!is_nice_part2("uurcxstgmygtbstg"));
        assert!(!is_nice_part2("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_has_double_pair() {
        assert!(!has_double_pair(""));
        assert!(has_double_pair("abab"));
        assert!(!has_double_pair("abbb"));
        assert!(has_double_pair("abcdab"));
        assert!(!has_double_pair("abcdbb"));
    }
}
