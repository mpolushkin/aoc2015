use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;
use std::str::FromStr;

use super::Challenge;

pub struct Day19 {
    replacements: Vec<Replacement>,
    input_molecule: String,
}

impl Challenge for Day19 {
    const DAY: u8 = 19;

    type Part1Solution = usize;
    type Part2Solution = usize;

    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let replacements: Vec<_> = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.parse::<Replacement>().unwrap())
            .collect();
        let input_molecule = lines
            .next()
            .expect("expected input molecule following blank line")
            .to_owned();
        Self {
            replacements,
            input_molecule,
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        let machine = Machine::with_replacements(self.replacements.clone());
        machine.calibrate(self.input_molecule.clone()).len()
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        let machine = Machine::with_replacements(self.replacements.clone());
        machine
            .optimal_recipe_a_star(self.input_molecule.clone())
            .expect("no valid recipes")
        // machine
        //     .optimal_recipe_len(&self.input_molecule)
        //     .expect("no valid recipes")
        // machine
        //     .optimal_recipe(self.input_molecule.clone())
        //     .expect("no valid recipes")
        //     .len()
    }
}

fn molecule_length(molecule: &str) -> usize {
    molecule
        .bytes()
        .filter(|&byte| byte.is_ascii_uppercase() || byte == b'e')
        .count()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Replacement {
    pattern: String,
    result: String,
}

impl Replacement {
    fn molecule_diff(&self) -> usize {
        molecule_length(&self.result) - molecule_length(&self.pattern)
    }
}

type Error = String;

impl FromStr for Replacement {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pattern, result) = s.split_once(" => ").ok_or_else(|| "expected \" => \"")?;
        Ok(Self {
            pattern: pattern.to_owned(),
            result: result.to_owned(),
        })
    }
}

struct Machine {
    replacements: Vec<Replacement>,
}

impl Machine {
    fn new() -> Self {
        Self {
            replacements: Vec::new(),
        }
    }

    fn with_replacements(replacements: impl IntoIterator<Item = Replacement>) -> Self {
        let mut self_ = Self::new();
        for replacement in replacements.into_iter() {
            self_.add_replacement(replacement);
        }
        self_
    }

    fn add_replacement(&mut self, replacement: Replacement) {
        self.replacements.push(replacement);
    }

    fn calibrate(&self, input: String) -> HashSet<String> {
        PossibleTransformations::new(&self.replacements, input, Direction::Forward)
            .unique_molecules()
            .map(|(output, _)| output)
            .collect()
    }

    fn recipes<'a>(&'a self, target: String) -> Recipes<'a> {
        Recipes::new(&self.replacements, target)
    }

    fn optimal_recipe(&self, target: String) -> Option<Vec<TransformationInfo>> {
        // FIXME: this doesn't compute the optimal recipe, just the first recipe it finds!
        Some(
            self.recipes(target.clone())
                .inspect(|solution| print_solution(&target, solution, &self.replacements))
                .inspect(|solution| println!("Solution length: {}", solution.len()))
                .next()?, // .min_by_key(|recipe| recipe.len())?,
        )
    }

    fn optimal_recipe_a_star(&self, target: String) -> Option<usize> {
        RecipeFinder::new(target, &self.replacements).find_shortest_path()
    }

    fn optimal_recipe_len(&self, target: &str) -> Option<usize> {
        if target == "e" {
            return Some(0);
        }
        let mut outputs = HashSet::new();
        outputs.insert(target.to_owned());

        for i in 1.. {
            println!("{}", i);
            outputs = outputs
                .into_iter()
                .flat_map(|input| {
                    PossibleTransformations::new(&self.replacements, input, Direction::Reverse)
                })
                .scan(false, |recipe_found, (candidate, _)| {
                    if *recipe_found {
                        None
                    } else {
                        if candidate == "e" {
                            *recipe_found = true;
                        }
                        Some(candidate)
                    }
                })
                .collect();
            if outputs.is_empty() {
                return None;
            } else if outputs.contains("e") {
                return Some(i);
            }
        }
        None
    }
}

