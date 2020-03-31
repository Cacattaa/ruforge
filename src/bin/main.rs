use std::convert::TryInto;

use ruforge::inventory::Inventory;
use ruforge::talisman::Talisman;

use clap::{clap_app, ArgMatches};
use ruforge::stats::Stats;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Calculate best talismans reforge")
        (@arg COMMON: -c --common +takes_value "Number of common talisman(s)")
        (@arg UNCOMMON: -u --uncommon +takes_value "Number of uncommon talisman(s)")
        (@arg RARE: -r --rare +takes_value "Number of rare talisman(s)")
        (@arg EPIC: -e --epic +takes_value "Number of epic talisman(s)")
        (@arg LEGENDARY: -l --legendary +takes_value "Number of legendary talisman(s)")
        (@arg WEAPON_DAMAGE: -W --("weapon-damage") +takes_value "Weapon damage")
        (@arg BASE_STRENGTH: -S --("base-strength") +takes_value "Base strength")
        (@arg BASE_CRIT_CHANCE: -C --("base-crit-chance") +takes_value "Base crit chances")
        (@arg BASE_CRIT_DAMAGE: -D --("base-crit-damage") +takes_value "Base crit damage")
        (@arg ITERATIONS: -i --("iterations") +takes_value "Number of iterations")
        (@arg ATTEMPTS: -I --("attempts") +takes_value "Number of attempts per iteration")
    )
    .get_matches();

    let mut talismans = Vec::new();
    for (i, t) in vec!["COMMON", "UNCOMMON", "RARE", "EPIC", "LEGENDARY"]
        .iter()
        .enumerate()
    {
        for _ in 0..matches
            .value_of(t)
            .map(|s| s.parse::<usize>().unwrap())
            .unwrap_or(0)
        {
            talismans.push(Talisman::new((i as u8).try_into().unwrap()))
        }
    }

    let weapon_damage = parse_flag(&matches, "WEAPON_DAMAGE", 0) as u16;
    let base_strength = parse_flag(&matches, "BASE_STRENGTH", 0) as u16;
    let base_critical_chance = parse_flag(&matches, "BASE_CRIT_CHANCE", 0) as u16;
    let base_critical_damage = parse_flag(&matches, "BASE_CRIT_DAMAGE", 0) as u16;

    let iterations = parse_flag(&matches, "ITERATIONS", 100_000);
    let attempts = parse_flag(&matches, "ATTEMPTS", 100);

    let inventory = Inventory::new(
        weapon_damage,
        Stats {
            strength: base_strength,
            critical_chance: base_critical_chance,
            critical_damage: base_critical_damage
        }, &talismans);
    let improved = inventory.improved(iterations, attempts);

    println!("{}", &improved);
}

fn parse_flag(matches: &ArgMatches, flag_name: &str, default: u64) -> u64 {
    matches
        .value_of(flag_name)
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap_or(default)
}
