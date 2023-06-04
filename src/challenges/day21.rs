use std::{collections::HashMap, str::FromStr};

enum ItemKind {
    Weapon,
    Armor,
    Ring,
}

struct Item {
    kind: ItemKind,
    name: &'static str,
    cost: u32,
    damage: u32,
    armor: u32,
}

struct Player {
    hit_points: u32,
    inventory: Vec<Item>,
}

impl Player {
    fn new(hit_points: u32) -> Self {
        Self {
            hit_points,
            inventory: Vec::new(),
        }
    }

    fn set_inventory(&mut self, inventory: Vec<Item>) {
        self.inventory = inventory;
    }

    fn inventory_cost(&self) -> u32 {
        self.inventory.iter().map(|item| item.cost).sum()
    }
}

impl Fighter for Player {
    fn hit_points(&self) -> u32 {
        self.hit_points
    }

    fn damage(&self) -> u32 {
        self.inventory.iter().map(|item| item.damage).sum()
    }

    fn armor(&self) -> u32 {
        self.inventory.iter().map(|item| item.armor).sum()
    }
}

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

trait Fighter {
    fn hit_points(&self) -> u32;
    fn damage(&self) -> u32;
    fn armor(&self) -> u32;
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

    #[test]
    fn player_inventory() {
        let mut player = Player::new(42);
        player.set_inventory(vec![
            Item {
                kind: ItemKind::Weapon,
                name: "dagger",
                cost: 5,
                damage: 6,
                armor: 0,
            },
            Item {
                kind: ItemKind::Armor,
                name: "chainmail",
                cost: 7,
                damage: 0,
                armor: 3,
            },
            Item {
                kind: ItemKind::Ring,
                name: "crazy ring",
                cost: 100,
                damage: 12,
                armor: 8,
            },
        ]);

        assert_eq!(player.inventory_cost(), 112);

        let fighter = &player as &dyn Fighter;
        assert_eq!(fighter.hit_points(), 42);
        assert_eq!(fighter.damage(), 18);
        assert_eq!(fighter.armor(), 11);
    }

}
