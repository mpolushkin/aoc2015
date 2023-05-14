use super::Challenge;
use std::error::Error;
use std::str::FromStr;

pub struct Day02 {
    list_of_dimensions: Vec<Dimensions>,
}

impl Challenge for Day02 {
    const DAY: u8 = 2;
    type Part1Solution = u32;
    type Part2Solution = u32;

    fn new(input: &str) -> Self {
        let list_of_dimensions: Vec<_> = input
            .lines()
            .map(|dimensions_str| dimensions_str.parse::<Dimensions>().unwrap())
            .collect();
        Self { list_of_dimensions }
    }

    fn solve_part1(&self) -> u32 {
        self.list_of_dimensions
            .iter()
            .map(|dimensions| dimensions.required_wrapping_paper())
            .sum()
    }

    fn solve_part2(&self) -> u32 {
        self.list_of_dimensions
            .iter()
            .map(|dimensions| dimensions.required_ribbon())
            .sum()
    }
}

#[derive(Debug, PartialEq)]
pub struct Dimensions {
    l: u32,
    w: u32,
    h: u32,
}

impl FromStr for Dimensions {
    type Err = Box<dyn Error>;
    fn from_str(value: &str) -> Result<Dimensions, Self::Err> {
        let elements: Vec<_> = value
            .split('x')
            .map(|x| x.parse())
            .collect::<Result<_, _>>()?;
        if elements.len() == 3 {
            Ok(Dimensions {
                l: elements[0],
                w: elements[1],
                h: elements[2],
            })
        } else {
            Err("Invalid number of elements".into())
        }
    }
}

impl Dimensions {
    pub fn required_wrapping_paper(&self) -> u32 {
        let face_areas = self.face_areas();
        face_areas.iter().sum::<u32>() * 2 + face_areas.iter().min().unwrap()
    }

    pub fn required_ribbon(&self) -> u32 {
        self.face_perimeters().iter().min().unwrap() + self.volume()
    }

    pub fn face_areas(&self) -> [u32; 3] {
        [self.l * self.w, self.w * self.h, self.h * self.l]
    }

    fn face_perimeters(&self) -> [u32; 3] {
        [
            2 * (self.l + self.w),
            2 * (self.w + self.h),
            2 * (self.h + self.l),
        ]
    }

    fn volume(&self) -> u32 {
        return self.l * self.w * self.h;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_from_str() {
        assert_eq!(
            "12x3x4".parse::<Dimensions>().unwrap(),
            Dimensions { l: 12, w: 3, h: 4 }
        );
        assert!("1x2x3x4".parse::<Dimensions>().is_err());
        assert!("1xa".parse::<Dimensions>().is_err());
    }

    #[test]
    fn test_required_wrapping_paper() {
        assert_eq!(
            "2x3x4"
                .parse::<Dimensions>()
                .unwrap()
                .required_wrapping_paper(),
            58
        );

        assert_eq!(
            "1x1x10"
                .parse::<Dimensions>()
                .unwrap()
                .required_wrapping_paper(),
            43
        );
    }

    #[test]
    fn test_required_ribbon() {
        assert_eq!("2x3x4".parse::<Dimensions>().unwrap().required_ribbon(), 34);

        assert_eq!(
            "1x1x10".parse::<Dimensions>().unwrap().required_ribbon(),
            14
        );
    }
}
