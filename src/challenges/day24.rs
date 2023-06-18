use itertools::Itertools;

use super::Challenge;

pub struct Day24 {
    package_weights: Vec<u64>,
}

impl Challenge for Day24 {
    const DAY: u8 = 24;

    type Part1Solution = u64;
    type Part2Solution = u64;

    fn new(input: &str) -> Self {
        Self {
            package_weights: input
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .collect(),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        let distributor = PackageDistributor::new(&self.package_weights, 3);
        quantum_entanglement(
            &distributor
                .optimal_group_one()
                .expect("no valid group one found"),
        )
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        let distributor = PackageDistributor::new(&self.package_weights, 4);
        quantum_entanglement(
            &distributor
                .optimal_group_one()
                .expect("no valid group one found"),
        )
    }
}

struct PackageDistributor<'a> {
    package_weights: &'a [u64],
    num_groups: u64,
    group_weight: u64,
}

impl<'a> PackageDistributor<'a> {
    fn new(package_weights: &'a [u64], num_groups: u64) -> Self {
        Self {
            package_weights,
            num_groups,
            group_weight: {
                let total_weight: u64 = package_weights.into_iter().copied().sum();
                if total_weight % num_groups == 0 {
                    total_weight / num_groups
                } else {
                    panic!("total weight not divisible by {}", num_groups)
                }
            },
        }
    }

    fn optimal_group_one(&self) -> Option<Vec<u64>> {
        for i in 1..self.package_weights.len() {
            if let Some(group_one) = self.optimal_group_one_of_size(i) {
                return Some(group_one);
            }
        }
        None
    }

    fn optimal_group_one_of_size(&self, size: usize) -> Option<Vec<u64>> {
        self.all_valid_groups_one_of_size(size)
            .min_by_key(|group| quantum_entanglement(&group))
    }

    fn all_valid_groups_one_of_size(&self, size: usize) -> impl Iterator<Item = Vec<u64>> + '_ {
        self.all_ways_to_take_group_of_size(size, self.package_weights)
            .filter_map(|(group_one, remainder)| {
                // if self.can_take_valid_group(&remainder) {
                if self.can_take_n_valid_groups(&remainder, self.num_groups - 1) {
                    Some(group_one)
                } else {
                    None
                }
            })
    }

    fn can_take_n_valid_groups(&self, package_weights: &[u64], num_required_groups: u64) -> bool {
        if num_required_groups <= 1 {
            true
        } else {
            if let Some((_, remainder)) = self.take_valid_group(package_weights) {
                self.can_take_n_valid_groups(&remainder, num_required_groups - 1)
            } else {
                false
            }
        }
    }

    fn take_valid_group(&self, package_weights: &[u64]) -> Option<(Vec<u64>, Vec<u64>)> {
        (1..package_weights.len())
            .flat_map(|size| self.all_ways_to_take_group_of_size(size, package_weights))
            .next()
    }

    fn can_take_valid_group(&self, package_weights: &[u64]) -> bool {
        self.take_valid_group(package_weights).is_some()
    }

    /// Returns an iterator where each element is a tuple of a vector containing the selected
    /// elements (which sum to group_weight) and a vector containing the remaining elements
    fn all_ways_to_take_group_of_size(
        &'a self,
        size: usize,
        package_weights: &'a [u64],
    ) -> impl Iterator<Item = (Vec<u64>, Vec<u64>)> + 'a {
        (0..package_weights.len())
            .combinations(size)
            .map(|group_indices| take_selection(package_weights, group_indices))
            .filter(|(group, _)| group.iter().copied().sum::<u64>() == self.group_weight)
    }
}

fn quantum_entanglement(group: &[u64]) -> u64 {
    group.into_iter().copied().product()
}

fn take_selection(
    available_elements: &[u64],
    mut selection_indices: Vec<usize>,
) -> (Vec<u64>, Vec<u64>) {
    let mut selected = Vec::with_capacity(selection_indices.len());
    let mut remainder = Vec::with_capacity(available_elements.len() - selection_indices.len());
    selection_indices.sort();
    let mut selection_indices = selection_indices.into_iter().peekable();
    for (i, element) in available_elements.into_iter().copied().enumerate() {
        if selection_indices.next_if_eq(&i).is_some() {
            selected.push(element);
        } else {
            remainder.push(element);
        }
    }
    (selected, remainder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_selection() {
        assert_eq!(
            take_selection(&[11, 12, 13, 36, 42, 17], vec![1, 2, 4]),
            (vec![12, 13, 42], vec![11, 36, 17])
        );
    }

    #[test]
    fn test_package_distributor() {
        let distributor = PackageDistributor::new(&[1, 2, 3, 4, 5, 7, 8, 9, 10, 11], 3);

        let mut optimal_group_one = distributor.optimal_group_one().unwrap();
        optimal_group_one.sort();
        assert_eq!(optimal_group_one, vec![9, 11])
    }
}
