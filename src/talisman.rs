use std::convert::TryFrom;

use crate::reforge::*;
use std::fmt::{Display, Formatter};

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
            return Err("invalid rarity index");
        }
        Ok(unsafe { std::mem::transmute(t) })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Talisman {
    pub rarity: Rarity,
    pub reforge: Reforge,
    time_reforged: usize,
}

impl Talisman {
    pub fn new(rarity: Rarity) -> Self {
        Self {
            rarity,
            reforge: REFORGES_MAPPING[rarity as usize][0],
            time_reforged: 0
        }
    }

    pub fn increment_reforge(&mut self) -> bool {
        self.time_reforged = (self.time_reforged + 1) % REFORGES_MAPPING[self.rarity as usize].len();
        self.reforge = REFORGES_MAPPING[self.rarity as usize][self.time_reforged];
        self.time_reforged == 0
    }

    pub fn randomize_reforge(&mut self, seed: &mut ThreadRng) {
        self.reforge = *REFORGES_MAPPING[self.rarity as usize].choose(seed).unwrap();
    }
}

// impl Into<Stats> for &Talisman {
//     fn into(self) -> Stats {
//         match self.reforge {
//             Reforge::CriticalChance(s, _) | Reforge::CriticalDamage(s, _) => s,
//         }
//     }
// }

impl Display for Talisman {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {}", self.rarity, self.reforge)
    }
}
