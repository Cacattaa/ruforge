use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use crate::stats::*;
use crate::talisman::*;

use rand::seq::SliceRandom;
use rayon::prelude::*;
use rand::{rngs::ThreadRng, thread_rng};

#[derive(Clone, Debug)]
pub struct Inventory {
    weapon_damage: u16,
    base_stats: Stats,
    talismans: Vec<Talisman>,
}

impl Inventory {
    pub fn new(weapon_damage: u16, base_stats: Stats, talismans: &[Talisman]) -> Inventory {
        Inventory {
            weapon_damage,
            base_stats,
            talismans: talismans.to_vec(),
        }
    }

    pub fn stats(&self) -> Stats {
        self.talismans
            .iter()
            .fold(self.base_stats, |c, t| c + t.reforge.stats)
    }
    #[inline(always)]
    fn damage(&self, stats: &Stats) -> u32 {
        ((5 + self.weapon_damage + stats.strength / 5) as u32 *
         (100 + stats.strength) as u32 *
         (100 + stats.critical_damage) as u32) / 10_000
    }

    pub fn improved(&self, iterations: u64, attempts: u64) -> Inventory {
        if self.talismans.is_empty() {
            return self.clone();
        }

        (0..iterations)
            .into_par_iter()
            .fold(|| self.clone(), |acc, _| self.find_best(attempts, &acc))
            .max()
            .unwrap()
            .clone()
    }

    fn find_best(&self, max_attempts: u64, current_best: &Inventory) -> Inventory {
        let mut seed = thread_rng();

        let mut current = self.clone();
        current.shuffle(&mut seed);

        let mut best = current_best.clone();

        let mut attempts = max_attempts;
        while attempts > 0 {
            current.talismans.choose_mut(&mut seed).unwrap().increment_reforge();
            current.talismans.choose_mut(&mut seed).unwrap().increment_reforge();

            if current > best {
                attempts = max_attempts;
                best = current.clone();
            } else {
                attempts -= 1;
            }
        }

        best
    }

    fn shuffle(&mut self, seed: &mut ThreadRng) {
        for t in self.talismans.iter_mut() {
            t.randomize_reforge(seed);
        }
    }
}

impl PartialEq for Inventory {
    fn eq(&self, other: &Self) -> bool {
        let self_stats = self.stats();
        let other_stats = other.stats();

        match (
            self_stats.critical_chance >= 100,
            other_stats.critical_chance >= 100,
        ) {
            (true, false) | (false, true) => false,
            (true, true) => {
                self_stats.strength == other_stats.strength
                    && self_stats.critical_damage == other_stats.critical_damage
            }
            (false, false) => self_stats == other_stats,
        }
    }
}
impl Eq for Inventory {}

impl Ord for Inventory {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_stats = self.stats();
        let other_stats = other.stats();

        match (
            self_stats.critical_chance >= 100,
            other_stats.critical_chance >= 100,
        ) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (true, true) => self.damage(&self_stats).cmp(&other.damage(&other_stats)),
            (false, false) => self_stats.critical_chance.cmp(&other_stats.critical_chance),
        }
    }
}

impl PartialOrd for Inventory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Inventory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut groups: Vec<(u8, Talisman)> = Vec::new();
        for t in &self.talismans {
            if let Some(group) = groups.iter_mut().find(|(_, t2)| t2 == t) {
                group.0 += 1;
            } else {
                groups.push((1, *t));
            }
        }

        writeln!(f, "Base Stats: {}", self.base_stats)?;
        for (n, t) in &groups {
            writeln!(f, "{}x {}", n, t)?;
        }
        if !groups.is_empty() {
            write!(f, "\n")?;
        }

        let final_stats = self.stats();
        writeln!(f, "Final Stats: {}", &final_stats)?;
        write!(f, "Final Damage: {}", self.damage(&final_stats))
    }
}
