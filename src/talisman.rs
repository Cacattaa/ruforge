use std::convert::TryFrom;

use crate::reforge::*;
use crate::stats::*;
use std::fmt::{Display, Formatter};

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
pub struct Talisman {
    pub rarity: Rarity,
    pub reforge: Reforge,
}

impl Talisman {
    pub fn new(rarity: Rarity) -> Self {
        Self {
            rarity,
            reforge: REFORGES_MAPPING[rarity as usize][0],
        }
    }

    #[inline(always)]
    pub fn reforge_as_crit_chance(&mut self) {
        self.reforge = REFORGES_MAPPING[self.rarity as usize][0]
    }

    #[inline(always)]
    pub fn reforge_as_crit_damage(&mut self) {
        self.reforge = REFORGES_MAPPING[self.rarity as usize][1]
    }
}

impl Into<Stats> for &Talisman {
    fn into(self) -> Stats {
        match self.reforge {
            Reforge::CriticalChance(s, _) | Reforge::CriticalDamage(s, _) => s,
        }
    }
}

impl Display for Talisman {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {}", self.rarity, self.reforge)
    }
}