fn print_solution(target: &str, solution: &Vec<TransformationInfo>, replacements: &[Replacement]) {
    println!("Solution:");
    println!("  e");
    let mut current = "e".to_owned();
    for info in solution {
        let replacement = &replacements[info.replacement_index];
        println!(
            "> {}{} => {}",
            " ".repeat(info.input_index),
            replacement.pattern,
            replacement.result,
        );
        current =
            try_replacement(&current, info.input_index, &replacement, Direction::Forward).unwrap();
        println!("  {}", current);
    }
    println!("= {}", target);
}

#[derive(Clone, Copy)]
enum Direction {
    Forward,
    Reverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TransformationInfo {
    replacement_index: usize,
    input_index: usize,
}

struct PossibleTransformations<'a> {
    replacements: &'a [Replacement],
    input: String,
    direction: Direction,
    replacements_cursor: usize,
    input_cursor: usize,
}

impl<'a> PossibleTransformations<'a> {
    fn new(replacements: &'a [Replacement], input: String, direction: Direction) -> Self {
        Self {
            replacements,
            input,
            direction,
            replacements_cursor: 0,
            input_cursor: 0,
        }
    }

    fn next_for_current_input_cursor(&mut self) -> Option<(String, TransformationInfo)> {
        loop {
            if self.replacements_cursor >= self.replacements.len() {
                return None;
            }

            let output = self.try_replacement();
            self.replacements_cursor += 1;
            if output.is_some() {
                return output;
            }
        }
    }

    fn try_replacement(&mut self) -> Option<(String, TransformationInfo)> {
        let output = try_replacement(
            &self.input,
            self.input_cursor,
            &self.replacements[self.replacements_cursor],
            self.direction,
        )?;
        Some((
            output,
            TransformationInfo {
                replacement_index: self.replacements_cursor,
                input_index: self.input_cursor,
            },
        ))
    }

    fn unique_molecules(self) -> impl Iterator<Item = (String, TransformationInfo)> {
        self.collect::<HashMap<String, TransformationInfo>>()
            .into_iter()
    }
}

fn try_replacement(
    input: &str,
    position: usize,
    replacement: &Replacement,
    direction: Direction,
) -> Option<String> {
    let (pattern, result) = match direction {
        Direction::Forward => (&replacement.pattern, &replacement.result),
        Direction::Reverse => (&replacement.result, &replacement.pattern),
    };
    if input[position..].starts_with(pattern) {
        let mut output = input.to_owned();
        output.replace_range(position..position + pattern.len(), result);
        Some(output)
    } else {
        None
    }
}

impl<'a> Iterator for PossibleTransformations<'a> {
    type Item = (String, TransformationInfo);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.input_cursor >= self.input.len() {
                return None;
            }
            match self.next_for_current_input_cursor() {
                Some(output) => return Some(output),
                None => {
                    self.input_cursor += 1;
                    self.replacements_cursor = 0;
                }
            }
        }
    }
}

struct Recipes<'a> {
    replacements: &'a Vec<Replacement>,
    target: Option<String>,
    stack: Vec<(PossibleTransformations<'a>, TransformationInfo)>,
    dead_ends: HashSet<String>,
}

impl<'a> Recipes<'a> {
    const ELECTRON: &'static str = "e";

    fn new(replacements: &'a Vec<Replacement>, target: String) -> Self {
        Self {
            replacements,
            target: Some(target),
            stack: Vec::new(),
            dead_ends: HashSet::new(),
        }
    }

    fn next_candidate(&mut self) -> Option<String> {
        loop {
            match self.stack.last_mut() {
                None => return self.target.take(),
                Some(&mut (ref mut all_transformations, ref mut current_transformation_info)) => {
                    if let Some((candidate_molecule, transformation_info)) =
                        all_transformations.next()
                    {
                        *current_transformation_info = transformation_info;
                        return Some(candidate_molecule);
                    }
                }
            }
            let (all_transformations, _) = self.stack.pop().unwrap();
            self.dead_ends.insert(all_transformations.input);
        }
    }

