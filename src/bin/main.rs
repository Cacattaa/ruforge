use std::convert::TryInto;

use ruforge::inventory::Inventory;
use ruforge::talisman::Talisman;

use clap::clap_app;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Calculate best talismans reforge")
        (@arg COMMON: -c --common +takes_value "Number of common talisman(s)")
        (@arg UNCOMMON: -u --uncommon +takes_value "Number of uncommon talisman(s)")
        (@arg RARE: -r --rare +takes_value "Number of rare talisman(s)")
        (@arg EPIC: -e --epic +takes_value "Number of epic talisman(s)")
        (@arg LEGENDARY: -l --legendary +takes_value "Number of legendary talisman(s)")
        (@arg BASE_CRIT_CHANCE: -b --("base-crit-chance") +takes_value "Base crit chance(s)")
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
    let base_crit_chance = matches
        .value_of("BASE_CRIT_CHANCE")
        .map(|s| s.parse::<u16>().unwrap())
        .unwrap_or(0);

    let inventory = Inventory::new(base_crit_chance, &talismans);
    let improved = inventory.improved();

    println!("{}", &improved);
}
