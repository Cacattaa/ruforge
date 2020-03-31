use std::fmt::{Display, Formatter, Result};

use crate::stats::*;

use lazy_static::lazy_static;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Reforge {
    pub stats: Stats,
    pub name: &'static str,
}

impl Display for Reforge {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)
    }
}

lazy_static! {
    pub static ref REFORGES_MAPPING: [Vec<Reforge>; 5] = [
        // Common
        vec![
            Reforge {
                stats: Stats {
                    strength: 1,
                    critical_chance: 1,
                    critical_damage: 1,
                },
                name: "Godly",
            },
            Reforge {
                stats: Stats {
                    strength: 1,
                    critical_chance: 0,
                    critical_damage: 3,
                },
                name: "Itchy",
            },
            Reforge {
                stats: Stats {
                    strength: 2,
                    critical_chance: 0,
                    critical_damage: 0,
                },
                name: "Forceful",
            },
        ],
        // Uncommon
        vec![
            Reforge {
                stats: Stats {
                    strength: 2,
                    critical_chance: 2,
                    critical_damage: 2,
                },
                name: "Godly",
            },
            Reforge {
                stats: Stats {
                    strength: 2,
                    critical_chance: 0,
                    critical_damage: 5,
                },
                name: "Itchy",
            },
            Reforge {
                stats: Stats {
                    strength: 4,
                    critical_chance: 0,
                    critical_damage: 0,
                },
                name: "Forceful",
            },
        ],
        // Rare
        vec![
            Reforge {
                stats: Stats {
                    strength: 4,
                    critical_chance: 2,
                    critical_damage: 3,
                },
                name: "Godly",
            },
            Reforge {
                stats: Stats {
                    strength: 2,
                    critical_chance: 0,
                    critical_damage: 8,
                },
                name: "Itchy",
            },
            Reforge {
                stats: Stats {
                    strength: 0,
                    critical_chance: 3,
                    critical_damage: 2,
                },
                name: "Unpleasant",
            },
            Reforge {
                stats: Stats {
                    strength: 7,
                    critical_chance: 0,
                    critical_damage: 0,
                },
                name: "Forceful",
            },
        ],
        // Epic
        vec![
            Reforge {
                stats: Stats {
                    strength: 7,
                    critical_chance: 3,
                    critical_damage: 6,
                },
                name: "Godly",
            },
            Reforge {
                stats: Stats {
                    strength: 3,
                    critical_chance: 0,
                    critical_damage: 12,
                },
                name: "Itchy",
            },
            Reforge {
                stats: Stats {
                    strength: 0,
                    critical_chance: 6,
                    critical_damage: 3,
                },
                name: "Unpleasant",
            },
            Reforge {
                stats: Stats {
                    strength: 10,
                    critical_chance: 0,
                    critical_damage: 0,
                },
                name: "Forceful",
            },
        ],
        // Legendary
        vec![
            Reforge {
                stats: Stats {
                    strength: 10,
                    critical_chance: 5,
                    critical_damage: 8,
                },
                name: "Godly",
            },
            Reforge {
                stats: Stats {
                    strength: 5,
                    critical_chance: 0,
                    critical_damage: 15,
                },
                name: "Itchy",
            },
            Reforge {
                stats: Stats {
                    strength: 0,
                    critical_chance: 8,
                    critical_damage: 5,
                },
                name: "Unpleasant",
            },
            Reforge {
                stats: Stats {
                    strength: 15,
                    critical_chance: 0,
                    critical_damage: 0,
                },
                name: "Forceful",
            },
        ]
    ];
}
