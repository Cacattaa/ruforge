use crate::stats::*;
use std::fmt::{Display, Formatter, Result};
use Reforge::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Reforge {
    CriticalChance(Stats, &'static str),
    CriticalDamage(Stats, &'static str),
}

impl Display for Reforge {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CriticalChance(_, name) => write!(f, "Critical Chance - {}", name),
            CriticalDamage(_, name) => write!(f, "Critical Damage - {}", name),
        }
    }
}

pub const REFORGES_MAPPING: [[Reforge; 2]; 5] = [
    [
        CriticalChance(
            Stats {
                critical_chance: 1,
                critical_damage: 1,
            },
            "Godly",
        ),
        CriticalDamage(
            Stats {
                critical_chance: 0,
                critical_damage: 3,
            },
            "Itchy",
        ),
    ],
    [
        CriticalChance(
            Stats {
                critical_chance: 2,
                critical_damage: 2,
            },
            "Godly",
        ),
        CriticalDamage(
            Stats {
                critical_chance: 0,
                critical_damage: 5,
            },
            "Itchy",
        ),
    ],
    [
        CriticalChance(
            Stats {
                critical_chance: 3,
                critical_damage: 2,
            },
            "Unpleasant",
        ),
        CriticalDamage(
            Stats {
                critical_chance: 0,
                critical_damage: 8,
            },
            "Itchy",
        ),
    ],
    [
        CriticalChance(
            Stats {
                critical_chance: 6,
                critical_damage: 3,
            },
            "Unpleasant",
        ),
        CriticalDamage(
            Stats {
                critical_chance: 0,
                critical_damage: 12,
            },
            "Itchy",
        ),
    ],
    [
        CriticalChance(
            Stats {
                critical_chance: 8,
                critical_damage: 5,
            },
            "Unpleasant",
        ),
        CriticalDamage(
            Stats {
                critical_chance: 0,
                critical_damage: 15,
            },
            "Itchy",
        ),
    ],
];
