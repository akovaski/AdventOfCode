use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use super::d15p1::{self, CharInfo, Point, Tile, Unit};

pub fn main() -> io::Result<()> {
    let f = File::open("./input/day15.txt")?;
    let reader = BufReader::new(f);

    let mut map: BTreeMap<Point, Tile> = BTreeMap::new();

    let mut chars: BTreeMap<Point, CharInfo> = BTreeMap::new();

    let init_health = 200;

    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            let p = Point { x, y };
            let tile = match c {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                'E' => {
                    chars.insert(
                        p,
                        CharInfo {
                            unit: Unit::Elf,
                            health: init_health,
                            moved: false,
                        },
                    );
                    Tile::Empty
                }
                'G' => {
                    chars.insert(
                        p,
                        CharInfo {
                            unit: Unit::Goblin,
                            health: init_health,
                            moved: false,
                        },
                    );
                    Tile::Empty
                }
                _ => panic!("unrecognized map char"),
            };

            map.insert(p, tile);
        }
    }

    for elf_attack in 4.. {
        let mut chars = chars.clone();
        let mut outcome = None;
        let elfs_initial_len = chars.values().filter(|&&c| c.unit == Unit::Elf).count();
        for round in 0.. {
            let complete_round = d15p1::simulate_one_round(&map, &mut chars, elf_attack);

            let elfs_remaining_count = chars.values().filter(|&&c| c.unit == Unit::Elf).count();
            if elfs_remaining_count < elfs_initial_len {
                break;
            }

            let goblins_remaining = chars.values().any(|&c| c.unit == Unit::Goblin);
            if !goblins_remaining {
                let last_full_round = match complete_round {
                    true => round + 1,
                    false => round,
                };
                let remaining_hp: i32 = chars.values().map(|c| c.health).sum();
                outcome = Some(last_full_round * remaining_hp);
                break;
            }
        }
        if let Some(outcome) = outcome {
            println!("Outcome: {}", outcome);
            break;
        }
    }

    Ok(())
}
