use std::convert::TryFrom;

use crate::stats::*;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Rarity {
    Common = 0,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl TryFrom<u8> for Rarity {
    type Error = &'static str;

    fn try_from(t: u8) -> Result<Self, Self::Error> {
        if t > (Rarity::Legendary as u8) {
            return Err("test");
        }
        Ok(unsafe { std::mem::transmute(t) })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Reforge {
    CriticalChance(Stats),
    CriticalDamage(Stats),
}

const STATS_MAPPING: [[Stats; 2]; 5] = [
    [
        Stats {
            critical_chance: 1,
            critical_damage: 1,
        },
        Stats {
            critical_chance: 0,
            critical_damage: 3,
        },
    ],
    [
        Stats {
            critical_chance: 2,
            critical_damage: 2,
        },
        Stats {
            critical_chance: 0,
            critical_damage: 5,
        },
    ],
    [
        Stats {
            critical_chance: 3,
            critical_damage: 2,
        },
        Stats {
            critical_chance: 0,
            critical_damage: 8,
        },
    ],
    [
        Stats {
            critical_chance: 6,
            critical_damage: 3,
        },
        Stats {
            critical_chance: 0,
            critical_damage: 12,
        },
    ],
    [
        Stats {
            critical_chance: 8,
            critical_damage: 5,
        },
        Stats {
            critical_chance: 0,
            critical_damage: 15,
        },
    ],
];

#[derive(Copy, Clone, Debug)]
pub struct Talisman {
    pub rarity: Rarity,
    pub reforge: Reforge,
}

impl Talisman {
    pub fn new(rarity: Rarity) -> Self {
        Self {
            rarity,
            reforge: Reforge::CriticalChance(STATS_MAPPING[rarity as usize][0]),
        }
    }

    #[inline(always)]
    pub fn reforge_as_crit_chance(&mut self) {
        self.reforge = Reforge::CriticalChance(STATS_MAPPING[self.rarity as usize][0])
    }

    #[inline(always)]
    pub fn reforge_as_crit_damage(&mut self) {
        self.reforge = Reforge::CriticalDamage(STATS_MAPPING[self.rarity as usize][1])
    }
}

impl Into<Stats> for &Talisman {
    fn into(self) -> Stats {
        match self.reforge {
            Reforge::CriticalChance(s) | Reforge::CriticalDamage(s) => s,
        }
    }
}
