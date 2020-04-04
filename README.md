# ruforge

Calculate optimal Talismans Reforges in Hypixel Skyblock.

### Usage

```sh
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -I, --attempts <ATTEMPTS>                    Number of attempts per iteration
    -C, --base-crit-chance <BASE_CRIT_CHANCE>    Base crit chances
    -D, --base-crit-damage <BASE_CRIT_DAMAGE>    Base crit damage
    -S, --base-strength <BASE_STRENGTH>          Base strength
    -c, --common <COMMON>                        Number of common talisman(s)
    -e, --epic <EPIC>                            Number of epic talisman(s)
    -i, --iterations <ITERATIONS>                Number of iterations
    -l, --legendary <LEGENDARY>                  Number of legendary talisman(s)
    -r, --rare <RARE>                            Number of rare talisman(s)
    -u, --uncommon <UNCOMMON>                    Number of uncommon talisman(s)
    -W, --weapon-damage <WEAPON_DAMAGE>          Weapon damage
```

### Example

```sh
$ cargo run -c 10 -u 11 -r 7 -e 1 -l 1 -W 225 -S 216 -C 63 -D 190
Base Stats: Strength: 216 - Critical Chance: 63 - Critical Damage: 190
4x Common - Itchy
5x Common - Godly
1x Common - Forceful
7x Uncommon - Godly
4x Uncommon - Itchy
2x Rare - Itchy
5x Rare - Godly
1x Epic - Godly
1x Legendary - Godly

Final Stats: Strength: 290 - Critical Chance: 100 - Critical Damage: 286
Final Damage: 4335
```
