use std::{cmp::max, collections::HashMap, ops::Range, str::FromStr};

use itertools::{Itertools, Product};

use super::Challenge;

pub struct Day21 {
    boss: Boss,
}

fn inventory_cost(inventory: &Vec<Item>) -> u32 {
    inventory.iter().map(|item| item.cost).sum()
}

impl Day21 {
    fn winner_given_inventory(&self, inventory: Vec<Item>) -> Winner {
        let player = Player::with_inventory(100, inventory);
        Battle::new(&player, &self.boss).resolve()
    }
}

impl Challenge for Day21 {
    const DAY: u8 = 21;

    type Part1Solution = u32;
    type Part2Solution = u32;

    fn new(input: &str) -> Self {
        Self {
            boss: input.parse::<Boss>().unwrap(),
        }
    }

    fn solve_part1(&self) -> Self::Part1Solution {
        AllPossibleInventories::new(ITEMS)
            .filter(|inventory| self.winner_given_inventory(inventory.clone()) == Winner::Player)
            .map(|inventory| inventory_cost(&inventory))
            .min()
            .expect("no valid inventory")
    }

    fn solve_part2(&self) -> Self::Part2Solution {
        AllPossibleInventories::new(ITEMS)
            .filter(|inventory| self.winner_given_inventory(inventory.clone()) == Winner::Boss)
            .map(|inventory| inventory_cost(&inventory))
            .max()
            .expect("no valid inventory")
    }
}

#[derive(Debug, Clone, Copy)]
enum ItemKind {
    Weapon,
    Armor,
    Ring,
}

#[derive(Debug, Clone, Copy)]
struct Item {
    kind: ItemKind,
    name: &'static str,
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    const fn new(kind: ItemKind, name: &'static str, cost: u32, damage: u32, armor: u32) -> Self {
        Self {
            kind,
            name,
            cost,
            damage,
            armor,
        }
    }
}

static ITEMS: &[Item] = &[
    Item::new(ItemKind::Weapon, "Dagger", 8, 4, 0),
    Item::new(ItemKind::Weapon, "Shortsword", 10, 5, 0),
    Item::new(ItemKind::Weapon, "Warhammer", 25, 6, 0),
    Item::new(ItemKind::Weapon, "Longsword", 40, 7, 0),
    Item::new(ItemKind::Weapon, "Greataxe", 74, 8, 0),
    Item::new(ItemKind::Armor, "Leather", 13, 0, 1),
    Item::new(ItemKind::Armor, "Chainmail", 31, 0, 2),
    Item::new(ItemKind::Armor, "Splintmail", 53, 0, 3),
    Item::new(ItemKind::Armor, "Bandedmail", 75, 0, 4),
    Item::new(ItemKind::Armor, "Platemail", 102, 0, 5),
    Item::new(ItemKind::Ring, "Damage +1", 25, 1, 0),
    Item::new(ItemKind::Ring, "Damage +2", 50, 2, 0),
    Item::new(ItemKind::Ring, "Damage +3", 100, 3, 0),
    Item::new(ItemKind::Ring, "Defense +1", 20, 0, 1),
    Item::new(ItemKind::Ring, "Defense +2", 40, 0, 2),
    Item::new(ItemKind::Ring, "Defense +3", 80, 0, 3),
];

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
    fn with_inventory(hit_points: u32, inventory: Vec<Item>) -> Self {
        let mut self_ = Self::new(hit_points);
        self_.set_inventory(inventory);
        self_
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

impl Fighter for Boss {
    fn hit_points(&self) -> u32 {
        self.hit_points
    }

    fn damage(&self) -> u32 {
        self.damage
    }

    fn armor(&self) -> u32 {
        self.armor
    }
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

#[derive(Debug, PartialEq, Eq)]
enum Winner {
    Player,
    Boss,
}

struct Battle<'a> {
    player: &'a Player,
    boss: &'a Boss,
    player_hit_points: u32,
    boss_hit_points: u32,
    player_turn: bool,
}

impl<'a> Battle<'a> {
    fn new(player: &'a Player, boss: &'a Boss) -> Self {
        Self {
            player,
            boss,
            player_hit_points: player.hit_points,
            boss_hit_points: boss.hit_points,
            player_turn: true,
        }
    }

    fn player_hit_points(&self) -> u32 {
        self.player_hit_points
    }

    fn boss_hit_points(&self) -> u32 {
        self.boss_hit_points
    }

    fn attacker(&self) -> &dyn Fighter {
        if self.player_turn {
            self.player
        } else {
            self.boss
        }
    }

    fn defender(&self) -> &dyn Fighter {
        if self.player_turn {
            self.boss
        } else {
            self.player
        }
    }

    fn deal_damage_to_defender(&mut self, damage: u32) {
        let defender_hit_points = if self.player_turn {
            &mut self.boss_hit_points
        } else {
            &mut self.player_hit_points
        };
        *defender_hit_points = defender_hit_points.saturating_sub(damage);
    }

    fn next_turn(&mut self) {
        self.deal_damage_to_defender(Self::calculate_damage(self.attacker(), self.defender()));
        self.player_turn = !self.player_turn;
    }

    fn calculate_damage(attacker: &dyn Fighter, defender: &dyn Fighter) -> u32 {
        max(attacker.damage().saturating_sub(defender.armor()), 1)
    }

    fn winner(&self) -> Option<Winner> {
        if self.boss_hit_points == 0 {
            Some(Winner::Player)
        } else if self.player_hit_points == 0 {
            Some(Winner::Boss)
        } else {
            None
        }
    }

    fn resolve(&mut self) -> Winner {
        loop {
            self.next_turn();
            if let Some(winner) = self.winner() {
                return winner;
            }
        }
    }
}

#[derive(Clone)]
struct VariableKIndexCombinations {
    num_elements: usize,
    max_k: usize,
    current_combinations: itertools::Combinations<Range<usize>>,
}

impl VariableKIndexCombinations {
    fn new(num_elements: usize, min_k: usize, max_k: usize) -> Self {
        Self {
            num_elements,
            max_k,
            current_combinations: (0..num_elements).combinations(min_k),
        }
    }
}

impl Iterator for VariableKIndexCombinations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(indices) = self.current_combinations.next() {
            Some(indices)
        } else {
            let k = self.current_combinations.k();
            if k < self.max_k {
                self.current_combinations = (0..self.num_elements).combinations(k + 1);
                self.next()
            } else {
                None
            }
        }
    }
}

