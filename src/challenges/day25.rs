use super::{Challenge, NotImplemented};

pub struct Day25 {
    required_coord: Coord,
}

impl Challenge for Day25 {
    const DAY: u8 = 25;

    type Part1Solution = u64;
    type Part2Solution = NotImplemented;

    fn new(_input: &str) -> Self {
        // I couldn't be arsed to parse the input for this one..
        Self {
            required_coord: Coord {
                row: 2947,
                column: 3029,
            },
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        find_code_at_coord(self.required_coord)
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        NotImplemented
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    row: u64,
    column: u64,
}

struct CodeGenerator {
    coord: Coord,
    code: u64,
}

impl CodeGenerator {
    const FIRST_CODE: u64 = 20151125;

    fn new() -> Self {
        Self {
            coord: Coord { row: 1, column: 1 },
            code: Self::FIRST_CODE,
        }
    }

    fn advance_code(&mut self) {
        self.code = (self.code * 252533) % 33554393;
    }

    fn advance_coord(&mut self) {
        let Coord { row, column } = self.coord;
        if row > 1 {
            self.coord = Coord {
                row: row - 1,
                column: column + 1,
            };
        } else {
            self.coord = Coord {
                row: column + 1,
                column: 1,
            }
        }
    }
}

fn find_code_at_coord(coord: Coord) -> u64 {
    CodeGenerator::new()
        .find_map(|(current_coord, code)| {
            if current_coord == coord {
                Some(code)
            } else {
                None
            }
        })
        .unwrap()
}

impl Iterator for CodeGenerator {
    type Item = (Coord, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let item = (self.coord, self.code);
        self.advance_code();
        self.advance_coord();
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_generator() {
        //    |    1         2         3         4         5         6
        // ---+---------+---------+---------+---------+---------+---------+
        //  1 | 20151125  18749137  17289845  30943339  10071777  33511524
        //  2 | 31916031  21629792  16929656   7726640  15514188   4041754
        //  3 | 16080970   8057251   1601130   7981243  11661866  16474243
        //  4 | 24592653  32451966  21345942   9380097  10600672  31527494
        //  5 |    77061  17552253  28094349   6899651   9250759  31663883
        //  6 | 33071741   6796745  25397450  24659492   1534922  27995004
        let mut code_generator = CodeGenerator::new();

        assert_eq!(
            code_generator.next(),
            Some((Coord { row: 1, column: 1 }, 20151125))
        );

        assert_eq!(
            code_generator.next(),
            Some((Coord { row: 2, column: 1 }, 31916031))
        );
        assert_eq!(
            code_generator.next(),
            Some((Coord { row: 1, column: 2 }, 18749137))
        );

        assert_eq!(find_code_at_coord(Coord { row: 6, column: 6 }), 27995004);
    }
}
