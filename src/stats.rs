use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub struct Stats {
    pub critical_chance: u16,
    pub critical_damage: u16,
}

impl Add for Stats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            critical_chance: self.critical_chance + other.critical_chance,
            critical_damage: self.critical_damage + other.critical_damage,
        }
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Critical Chance: {} - Critical Damage: {}",
            self.critical_chance, self.critical_damage
        )
    }
}
