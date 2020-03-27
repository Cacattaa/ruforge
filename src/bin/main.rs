use ruforge::*;

use clap::clap_app;
use rayon::prelude::*;

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
    ).get_matches();

    let mut talismans = Vec::new();
    for (i, t) in vec!["COMMON", "UNCOMMON", "RARE", "EPIC", "LEGENDARY"].iter().enumerate() {
        for _ in 0..matches.value_of(t).map(|s| s.parse::<u32>().unwrap()).unwrap_or(0) {
            talismans.push(Talisman{
                rarity: (i as u8).into()
            })
        }
    }

    let base_crit_chance = matches.value_of("BASE_CRIT_CHANCE").map(|s| s.parse::<u64>().unwrap()).unwrap_or(0);

    let end = 2_u64.pow(talismans.len() as u32) - 1;
    let chunk = end / (num_cpus::get() as u64);

    (0..(num_cpus::get() as u64)).into_par_iter().for_each(|i| {
        let best_config = solve(&talismans, i * chunk, (i + 1) * chunk, base_crit_chance);

        let score = score(&talismans, best_config);
        let current = evaluate(score, base_crit_chance);
        println!("Score: {}, config: {:028b}", current, best_config);
    });
    //println!("{:#064b}", solve(&talismans, 0, end, base_crit_chance));
}