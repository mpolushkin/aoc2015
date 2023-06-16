use std::{cmp::max, collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Player {
    hit_points: u32,
    armor: u32,
    mana: u32,
}

impl Player {
    pub fn new(hit_points: u32, mana: u32) -> Self {
        Self {
            hit_points,
            armor: 0,
            mana,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Player has {} hit point{}, {} armor, {} mana",
            self.hit_points,
            if self.hit_points > 1 { "s" } else { "" },
            self.armor,
            self.mana
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Boss {
    hit_points: u32,
    damage: u32,
}

impl Boss {
    pub fn new(hit_points: u32, damage: u32) -> Self {
        Self { hit_points, damage }
    }
}

impl Display for Boss {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Boss has {} hit points", self.hit_points)
    }
}

#[derive(Clone, Copy)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn mana_cost(&self) -> u32 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }

    fn cast(&self, game: &mut Game) {
        game.player.mana -= self.mana_cost();
        match self {
            Spell::MagicMissile => deal_damage(&mut game.boss.hit_points, 4),
            Spell::Drain => {
                deal_damage(&mut game.boss.hit_points, 2);
                game.player.hit_points += 2;
            }
            Self::Shield => game.activate_effect(Effect::Shield, 6),
            Spell::Poison => game.activate_effect(Effect::Poison, 6),
            Spell::Recharge => game.activate_effect(Effect::Recharge, 5),
        }
    }

    fn effect(&self) -> Option<Effect> {
        match self {
            Spell::MagicMissile | Spell::Drain => None,
            Spell::Shield => Some(Effect::Shield),
            Spell::Poison => Some(Effect::Poison),
            Spell::Recharge => Some(Effect::Recharge),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Effect {
    Shield,
    Poison,
    Recharge,
}

impl Effect {
    pub fn activate(&self, player: &mut Player, _boss: &mut Boss) {
        match self {
            Effect::Shield => player.armor += 7,
            Effect::Poison | Effect::Recharge => (),
        }
    }
    pub fn apply(&self, player: &mut Player, boss: &mut Boss) {
        match self {
            Effect::Shield => (),
            Effect::Poison => deal_damage(&mut boss.hit_points, 3),
            Effect::Recharge => player.mana += 101,
        }
    }

    pub fn deactivate(&self, player: &mut Player, _boss: &mut Boss) {
        match self {
            Effect::Shield => player.armor -= 7,
            Effect::Poison | Effect::Recharge => (),
        }
    }
}

struct EffectTimers {
    shield_timer: u8,
    poison_timer: u8,
    recharge_timer: u8,
}

impl EffectTimers {
    pub fn new() -> Self {
        Self {
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
        }
    }

    pub fn is_active(&self, effect: Effect) -> bool {
        self.timer(effect) > 0
    }

    pub fn activate(&mut self, effect: Effect, duration: u8) {
        *self.timer_mut(effect) = duration;
    }

    pub fn try_decrement(&mut self, effect: Effect) -> Result<u8, ()> {
        let timer = self.timer_mut(effect);
        if *timer == 0 {
            Err(())
        } else {
            *timer -= 1;
            Ok(*timer)
        }
    }

    fn timer(&self, effect: Effect) -> u8 {
        match effect {
            Effect::Shield => self.shield_timer,
            Effect::Poison => self.poison_timer,
            Effect::Recharge => self.recharge_timer,
        }
    }

    fn timer_mut(&mut self, effect: Effect) -> &mut u8 {
        match effect {
            Effect::Shield => &mut self.shield_timer,
            Effect::Poison => &mut self.poison_timer,
            Effect::Recharge => &mut self.recharge_timer,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum GameError {
    GameFinished,
    NotEnoughMana,
    EffectAlreadyActive,
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::GameFinished => "game is already finished",
                Self::NotEnoughMana => "not enough mana",
                Self::EffectAlreadyActive => "effect already active",
            }
        )
    }
}

impl std::error::Error for GameError {}

struct Game {
    player: Player,
    boss: Boss,
    effect_timers: EffectTimers,
}

impl Game {
    pub fn new(player: Player, boss: Boss) -> Self {
        Self {
            player,
            boss,
            effect_timers: EffectTimers::new(),
        }
    }

    pub fn player(&self) -> Player {
        self.player
    }

    pub fn boss(&self) -> Boss {
        self.boss
    }

    pub fn poison_timer(&self) -> u8 {
        self.effect_timers.timer(Effect::Poison)
    }

    pub fn winner(&self) -> Option<Winner> {
        if self.player.hit_points == 0 {
            Some(Winner::Boss)
        } else if self.boss.hit_points == 0 {
            Some(Winner::Player)
        } else {
            None
        }
    }

    pub fn player_take_turn(&mut self, spell: Spell) -> Result<Option<Winner>, GameError> {
        self.assert_no_winner_yet()?;
        self.assert_player_can_cast(spell)?;
        self.apply_active_effects()
            .and_then(|()| self.player_cast_spell(spell))
            .map_or_else(|winner| Ok(Some(winner)), |()| Ok(None))
    }

    pub fn boss_take_turn(&mut self) -> Result<Option<Winner>, GameError> {
        self.assert_no_winner_yet()?;
        self.apply_active_effects()
            .and_then(|()| self.boss_attack())
            .map_or_else(|winner| Ok(Some(winner)), |()| Ok(None))
    }

    fn assert_no_winner_yet(&self) -> Result<(), GameError> {
        if self.winner().is_some() {
            Err(GameError::GameFinished)
        } else {
            Ok(())
        }
    }

    fn assert_player_can_cast(&self, spell: Spell) -> Result<(), GameError> {
        if self.player.mana < spell.mana_cost() {
            return Err(GameError::NotEnoughMana);
        }
        if let Some(effect) = spell.effect() {
            if self.effect_timers.is_active(effect) {
                return Err(GameError::EffectAlreadyActive);
            }
        }
        Ok(())
    }

    fn winner_result(&self) -> Result<(), Winner> {
        match self.winner() {
            None => Ok(()),
            Some(winner) => Err(winner),
        }
    }

    fn activate_effect(&mut self, effect: Effect, duration: u8) {
        self.effect_timers.activate(effect, duration);
        effect.activate(&mut self.player, &mut self.boss);
    }

    fn apply_active_effects(&mut self) -> Result<(), Winner> {
        self.apply_effect_if_active(Effect::Shield)?;
        self.apply_effect_if_active(Effect::Poison)?;
        self.apply_effect_if_active(Effect::Recharge)?;
        Ok(())
    }

    fn apply_effect_if_active(&mut self, effect: Effect) -> Result<(), Winner> {
        if let Ok(timer) = self.effect_timers.try_decrement(effect) {
            effect.apply(&mut self.player, &mut self.boss);
            if timer == 0 {
                effect.deactivate(&mut self.player, &mut self.boss);
            }
            self.winner_result()
        } else {
            Ok(())
        }
    }

    fn player_cast_spell(&mut self, spell: Spell) -> Result<(), Winner> {
        spell.cast(self);
        self.winner_result()
    }

    fn boss_attack(&mut self) -> Result<(), Winner> {
        deal_damage(
            &mut self.player.hit_points,
            self.boss.damage.saturating_sub(self.player.armor),
        );
        self.winner_result()
    }
}

fn deal_damage(defender_hit_points: &mut u32, attacker_damage: u32) {
    *defender_hit_points = defender_hit_points.saturating_sub(max(attacker_damage, 1));
}

fn decrement_effect_counter(counter: &mut Option<u8>) -> Result<(), ()> {
    match counter {
        None => Err(()),
        Some(ref mut count) => {
            *count -= 1;
            if *count == 0 {
                *counter = None;
            }
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Winner {
    Player,
    Boss,
}

type ParseError = String;

impl FromStr for Boss {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let attributes: HashMap<_, _> = s
            .trim()
            .lines()
            .map(|line| {
                let (name, value) = line
                    .split_once(':')
                    .ok_or_else(|| "expected ':'".to_owned())?;
                Ok((
                    name.trim(),
                    value
                        .trim()
                        .parse::<u32>()
                        .map_err(|_| "could not parse value".to_owned())?,
                ))
            })
            .collect::<Result<_, ParseError>>()?;
        Ok(Self {
            hit_points: *attributes
                .get("Hit Points")
                .ok_or_else(|| "hit points not defined".to_owned())?,
            damage: *attributes
                .get("Damage")
                .ok_or_else(|| "damage not defined".to_owned())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let boss: Boss = "
            Hit Points: 51
            Damage: 9
            "
        .parse()
        .unwrap();

        assert_eq!(boss.hit_points, 51);
        assert_eq!(boss.damage, 9);
    }

    #[test]
    fn test_game_scenario_1() {
        let mut game = Game::new(Player::new(10, 250), Boss::new(13, 8));
        assert_eq!(
            game.player().to_string(),
            "Player has 10 hit points, 0 armor, 250 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 13 hit points");

        assert_eq!(game.player_take_turn(Spell::Poison), Ok(None));
        assert_eq!(
            game.player().to_string(),
            "Player has 10 hit points, 0 armor, 77 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 13 hit points");
        assert_eq!(game.poison_timer(), 6);

        assert_eq!(game.boss_take_turn(), Ok(None));
        assert_eq!(
            game.player().to_string(),
            "Player has 2 hit points, 0 armor, 77 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 10 hit points");
        assert_eq!(game.poison_timer(), 5);

        assert_eq!(game.player_take_turn(Spell::MagicMissile), Ok(None));
        assert_eq!(
            game.player().to_string(),
            "Player has 2 hit points, 0 armor, 24 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 3 hit points");

        assert_eq!(game.boss_take_turn(), Ok(Some(Winner::Player)))
    }

    #[test]
    fn test_game_scenario_2() {
        let mut game = Game::new(Player::new(10, 250), Boss::new(14, 8));
        assert_eq!(
            game.player().to_string(),
            "Player has 10 hit points, 0 armor, 250 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 14 hit points");
        assert_eq!(game.player_take_turn(Spell::Recharge), Ok(None));

        assert_eq!(
            game.player().to_string(),
            "Player has 10 hit points, 0 armor, 21 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 14 hit points");
        assert_eq!(game.boss_take_turn(), Ok(None));
        assert_eq!(game.effect_timers.recharge_timer, 4);

        assert_eq!(
            game.player().to_string(),
            "Player has 2 hit points, 0 armor, 122 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 14 hit points");
        assert_eq!(game.player_take_turn(Spell::Shield), Ok(None));
        assert_eq!(game.effect_timers.recharge_timer, 3);

        assert_eq!(
            game.player().to_string(),
            "Player has 2 hit points, 7 armor, 110 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 14 hit points");
        assert_eq!(game.boss_take_turn(), Ok(None));
        assert_eq!(game.effect_timers.shield_timer, 5);
        assert_eq!(game.effect_timers.recharge_timer, 2);

        assert_eq!(
            game.player().to_string(),
            "Player has 1 hit point, 7 armor, 211 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 14 hit points");
        assert_eq!(game.player_take_turn(Spell::Drain), Ok(None));
        assert_eq!(game.effect_timers.shield_timer, 4);
        assert_eq!(game.effect_timers.recharge_timer, 1);

        assert_eq!(
            game.player().to_string(),
            "Player has 3 hit points, 7 armor, 239 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 12 hit points");
        assert_eq!(game.boss_take_turn(), Ok(None));
        assert_eq!(game.effect_timers.shield_timer, 3);
        assert_eq!(game.effect_timers.recharge_timer, 0);

        assert_eq!(
            game.player().to_string(),
            "Player has 2 hit points, 7 armor, 340 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 12 hit points");
        assert_eq!(game.player_take_turn(Spell::Poison), Ok(None));
        assert_eq!(game.effect_timers.shield_timer, 2);

        assert_eq!(
            game.player().to_string(),
            "Player has 2 hit points, 7 armor, 167 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 12 hit points");
        assert_eq!(game.boss_take_turn(), Ok(None));
        assert_eq!(game.effect_timers.poison_timer, 5);
        assert_eq!(game.effect_timers.shield_timer, 1);

        assert_eq!(
            game.player().to_string(),
            "Player has 1 hit point, 7 armor, 167 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 9 hit points");
        assert_eq!(game.player_take_turn(Spell::MagicMissile), Ok(None));
        assert_eq!(game.effect_timers.poison_timer, 4);
        assert_eq!(game.effect_timers.shield_timer, 0);

        assert_eq!(
            game.player().to_string(),
            "Player has 1 hit point, 0 armor, 114 mana"
        );
        assert_eq!(game.boss().to_string(), "Boss has 2 hit points");
        assert_eq!(game.boss_take_turn(), Ok(Some(Winner::Player)));
    }
}
