#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Rarity {
    Common = 0,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl From<u8> for Rarity {
    fn from(t: u8) -> Rarity {
        assert!(t >= (Rarity::Common as u8) && t <= (Rarity::Legendary as u8));
        unsafe { std::mem::transmute(t) }
    }
}

#[derive(Debug)]
pub enum Reforge {
    CriticalChance = 0,
    CriticalDamage
}

#[derive(Debug)]
pub struct Stats {
    pub critical_chance: u64,
    pub critical_damage: u64
}

const STATS_MAPPING: [[Stats; 2]; 5] = [
    [
        Stats{critical_chance: 1, critical_damage: 1},
        Stats{critical_chance: 0, critical_damage: 3},
    ],
    [
        Stats{critical_chance: 2, critical_damage: 2},
        Stats{critical_chance: 0, critical_damage: 5},
    ],
    [
        Stats{critical_chance: 3, critical_damage: 2},
        Stats{critical_chance: 0, critical_damage: 8},
    ],
    [
        Stats{critical_chance: 6, critical_damage: 3},
        Stats{critical_chance: 0, critical_damage: 12},
    ],
    [
        Stats{critical_chance: 8, critical_damage: 5},
        Stats{critical_chance: 0, critical_damage: 15},
    ],
];

#[derive(Debug)]
pub struct Talisman {
    pub rarity: Rarity
}

pub fn solve(talismans: &[Talisman], start: u64, end: u64, base_crit_chance: u64) -> u64 {
    let mut max = 0;
    let mut res = 0;
    for i in start..end {
        let score = score(talismans, i);
        let current = evaluate(score, base_crit_chance);
        if current > max {
            max = current;
            res = i;
        }
    }

    res
}

pub fn score(talismans: &[Talisman], mut config: u64) -> Stats {
    let mut total_stats = Stats{ critical_chance: 0, critical_damage: 0 };

    let mut talismans = talismans.iter();
    while config != 0 {
        let current = talismans.next().expect("should have find one here");
        let choice = config & 1;

        let stats = &STATS_MAPPING[current.rarity as u8 as usize][choice as usize];
        total_stats.critical_chance += stats.critical_chance;
        total_stats.critical_damage += stats.critical_damage;

        config = config >> 1;
    }

    for talisman in talismans {
        let stats = &STATS_MAPPING[talisman.rarity as u8 as usize][0];
        total_stats.critical_chance += stats.critical_chance;
        total_stats.critical_damage += stats.critical_damage;
    }

    total_stats
}

pub fn evaluate(stats: Stats, base_crit_chance: u64) -> u64 {
    let mut res: u64 = 0;
    res += stats.critical_damage;
    if base_crit_chance + stats.critical_chance > 100 {
        res += 100_000
    } else {
        res += (base_crit_chance + stats.critical_chance) * 1_000
    }

    res
}