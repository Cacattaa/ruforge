use std::cmp::Ordering;
use std::ops::Add;

use crate::stats::*;
use crate::talisman::*;

use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct Inventory {
    base_crit_chance: u16,
    talismans: Vec<Talisman>,
}

impl Inventory {
    pub fn new(base_crit_chance: u16, talismans: &[Talisman]) -> Inventory {
        Inventory {
            base_crit_chance,
            talismans: talismans.to_vec(),
        }
    }

    pub fn stats(&self) -> Stats {
        let stats = Stats {
            critical_chance: self.base_crit_chance,
            critical_damage: 0,
        };

        self.talismans
            .iter()
            .map(|t| t.into())
            .fold(stats, Stats::add)
    }

    pub fn improved(&self) -> Inventory {
        let mut start = self.clone();
        start
            .talismans
            .iter_mut()
            .for_each(Talisman::reforge_as_crit_chance);

        let number_of_proc = num_cpus::get() as f32;
        let number_of_proc = 2_u32.pow(number_of_proc.log2() as u32);

        let mut chunks = vec![start.clone()];
        for _ in 0..(number_of_proc - 1) {
            Inventory::increment(start.talismans.iter_mut().rev());
            chunks.push(start.clone());
        }

        let chunk_size = 2_u64.pow(start.talismans.len() as u32) / (number_of_proc as u64);
        chunks
            .into_par_iter()
            .map(|i| i.find_best(chunk_size))
            .max()
            .unwrap()
    }

    fn find_best(&self, mut iterations: u64) -> Inventory {
        let mut current = self.clone();
        let mut best = current.clone();

        while iterations > 0 {
            iterations -= 1;

            Inventory::increment(current.talismans.iter_mut());
            if current > best {
                best = current.clone();
            }
        }

        best
    }

    fn increment<'a, T>(iter: T)
    where
        T: Iterator<Item = &'a mut Talisman>,
    {
        for t in iter {
            match t.reforge {
                Reforge::CriticalChance(_) => {
                    t.reforge_as_crit_damage();
                    break;
                }
                Reforge::CriticalDamage(_) => t.reforge_as_crit_chance(),
            }
        }
    }
}

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
            (true, true) => self_stats.critical_damage.cmp(&other_stats.critical_damage),
            (false, false) => self_stats.critical_chance.cmp(&other_stats.critical_chance),
        }
    }
}

impl PartialOrd for Inventory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
            (true, true) => self_stats.critical_damage == other_stats.critical_damage,
            (false, false) => self_stats == other_stats,
        }
    }
}
impl Eq for Inventory {}
