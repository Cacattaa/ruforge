use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub struct Stats {
    pub strength: u16,
    pub critical_chance: u16,
    pub critical_damage: u16,
}

impl Add for Stats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            strength: self.strength + other.strength,
            critical_chance: self.critical_chance + other.critical_chance,
            critical_damage: self.critical_damage + other.critical_damage,
        }
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Strength: {} - Critical Chance: {} - Critical Damage: {}",
            self.strength, self.critical_chance, self.critical_damage
        )
    }
}
