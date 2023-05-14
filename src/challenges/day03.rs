use super::Challenge;
use std::collections::HashSet;

pub struct Day03 {
    list_of_directions: Vec<Direction>,
}

impl Day03 {
    fn count_visited_by_santa(&self) -> usize {
        visited_houses(&self.list_of_directions).len()
    }

    fn count_visited_by_santa_or_helper(&self) -> usize {
        let directions_for_santa = self.list_of_directions.iter().step_by(2);
        let directions_for_helper = self.list_of_directions.iter().skip(1).step_by(2);

        let visited_by_santa = visited_houses(directions_for_santa);
        let visited_by_helper = visited_houses(directions_for_helper);

        visited_by_santa.union(&visited_by_helper).count()
    }
}

fn visited_houses<'a, I>(directions: I) -> HashSet<Position>
where
    I: IntoIterator<Item = &'a Direction>,
{
    let mut position = Position { x: 0, y: 0 };
    let mut visited = HashSet::new();
    visited.insert(position);

    for direction in directions.into_iter() {
        match direction {
            Direction::North => position.y += 1,
            Direction::East => position.x += 1,
            Direction::South => position.y -= 1,
            Direction::West => position.x -= 1,
        };
        visited.insert(position);
    }

    visited
}

impl Challenge for Day03 {
    const DAY: u8 = 3;
    type Part1Solution = usize;
    type Part2Solution = usize;

    fn new(input: &str) -> Self {
        let list_of_directions: Vec<_> = input
            .trim()
            .chars()
            .map(|c| match c {
                '^' => Direction::North,
                '>' => Direction::East,
                'v' => Direction::South,
                '<' => Direction::West,
                _ => {
                    panic!("invalid character: {}", c);
                }
            })
            .collect();
        Self { list_of_directions }
    }
    fn solve_part1(&self) -> Self::Part1Solution {
        self.count_visited_by_santa()
    }
    fn solve_part2(&self) -> Self::Part2Solution {
        self.count_visited_by_santa_or_helper()
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day03::new(">").count_visited_by_santa(), 2);
        assert_eq!(Day03::new("^>v<").count_visited_by_santa(), 4);
        assert_eq!(Day03::new("^v^v^v^v^v").count_visited_by_santa(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day03::new("^v").count_visited_by_santa_or_helper(), 3);
        assert_eq!(Day03::new("^>v<").count_visited_by_santa_or_helper(), 3);
        assert_eq!(
            Day03::new("^v^v^v^v^v").count_visited_by_santa_or_helper(),
            11
        );
    }
}
