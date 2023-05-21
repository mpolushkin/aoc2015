use super::Challenge;

pub struct Day11 {
    input: String,
}

impl Challenge for Day11 {
    const DAY: u8 = 11;

    type Part1Solution = String;

    type Part2Solution = String;

    fn new(input: &str) -> Self {
        Self {
            input: input.trim().to_owned(),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        let mut password = Password::new(&self.input).unwrap();
        password.increment_until_valid();
        password.as_str().to_owned()
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        let mut password = Password::new(&self.input).unwrap();
        password.increment_until_valid();
        password.increment_until_valid();
        password.as_str().to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Password {
    inner: [u8; 8],
}

impl<'a> Password {
    fn new(input: &str) -> Result<Self, String> {
        let inner: [u8; 8] = input
            .as_bytes()
            .try_into()
            .map_err(|_| "password must be 8 characters long".to_owned())?;
        if inner.into_iter().all(|byte| (b'a'..=b'z').contains(&byte)) {
            Ok(Self { inner })
        } else {
            Err("password may contain only lowercase ASCII letters".to_owned())
        }
    }

    fn as_str(&'a self) -> &'a str {
        std::str::from_utf8(&self.inner).unwrap()
    }

    fn increment(&mut self) {
        for byte in self.inner.iter_mut().rev() {
            match byte {
                b'z' => {
                    *byte = b'a';
                }
                _ => {
                    *byte += 1;
                    break;
                }
            }
        }
    }

    fn increment_until_valid(&mut self) {
        loop {
            self.increment();
            if self.is_valid() {
                break;
            }
        }
    }

    fn is_valid(&self) -> bool {
        self.contains_straight_of_3()
            && !self.contains_illegal_characters()
            && self.contains_2_pairs()
    }

    fn contains_illegal_characters(&self) -> bool {
        self.inner.contains(&b'i') || self.inner.contains(&b'o') || self.inner.contains(&b'l')
    }

    fn contains_straight_of_3(&self) -> bool {
        let mut current_straight: u8 = 1;
        for pair in self.inner.windows(2) {
            if pair[1] == pair[0] + 1 {
                current_straight += 1;
                if current_straight == 3 {
                    return true;
                }
            } else {
                current_straight = 1;
            }
        }
        false
    }

    fn contains_2_pairs(&self) -> bool {
        let mut num_pairs = 0usize;
        let mut last: Option<u8> = None;

        for &byte in self.inner.iter() {
            match last {
                None => last = Some(byte),
                Some(last_byte) => {
                    if byte == last_byte {
                        num_pairs += 1;
                        last = None;
                        if num_pairs == 2 {
                            return true;
                        }
                    } else {
                        last = Some(byte)
                    }
                }
            }
        }
        false
    }
}

impl std::cmp::PartialEq<&str> for Password {
    fn eq(&self, other: &&str) -> bool {
        &self.inner == other.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        assert!(Password::new("abcdefg").is_err());
        assert!(Password::new("abcdefghi").is_err());
        assert!(Password::new("abcdAfgh").is_err());
        assert!(Password::new("ab4defgh").is_err());
        assert_eq!(Password::new("abcdefgh").unwrap(), "abcdefgh");
    }

    #[test]
    fn test_increment() {
        let cases = [
            ("aaaaaaaa", "aaaaaaab"),
            ("asdfasdf", "asdfasdg"),
            ("abcdefgz", "abcdefha"),
            ("qwezzzzz", "qwfaaaaa"),
            ("zzzzzzzz", "aaaaaaaa"),
        ];
        for (initial, expected) in cases {
            let mut password = Password::new(initial).unwrap();
            password.increment();
            assert_eq!(
                password,
                expected,
                "expected: {:?}, got: {:?}",
                expected,
                password.as_str()
            )
        }
    }

    #[test]
    fn test_validation() {
        assert!(Password::new("aabbcdef").unwrap().is_valid());
        assert!(
            !Password::new("hijklmmn").unwrap().is_valid(),
            "must not contain i, o or l"
        );
        assert!(
            !Password::new("abbceffg").unwrap().is_valid(),
            "must contain straight of three increasing letters"
        );
        assert!(
            !Password::new("abbcdgjk").unwrap().is_valid(),
            "must contain two non-overlapping pairs"
        );
        assert!(
            !Password::new("abbbcdgj").unwrap().is_valid(),
            "must contain two *non-overlapping* pairs"
        );
    }

    #[test]
    fn test_increment_until_valid() {
        let cases = [("abcdefgh", "abcdffaa"), ("ghijklmn", "ghjaabcc")];
        for (initial, expected) in cases {
            let mut password = Password::new(initial).unwrap();
            password.increment_until_valid();
            assert_eq!(
                password,
                expected,
                "expected: {:?}, got: {:?}",
                expected,
                password.as_str()
            )
        }
    }
}
