use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

use super::Challenge;

pub struct Day09 {
    map: Map,
}

impl Challenge for Day09 {
    const DAY: u8 = 9;

    type Part1Solution = u32;
    type Part2Solution = u32;

    fn new(input: &str) -> Self {
        Self {
            map: Map::from_intercity_distances(
                input
                    .lines()
                    .map(|line| line.parse::<IntercityDistance>().unwrap()),
            ),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        self.map.find_shortest_route().unwrap()
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        self.map.find_longest_route().unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct IntercityDistance {
    city1: String,
    city2: String,
    distance: u32,
}

type ParseError = String;

impl FromStr for IntercityDistance {
    type Err = ParseError;

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split_whitespace().collect();
        if words.len() != 5 || words[1] != "to" || words[3] != "=" {
            return Err("invalid format".into());
        }

        let distance: u32 = words[4]
            .parse()
            .map_err(|_| format!("failed to parse distance: {}", words[4]))?;

        Ok(IntercityDistance {
            city1: words[0].to_owned(),
            city2: words[2].to_owned(),
            distance,
        })
    }
}

struct Map {
    distances: HashMap<String, HashMap<String, u32>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            distances: HashMap::new(),
        }
    }

    pub fn from_intercity_distances(
        distances: impl IntoIterator<Item = IntercityDistance>,
    ) -> Self {
        let mut map = Self::new();
        for distance in distances.into_iter() {
            map.add_intercity_distance(distance);
        }
        map
    }

    pub fn add_intercity_distance(&mut self, distance: IntercityDistance) {
        self.distances
            .entry(distance.city1.clone())
            .or_insert(HashMap::new())
            .insert(distance.city2.clone(), distance.distance);
        self.distances
            .entry(distance.city2)
            .or_insert(HashMap::new())
            .insert(distance.city1, distance.distance);
    }

    pub fn distance_between(&self, city1: &str, city2: &str) -> Option<u32> {
        self.distances
            .get(city1)
            .map(|inner| inner.get(city2))?
            .copied()
    }

    fn num_cities(&self) -> usize {
        self.distances.len()
    }

    fn cities(&self) -> impl Iterator<Item = &str> {
        self.distances.keys().map(|city| city as &str)
    }

    fn route_length(&self, cities: Vec<&str>) -> Option<u32> {
        Some(
            cities
                .windows(2)
                .map(|pair| self.distance_between(pair[0], pair[1]))
                .sum::<Option<u32>>()?,
        )
    }

    pub fn find_shortest_route(&self) -> Option<u32> {
        self.cities()
            .permutations(self.num_cities())
            .filter_map(|cities| self.route_length(cities))
            .min()
    }

    pub fn find_longest_route(&self) -> Option<u32> {
        self.cities()
            .permutations(self.num_cities())
            .filter_map(|cities| self.route_length(cities))
            .max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_parse() {
        assert_eq!(
            "Somewhere to SomewhereElse = 42"
                .parse::<IntercityDistance>()
                .unwrap(),
            IntercityDistance {
                city1: "Somewhere".into(),
                city2: "SomewhereElse".into(),
                distance: 42
            }
        );
    }

    #[test]
    fn test_map() {
        let map = Map::from_intercity_distances(
            ["London to Dublin = 464", "London to Belfast = 518"]
                .into_iter()
                .map(|s| s.parse::<IntercityDistance>().unwrap()),
        );

        assert_eq!(map.distance_between("London", "Dublin").unwrap(), 464);
        assert_eq!(map.distance_between("Dublin", "London").unwrap(), 464);
        assert_eq!(map.distance_between("Belfast", "London").unwrap(), 518);
        assert!(map.distance_between("Belfast", "Dublin").is_none());
        assert!(map.distance_between("a", "b").is_none());

        let all_cities: HashSet<_> = map.cities().collect();
        assert!(["London", "Dublin", "Belfast"]
            .into_iter()
            .all(|city| all_cities.contains(city)))
    }

    #[test]
    fn test_find_shortest_route() {
        let map = Map::from_intercity_distances(
            [
                "London to Dublin = 464",
                "London to Belfast = 518",
                "Dublin to Belfast = 141",
            ]
            .into_iter()
            .map(|s| s.parse::<IntercityDistance>().unwrap()),
        );

        assert_eq!(map.find_shortest_route().unwrap(), 605);
        assert_eq!(map.find_longest_route().unwrap(), 982);
    }
}