    fn recipe_from_stack(&self) -> Vec<TransformationInfo> {
        self.stack
            .iter()
            .rev()
            .map(|(_, transformation_info)| *transformation_info)
            .collect()
    }

    fn push(&mut self, new_target: String) {
        self.stack.push((
            self.possible_transformations(new_target),
            TransformationInfo {
                replacement_index: 0,
                input_index: 0,
            },
        ))
    }

    fn next_valid_recipe(&mut self) -> Option<Vec<TransformationInfo>> {
        while let Some(candidate_molecule) = self.next_candidate() {
            if candidate_molecule == Self::ELECTRON {
                let recipe = self.recipe_from_stack();
                return Some(recipe);
            } else if !self.dead_ends.contains(&candidate_molecule) {
                self.push(candidate_molecule);
            }
        }
        None
    }

    fn possible_transformations(&self, string: String) -> PossibleTransformations<'a> {
        PossibleTransformations::new(self.replacements, string, Direction::Reverse)
    }
}

impl<'a> Iterator for Recipes<'a> {
    type Item = Vec<TransformationInfo>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_valid_recipe()
    }
}

#[derive(PartialEq, Eq)]
struct NodeWithDistanceThrough {
    node: Rc<String>,
    distance_through: usize,
}

impl Ord for NodeWithDistanceThrough {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse ordering on distance_through to turn BinaryHeap into a min-heap
        other
            .distance_through
            .cmp(&self.distance_through)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for NodeWithDistanceThrough {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct RecipeFinder<'a> {
    replacements: &'a [Replacement],
    max_diff_per_step: usize,
    node_distances_to: HashMap<Rc<String>, usize>,
    node_distances_through: HashMap<Rc<String>, usize>,
    unvisited: BinaryHeap<NodeWithDistanceThrough>,
}

impl<'a> RecipeFinder<'a> {
    const ELECTRON: &'static str = "e";

    fn new(target: String, replacements: &'a [Replacement]) -> Self {
        let max_diff_per_step = replacements
            .into_iter()
            .map(|replacement| replacement.molecule_diff())
            .max()
            .expect("replacement list is empty");
        let mut self_ = Self {
            replacements,
            max_diff_per_step,
            node_distances_to: HashMap::new(),
            node_distances_through: HashMap::new(),
            unvisited: BinaryHeap::new(),
        };
        self_.register_node(target, 0);
        self_
    }

    fn register_node(&mut self, node: String, distance_to: usize) {
        let node = Rc::new(node);
        self.node_distances_to.insert(node.clone(), distance_to);
        let distance_through = distance_to + self.estimate_distance_from(&node);
        self.node_distances_through
            .insert(node.clone(), distance_through);
        self.unvisited.push(NodeWithDistanceThrough {
            node: node.clone(),
            distance_through,
        })
    }

    fn distance_to(&self, node: &String) -> usize {
        self.node_distances_to
            .get(node)
            .copied()
            .unwrap_or(usize::MAX)
    }

    fn distance_through(&self, node: &String) -> usize {
        self.node_distances_through
            .get(node)
            .copied()
            .unwrap_or(usize::MAX)
    }

    fn estimate_distance_from(&self, node: &String) -> usize {
        let needed_diff = molecule_length(node).saturating_sub(1);
        let remainder = needed_diff % self.max_diff_per_step;
        needed_diff / self.max_diff_per_step + if remainder > 0 { 1 } else { 0 }
    }

