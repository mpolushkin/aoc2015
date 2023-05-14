use super::Challenge;
use md5;

pub struct Day04 {
    miner: Miner,
}

impl Challenge for Day04 {
    const DAY: u8 = 4;
    type Part1Solution = i32;
    type Part2Solution = i32;

    fn new(input: &str) -> Self {
        Self {
            miner: Miner::new(input),
        }
    }
    fn solve_part1(&self) -> Self::Part1Solution {
        self.miner.mine(5)
    }
    fn solve_part2(&self) -> Self::Part2Solution {
        self.miner.mine(6)
    }
}

struct Miner {
    secret_key: String,
}

impl Miner {
    fn new(secret_key: &str) -> Self {
        Miner {
            secret_key: secret_key.trim().to_owned(),
        }
    }

    fn mine(&self, num_leading_zeros: usize) -> i32 {
        for i in 1.. {
            if self.answer_yields_digest_with_num_leading_zeros(i, num_leading_zeros) {
                return i;
            }
        }
        panic!("loop ran 0 times");
    }

    fn answer_yields_digest_with_num_leading_zeros(
        &self,
        answer: i32,
        num_leading_zeros: usize,
    ) -> bool {
        let digest = md5::compute(format!("{}{}", self.secret_key, answer));
        for c in format!("{:x}", digest).chars().take(num_leading_zeros) {
            if c != '0' {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_answer_satisfies_condition() {
        assert!(Miner::new("abcdef").answer_yields_digest_with_num_leading_zeros(609043, 5));
    }

    #[test]
    fn whitespace_surrounding_key_is_ignored() {
        assert!(Miner::new(
            "	abcdef 
                "
        )
        .answer_yields_digest_with_num_leading_zeros(609043, 5));
    }

    #[test]
    fn mine() {
        assert_eq!(Miner::new("pqrstuv").mine(5), 1048970)
    }
}
