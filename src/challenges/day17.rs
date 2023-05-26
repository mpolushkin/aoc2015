use std::cmp::Ordering;

use super::Challenge;

pub struct Day17 {
    available_items: Vec<u32>,
}

impl Challenge for Day17 {
    const DAY: u8 = 17;

    type Part1Solution = usize;
    type Part2Solution = usize;

    fn new(input: &str) -> Self {
        Self {
            available_items: input
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .collect(),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        AllPossibleDistributions::new(150, &self.available_items).count()
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        let all_possible_distributions: Vec<_> =
            AllPossibleDistributions::new(150, &self.available_items).collect();
        let min_length = all_possible_distributions
            .iter()
            .map(|distribution| distribution.len())
            .min()
            .expect("no possible solutions");
        all_possible_distributions
            .iter()
            .filter(|distribution| distribution.len() == min_length)
            .count()
    }
}

#[derive(Debug)]
struct AllPossibleDistributions<'a> {
    total: u32,
    available_items: &'a Vec<u32>,
    current_indices: Vec<usize>,
}

impl<'a> AllPossibleDistributions<'a> {
    fn new(total: u32, available_items: &'a Vec<u32>) -> Self {
        Self {
            total,
            available_items,
            current_indices: vec![0; 1],
        }
    }

    fn is_index_valid(&self, index: usize) -> bool {
        index < self.available_items.len()
    }

    fn next_valid_index(&self) -> Option<usize> {
        let next_index = self.current_indices.last().map_or_else(|| 0, |i| i + 1);
        if self.is_index_valid(next_index) {
            Some(next_index)
        } else {
            None
        }
    }

    fn increment_last_item(&mut self) -> Option<()> {
        for _ in 0..self.current_indices.len() {
            if let Some(next_index) = self.next_valid_index() {
                *self
                    .current_indices
                    .last_mut()
                    .expect("no last item to increment") = next_index;
                return Some(());
            } else {
                self.current_indices.pop();
            }
        }
        None
    }

    fn append_item(&mut self) -> Option<()> {
        self.current_indices.push(self.next_valid_index()?);
        Some(())
    }

    fn current(&self) -> Option<Vec<u32>> {
        if self.current_indices.len() == 0 {
            None
        } else {
            Some(
                self.current_indices
                    .iter()
                    .copied()
                    .map(|i| self.available_items[i])
                    .collect(),
            )
        }
    }
}

impl<'a> Iterator for AllPossibleDistributions<'a> {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current = self.current()?;
            match current.iter().copied().sum::<u32>().cmp(&self.total) {
                Ordering::Equal => {
                    self.increment_last_item();
                    return Some(current);
                }
                Ordering::Less => self.append_item().or_else(|| self.increment_last_item())?,
                Ordering::Greater => self.increment_last_item()?,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_distributor() {
        assert_eq!(
            AllPossibleDistributions::new(25, &vec![20, 15, 10, 5, 5])
                .sorted()
                .collect_vec(),
            [vec![15, 10], vec![20, 5], vec![20, 5], vec![15, 5, 5]]
                .into_iter()
                .sorted()
                .collect_vec()
        )
    }
}