    fn find_shortest_path(mut self) -> Option<usize> {
        let mut max_distance_through = 0;
        while let Some(NodeWithDistanceThrough {
            node: current,
            distance_through: current_distance_through,
        }) = self.unvisited.pop()
        {
            let current_distance = self.distance_to(&current);

            if current_distance_through > max_distance_through {
                max_distance_through = current_distance_through;
                println!(
                    "distance_to: {:4}, distance_through: {:4} - {}",
                    current_distance, current_distance_through, current
                );
            }

            if *current == Self::ELECTRON {
                return Some(current_distance);
            }

            if self.distance_through(&current) < current_distance_through {
                continue; // we've already found a shorter path to this node
            }

            let neighbor_distance = current_distance + 1;
            for (neighbor, _) in PossibleTransformations::new(
                &self.replacements,
                (*current).clone(),
                Direction::Reverse,
            )
            .unique_molecules()
            {
                if neighbor_distance < self.distance_to(&neighbor) {
                    self.register_node(neighbor, neighbor_distance)
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            "Pattern => Result".parse(),
            Ok(Replacement {
                pattern: "Pattern".to_owned(),
                result: "Result".to_owned(),
            })
        )
    }

    #[test]
    fn test_machine_calibration() {
        let machine = Machine::with_replacements(
            ["H => HO", "H => OH", "O => HH"]
                .into_iter()
                .map(|line| line.parse::<Replacement>().unwrap()),
        );

        assert_eq!(
            machine.calibrate("HOH".to_owned()),
            ["HOOH", "HOHO", "OHOH", "HHHH"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect()
        );

        assert_eq!(machine.calibrate("HOHOHO".to_owned()).len(), 7);
    }

    #[test]
    fn test_machine_optimal_recipe() {
        let machine = Machine::with_replacements(
            ["e => H", "e => O", "H => HO", "H => OH", "O => HH"]
                .into_iter()
                .map(|line| line.parse::<Replacement>().unwrap()),
        );

        assert_eq!(machine.optimal_recipe("".to_owned()), None);
        assert_eq!(machine.optimal_recipe("e".to_owned()).unwrap().len(), 0);
        assert_eq!(machine.optimal_recipe("H".to_owned()).unwrap().len(), 1);
        assert_eq!(machine.optimal_recipe("HOH".to_owned()).unwrap().len(), 3);
        assert_eq!(
            machine.optimal_recipe("HOHOHO".to_owned()).unwrap().len(),
            6
        );
        assert_eq!(machine.optimal_recipe_len(""), None);
        assert_eq!(machine.optimal_recipe_len("e").unwrap(), 0);
        assert_eq!(machine.optimal_recipe_len("H").unwrap(), 1);
        assert_eq!(machine.optimal_recipe_len("HOH").unwrap(), 3);
        assert_eq!(machine.optimal_recipe_len("HOHOHO").unwrap(), 6);

        assert_eq!(machine.optimal_recipe_a_star("".to_owned()), None);
        assert_eq!(machine.optimal_recipe_a_star("e".to_owned()).unwrap(), 0);
        assert_eq!(machine.optimal_recipe_a_star("H".to_owned()).unwrap(), 1);
        assert_eq!(machine.optimal_recipe_a_star("HOH".to_owned()).unwrap(), 3);
        assert_eq!(machine.optimal_recipe_a_star("HOHOHO".to_owned()).unwrap(), 6);
    }

    #[test]
    fn test_molecule_length() {
        assert_eq!(molecule_length(""), 0);
        assert_eq!(molecule_length("e"), 1);
        assert_eq!(molecule_length("H"), 1);
        assert_eq!(molecule_length("Mg"), 1);
        assert_eq!(molecule_length("CRnFAr"), 4);

        assert_eq!(
            "Al => ThF".parse::<Replacement>().unwrap().molecule_diff(),
            1
        );
        assert_eq!(
            "H => CRnFYFYFAr"
                .parse::<Replacement>()
                .unwrap()
                .molecule_diff(),
            7
        );
        assert_eq!(
            "e => OMg".parse::<Replacement>().unwrap().molecule_diff(),
            1
        );
    }
}
