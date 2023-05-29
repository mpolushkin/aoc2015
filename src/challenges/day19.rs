use std::collections::HashSet;
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
            .optimal_recipe_len(&self.input_molecule)
            .expect("no valid recipes")
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Replacement {
    pattern: String,
    result: String,
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
    }
}
