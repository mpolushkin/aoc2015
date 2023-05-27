use std::str::FromStr;

use super::Challenge;

pub struct Day18 {
    initial_grid: Grid,
}

impl Challenge for Day18 {
    const DAY: u8 = 18;

    type Part1Solution = usize;
    type Part2Solution = usize;

    fn new(input: &str) -> Self {
        Self {
            initial_grid: input.parse::<Grid>().unwrap(),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        let mut lights = Lights::new(self.initial_grid.clone());
        lights.animate(100);
        lights.count_on()
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        let mut lights = Lights::new(self.initial_grid.clone());
        lights.set_corners_always_on();
        lights.animate(100);
        lights.count_on()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum LightState {
    On,
    Off,
}

impl LightState {
    fn is_on(&self) -> bool {
        *self == LightState::On
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    x_len: usize,
    y_len: usize,
    inner: Vec<LightState>,
}

impl Grid {
    fn new(x_len: usize, y_len: usize) -> Self {
        Self {
            x_len,
            y_len,
            inner: vec![LightState::Off; x_len * y_len],
        }
    }

    fn xy_to_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.x_len && y < self.y_len {
            Some(y * self.x_len + x)
        } else {
            None
        }
    }

    fn light_at(&self, x: usize, y: usize) -> Option<&LightState> {
        Some(&self.inner[self.xy_to_index(x, y)?])
    }

    fn light_at_mut(&mut self, x: usize, y: usize) -> Option<&mut LightState> {
        let index = self.xy_to_index(x, y)?;
        Some(&mut self.inner[index])
    }

    fn count_on(&self) -> usize {
        self.inner.iter().filter(|light| light.is_on()).count()
    }

    fn coordinates(&self) -> Coordinates {
        Coordinates::new(self.x_len, self.y_len)
    }

    fn neighbors(&self, x: usize, y: usize) -> Neighbors {
        Neighbors::new(&self, x, y)
    }

    fn turn_on_corners(&mut self) {
        for (x, y) in [
            (0, 0),
            (self.x_len - 1, 0),
            (0, self.y_len - 1),
            (self.x_len - 1, self.y_len - 1),
        ] {
            *self.light_at_mut(x, y).unwrap() = LightState::On;
        }
    }
}

struct Coordinates {
    x_len: usize,
    y_len: usize,
    cursor: usize,
}

impl Coordinates {
    fn new(x_len: usize, y_len: usize) -> Self {
        Self {
            x_len,
            y_len,
            cursor: 0,
        }
    }
}

impl Iterator for Coordinates {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.x_len * self.y_len {
            let x = self.cursor % self.x_len;
            let y = self.cursor / self.x_len;
            self.cursor += 1;
            Some((x, y))
        } else {
            None
        }
    }
}

struct Neighbors<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
    cursor: usize,
}

impl<'a> Neighbors<'a> {
    const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    fn new(grid: &'a Grid, x: usize, y: usize) -> Self {
        Self {
            grid,
            x,
            y,
            cursor: 0,
        }
    }

    fn xy_at_offset(&self, x_offset: isize, y_offset: isize) -> Option<(usize, usize)> {
        let x = self.x.checked_add_signed(x_offset)?;
        let y = self.y.checked_add_signed(y_offset)?;
        if x < self.grid.x_len && y < self.grid.y_len {
            Some((x, y))
        } else {
            None
        }
    }

    fn count_on(self) -> usize {
        let grid = self.grid;
        self.filter(|(x, y)| grid.light_at(*x, *y).expect("invalid coordinate").is_on())
            .count()
    }
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (x_offset, y_offset) = Self::NEIGHBOR_OFFSETS.get(self.cursor)?;
            self.cursor += 1;
            if let Some(xy) = self.xy_at_offset(*x_offset, *y_offset) {
                return Some(xy);
            }
        }
    }
}

struct Lights {
    current: Grid,
    next: Grid,
    corners_always_on: bool,
}

impl Lights {
    fn new(grid: Grid) -> Self {
        let x_len = grid.x_len;
        let y_len = grid.y_len;
        Self {
            current: grid,
            next: Grid::new(x_len, y_len),
            corners_always_on: false,
        }
    }

    fn set_corners_always_on(&mut self) {
        self.corners_always_on = true;
        self.current.turn_on_corners();
    }

    fn grid(&self) -> &Grid {
        &self.current
    }

    fn animate(&mut self, num_steps: u32) {
        for _ in 0..num_steps {
            self.step()
        }
    }