struct AllPossibleInventories<'a> {
    available_items: &'a [Item],
    weapon_ids: Vec<usize>,
    armor_ids: Vec<usize>,
    ring_ids: Vec<usize>,
    inner: Product<
        Product<VariableKIndexCombinations, VariableKIndexCombinations>,
        VariableKIndexCombinations,
    >,
}

impl<'a> AllPossibleInventories<'a> {
    fn new(available_items: &'a [Item]) -> Self {
        let mut weapon_ids = Vec::new();
        let mut armor_ids = Vec::new();
        let mut ring_ids = Vec::new();
        for (i, item) in available_items.iter().enumerate() {
            match item.kind {
                ItemKind::Weapon => weapon_ids.push(i),
                ItemKind::Armor => armor_ids.push(i),
                ItemKind::Ring => ring_ids.push(i),
            }
        }
        let inner = VariableKIndexCombinations::new(weapon_ids.len(), 1, 1)
            .cartesian_product(VariableKIndexCombinations::new(armor_ids.len(), 0, 1))
            .cartesian_product(VariableKIndexCombinations::new(ring_ids.len(), 0, 2));
        Self {
            available_items,
            weapon_ids,
            armor_ids,
            ring_ids,
            inner,
        }
    }
}

impl Iterator for AllPossibleInventories<'_> {
    type Item = Vec<Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let ((weapon, armor), rings) = self.inner.next()?;
        Some(
            weapon
                .iter()
                .map(|&i| self.weapon_ids[i])
                .chain(armor.iter().map(|&i| self.armor_ids[i]))
                .chain(rings.iter().map(|&i| self.ring_ids[i]))
                .map(|i| self.available_items[i])
                .collect(),
        )
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

    const DAGGER: Item = Item {
        kind: ItemKind::Weapon,
        name: "dagger",
        cost: 5,
        damage: 5,
        armor: 0,
    };
    const CHAINMAIL: Item = Item {
        kind: ItemKind::Armor,
        name: "chainmail",
        cost: 7,
        damage: 0,
        armor: 5,
    };

    #[test]
    fn test_player_inventory() {
        let mut player = Player::new(42);
        player.set_inventory(vec![
            DAGGER,
            CHAINMAIL,
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
        assert_eq!(fighter.damage(), 17);
        assert_eq!(fighter.armor(), 13);
    }

    #[test]
    fn test_battle() {
        let player = Player::with_inventory(8, vec![DAGGER, CHAINMAIL]);
        let boss = Boss {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };

        let mut battle = Battle::new(&player, &boss);

        assert_eq!(battle.player_hit_points(), 8);
        assert_eq!(battle.boss_hit_points(), 12);
        battle.next_turn();
        assert_eq!(battle.player_hit_points(), 8);
        assert_eq!(battle.boss_hit_points(), 9);
        battle.next_turn();
        assert_eq!(battle.player_hit_points(), 6);
        assert_eq!(battle.boss_hit_points(), 9);
        battle.next_turn();
        assert_eq!(battle.player_hit_points(), 6);
        assert_eq!(battle.boss_hit_points(), 6);

        assert_eq!(battle.resolve(), Winner::Player);
        assert_eq!(battle.player_hit_points(), 2);
        assert_eq!(battle.boss_hit_points(), 0);

        let weak_but_armored_boss = Boss {
            hit_points: 10,
            damage: 0,
            armor: 5,
        };
        let mut battle_weak_but_armored = Battle::new(&player, &weak_but_armored_boss);

        battle_weak_but_armored.next_turn();
        assert_eq!(battle_weak_but_armored.player_hit_points(), 8);
        assert_eq!(
            battle_weak_but_armored.boss_hit_points(),
            9,
            "zero damage counts as 1 damage"
        );
        battle_weak_but_armored.next_turn();
        assert_eq!(
            battle_weak_but_armored.player_hit_points(),
            7,
            "negative damage counts as 1 damage"
        );
        assert_eq!(battle_weak_but_armored.boss_hit_points(), 9);
    }

    #[test]
    fn test_variable_k_index_combinations() {
        assert_eq!(
            VariableKIndexCombinations::new(4, 1, 1).collect::<Vec<_>>(),
            vec![vec![0], vec![1], vec![2], vec![3]]
        );

        assert_eq!(
            VariableKIndexCombinations::new(2, 0, 1).collect::<Vec<_>>(),
            vec![Vec::new(), vec![0], vec![1]]
        );

        assert_eq!(
            VariableKIndexCombinations::new(3, 0, 2).collect::<Vec<_>>(),
            vec![
                Vec::new(),
                vec![0],
                vec![1],
                vec![2],
                vec![0, 1],
                vec![0, 2],
                vec![1, 2]
            ]
        );
    }
}
