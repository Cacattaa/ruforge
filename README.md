# ruforge

Calculate optimal Talismans Reforges in Hypixel Skyblock.

### Usage

```sh
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --base-crit-chance <BASE_CRIT_CHANCE>    Base crit chance(s)
    -c, --common <COMMON>                        Number of common talisman(s)
    -e, --epic <EPIC>                            Number of epic talisman(s)
    -l, --legendary <LEGENDARY>                  Number of legendary talisman(s)
    -r, --rare <RARE>                            Number of rare talisman(s)
    -u, --uncommon <UNCOMMON>                    Number of uncommon talisman(s)
```

### Example

```sh
$ cargo run --bin main -- -c 11 -u 10 -r 6 -e 1 -l 0 -b 62
Base Critical Damage: 62
9x Common - Critical Damage - Itchy
2x Common - Critical Chance - Godly
12x Uncommon - Critical Chance - Godly
4x Rare - Critical Damage - Itchy
2x Rare - Critical Chance - Unpleasant
1x Epic - Critical Chance - Unpleasant

Final Stats: Critical Chance: 100 - Critical Damage: 92
```
