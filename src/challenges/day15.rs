use std::{
    cmp::max,
    iter::Sum,
    ops::{Add, Mul},
    str::FromStr,
};

use super::Challenge;

pub struct Day15 {
    recipe_optimizer: RecipeOptimizer,
}

impl Challenge for Day15 {
    const DAY: u8 = 15;

    type Part1Solution = u32;

    type Part2Solution = u32;

    fn new(input: &str) -> Self {
        Self {
            recipe_optimizer: RecipeOptimizer::with_ingredients(
                input
                    .lines()
                    .map(|line| line.parse::<Ingredient>().unwrap()),
            ),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        self.recipe_optimizer.optimal_recipe_score()
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        self.recipe_optimizer.optimal_recipe_score_with_calories(500)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Ingredient {
    name: String,
    properties: Properties,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Properties {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Properties {
    fn score(&self) -> u32 {
        [self.capacity, self.durability, self.flavor, self.texture]
            .into_iter()
            .map(|value| max(value, 0) as u32)
            .product()
    }
}

impl Add for Properties {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Properties {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl Sum for Properties {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            Properties {
                capacity: 0,
                durability: 0,
                flavor: 0,
                texture: 0,
                calories: 0,
            },
            |acc, item| acc + item,
        )
    }
}

impl Mul<Properties> for i32 {
    type Output = Properties;

    fn mul(self, rhs: Properties) -> Self::Output {
        Properties {
            capacity: self * rhs.capacity,
            durability: self * rhs.durability,
            flavor: self * rhs.flavor,
            texture: self * rhs.texture,
            calories: self * rhs.calories,
        }
    }
}

struct RecipeOptimizer {
    ingredients: Vec<Ingredient>,
}

impl RecipeOptimizer {
    fn with_ingredients(ingredients: impl IntoIterator<Item = Ingredient>) -> Self {
        Self {
            ingredients: ingredients.into_iter().collect(),
        }
    }

    fn optimal_recipe_score(&self) -> u32 {
        AllPossibleMixes::new(100, self.ingredients.len())
            .map(|mix| self.recipe_properties(mix).score())
            .max()
            .expect("no valid recipes")
    }

    fn optimal_recipe_score_with_calories(&self, expected: i32) -> u32 {
        AllPossibleMixes::new(100, self.ingredients.len())
            .map(|mix| self.recipe_properties(mix))
            .filter_map(|properties| {
                if properties.calories == expected {
                    Some(properties.score())
                } else {
                    None
                }
            })
            .max()
            .expect("no valid recipes")
    }

    fn recipe_properties(&self, mix: Vec<u32>) -> Properties {
        assert_eq!(self.ingredients.len(), mix.len());
        mix.iter()
            .zip(self.ingredients.iter())
            .map(|(&quantity, ingredient)| quantity as i32 * ingredient.properties)
            .sum()
    }
}

type Error = String;
type Result<T> = std::result::Result<T, Error>;

impl FromStr for Ingredient {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (name, properties) = s.split_once(": ").ok_or_else(|| "no \": \" in input")?;
        Ok(Self {
            name: name.trim().to_owned(),
            properties: properties.trim().parse::<Properties>()?,
        })
    }
}

impl FromStr for Properties {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut properties = s.split(", ");
        Ok(Self {
            capacity: parse_property(&mut properties, "capacity")?,
            durability: parse_property(&mut properties, "durability")?,
            flavor: parse_property(&mut properties, "flavor")?,
            texture: parse_property(&mut properties, "texture")?,
            calories: parse_property(&mut properties, "calories")?,
        })
    }
}

fn parse_property<'a>(
    properties: &mut impl Iterator<Item = &'a str>,
    expected: &str,
) -> Result<i32> {
    let (name, value) = properties
        .next()
        .ok_or_else(|| format!("input ended while expecting property \"{}\"", expected))?
        .split_once(' ')
        .ok_or_else(|| "expected property name and value to be separated by space")?;

    if name == expected {
        Ok(value
            .parse::<i32>()
            .map_err(|_| format!("could not parse value for property \"{}\"", expected))?)
    } else {
        Err(format!(
            "expected property \"{}\", got \"{}\"",
            expected, name
        ))
    }
}

struct AllPossibleMixes {
    total: u32,
    independent_ingredients: Option<Vec<u32>>,
}

impl AllPossibleMixes {
    fn new(total: u32, num_ingredients: usize) -> Self {
        Self {
            total,
            independent_ingredients: Some(vec![0; num_ingredients - 1]),
        }
    }

    fn get(&self) -> Option<Vec<u32>> {
        let independent_ingredients = self.independent_ingredients.as_ref()?;
        let dependent = self.total - independent_ingredients.iter().copied().sum::<u32>();
        Some(Vec::from_iter(
            independent_ingredients
                .iter()
                .copied()
                .chain(std::iter::once(dependent)),
        ))
    }

    fn advance(&mut self) {
        if let Some(ref mut independent_ingredients) = self.independent_ingredients {
            for i in (0..independent_ingredients.len()).rev() {
                independent_ingredients[i] += 1;
                if independent_ingredients.iter().copied().sum::<u32>() <= self.total {
                    return;
                } else {
                    independent_ingredients[i] = 0;
                }
            }
            self.independent_ingredients = None;
        }
    }
}

impl Iterator for AllPossibleMixes {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.get();
        self.advance();
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8".parse(),
            Ok(Ingredient {
                name: "Butterscotch".to_owned(),
                properties: Properties {
                    capacity: -1,
                    durability: -2,
                    flavor: 6,
                    texture: 3,
                    calories: 8
                }
            })
        )
    }