    fn step(&mut self) {
        for (x, y) in self.grid().coordinates() {
            let on_neighbors = self.current.neighbors(x, y).count_on();
            *self.next.light_at_mut(x, y).expect("invalid coordinates") =
                match self.current.light_at(x, y).expect("invalid coordinates") {
                    LightState::On => match on_neighbors {
                        2 | 3 => LightState::On,
                        _ => LightState::Off,
                    },
                    LightState::Off => match on_neighbors {
                        3 => LightState::On,
                        _ => LightState::Off,
                    },
                }
        }
        std::mem::swap(&mut self.next, &mut self.current);
        if self.corners_always_on {
            self.current.turn_on_corners();
        }
    }

    fn count_on(&self) -> usize {
        self.grid().count_on()
    }
}

type Error = String;
type Result<T> = std::result::Result<T, Error>;

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines: Vec<_> = s
            .lines()
            .map(|line| line.trim().as_bytes())
            .filter(|line| !line.is_empty())
            .collect();

        let y_len = lines.len();
        if y_len == 0 {
            return Err("empty grid".to_owned());
        }
        let x_len = lines[0].len();
        if !lines.iter().all(|line| line.len() == x_len) {
            return Err("lines vary in length".to_owned());
        }

        Ok(Grid {
            x_len: lines[0].len(),
            y_len: lines.len(),
            inner: lines
                .into_iter()
                .flatten()
                .map(|&byte| match byte {
                    b'.' => Ok(LightState::Off),
                    b'#' => Ok(LightState::On),
                    _ => Err(format!("invalid character: '{}'", byte)),
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    const INITIAL_STATE: &'static str = "
        .#.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####..
    ";

    #[test]
    fn test_parsing() {
        assert!("".parse::<Grid>().is_err());
        assert!("...\n..".parse::<Grid>().is_err());

        let rectangular_grid = ".#.\n##.".parse::<Grid>().unwrap();
        assert_eq!((rectangular_grid.x_len, rectangular_grid.y_len), (3, 2));

        // (0, 0) is top-left corner)
        assert!([(1, 0), (0, 1), (1, 1)]
            .into_iter()
            .all(|(x, y)| rectangular_grid.light_at(x, y).unwrap().is_on()));
        assert!([(0, 0), (2, 0), (2, 1)]
            .into_iter()
            .all(|(x, y)| !rectangular_grid.light_at(x, y).unwrap().is_on()));

        let grid = INITIAL_STATE.parse::<Grid>().unwrap();
        assert_eq!((grid.x_len, grid.y_len), (6, 6));
        assert_eq!(grid.count_on(), 15);
    }

    #[test]
    fn test_neighbors() {
        let grid = Grid::new(3, 4);

        assert_eq!(
            grid.neighbors(1, 1).collect::<HashSet<_>>(),
            [
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 2),
                (2, 2),
                (2, 1),
                (2, 0),
                (1, 0)
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(
            grid.neighbors(2, 3).collect::<HashSet<_>>(),
            [(1, 3), (1, 2), (2, 2)].into_iter().collect()
        );
        assert_eq!(
            grid.neighbors(1, 0).collect::<HashSet<_>>(),
            [(0, 0), (0, 1), (1, 1), (2, 1), (2, 0)]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn test_lights() {
        let mut lights = Lights::new(INITIAL_STATE.parse::<Grid>().unwrap());

        lights.animate(1);
        assert_eq!(
            lights.grid(),
            &"
            ..##..
            ..##.#
            ...##.
            ......
            #.....
            #.##..
            "
            .parse::<Grid>()
            .unwrap()
        );

        lights.animate(3);
        assert_eq!(
            lights.grid(),
            &"
            ......
            ......
            ..##..
            ..##..
            ......
            ......
            "
            .parse::<Grid>()
            .unwrap()
        );

        assert_eq!(lights.count_on(), 4);
    }

    #[test]
    fn test_lights_stuck_corners() {
        let mut lights = Lights::new(INITIAL_STATE.parse::<Grid>().unwrap());

        lights.set_corners_always_on();
        assert_eq!(
            *lights.grid(),
            "
            ##.#.#
            ...##.
            #....#
            ..#...
            #.#..#
            ####.#
            "
            .parse::<Grid>()
            .unwrap()
        );

        lights.animate(5);

        assert_eq!(
            lights.grid(),
            &"
            ##.###
            .##..#
            .##...
            .##...
            #.#...
            ##...#
            "
            .parse::<Grid>()
            .unwrap()
        );
        assert_eq!(lights.count_on(), 17);
    }
}
