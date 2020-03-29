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

`cargo run --bin main -- -c 11 -u 10 -r 6 -e 1 -l 0 -b 62`
