use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct Boss {
    hit_points: u32,
    damage: u32,
    armor: u32,
}

type Error = String;

impl FromStr for Boss {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let attributes: HashMap<_, _> = s
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let (name, value) = line
                    .trim()
                    .split_once(": ")
                    .ok_or_else(|| "could not split line".to_owned())?;
                Ok((
                    name.to_owned(),
                    value
                        .parse::<u32>()
                        .map_err(|_| "could not parse value".to_owned())?,
                ))
            })
            .collect::<Result<_, Error>>()?;
        Ok(Self {
            hit_points: *attributes
                .get("Hit Points")
                .ok_or_else(|| "Hit points not specified".to_owned())?,
            damage: *attributes
                .get("Damage")
                .ok_or_else(|| "Damage not specified".to_owned())?,
            armor: *attributes
                .get("Armor")
                .ok_or_else(|| "Armor not specified".to_owned())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_boss() {
        assert_eq!(
            "
            Hit Points: 103
            Damage: 9
            Armor: 2
            "
            .parse(),
            Ok(Boss {
                hit_points: 103,
                damage: 9,
                armor: 2
            })
        );
    }
}