    #[test]
    fn test_all_possible_mixes() {
        assert!(AllPossibleMixes::new(1, 1).eq([[1]]));
        assert!(AllPossibleMixes::new(2, 1).eq([[2]]));
        assert!(AllPossibleMixes::new(2, 2).eq([[0, 2], [1, 1], [2, 0]]));
        assert!(AllPossibleMixes::new(2, 3).eq([
            [0, 0, 2],
            [0, 1, 1],
            [0, 2, 0],
            [1, 0, 1],
            [1, 1, 0],
            [2, 0, 0]
        ]));
        assert!(AllPossibleMixes::new(4, 3).eq([
            [0, 0, 4],
            [0, 1, 3],
            [0, 2, 2],
            [0, 3, 1],
            [0, 4, 0],
            [1, 0, 3],
            [1, 1, 2],
            [1, 2, 1],
            [1, 3, 0],
            [2, 0, 2],
            [2, 1, 1],
            [2, 2, 0],
            [3, 0, 1],
            [3, 1, 0],
            [4, 0, 0],
        ]));
    }

    #[test]
    fn test_property_arithmetic() {
        assert_eq!(
            Properties {
                capacity: 1,
                durability: 2,
                flavor: 0,
                texture: -3,
                calories: 5
            } + Properties {
                capacity: 0,
                durability: -2,
                flavor: 0,
                texture: -3,
                calories: 4
            },
            Properties {
                capacity: 1,
                durability: 0,
                flavor: 0,
                texture: -6,
                calories: 9
            }
        );

        assert_eq!(
            3 * Properties {
                capacity: 1,
                durability: 2,
                flavor: 0,
                texture: -3,
                calories: 5
            },
            Properties {
                capacity: 3,
                durability: 6,
                flavor: 0,
                texture: -9,
                calories: 15
            }
        )
    }

    #[test]
    fn test_property_score() {
        assert_eq!(
            Properties {
                capacity: 44 * -1 + 56 * 2,
                durability: 44 * -2 + 56 * 3,
                flavor: 44 * 6 + 56 * -2,
                texture: 44 * 3 + 56 * -1,
                calories: 12345 // not relevant for score
            }
            .score(),
            62842880
        )
    }

    #[test]
    fn test_recipe_optimizer() {
        let optimizer = RecipeOptimizer::with_ingredients(
            [
                "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
                "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
            ]
            .into_iter()
            .map(|line| line.parse::<Ingredient>().unwrap()),
        );

        assert_eq!(optimizer.optimal_recipe_score(), 62842880);
        assert_eq!(optimizer.optimal_recipe_score_with_calories(500), 57600000);
    }
}
